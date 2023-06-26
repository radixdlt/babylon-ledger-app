// SBOR decoder

use crate::decoder_error::DecoderError;
use crate::type_info::*;
use core::option::Option::{None, Some};
use core::result::Result;
use core::result::Result::{Err, Ok};

#[cfg(target_os = "nanos")]
pub const STACK_DEPTH: u8 = 25; // Use minimal possible stack for Nano S
#[cfg(target_os = "nanosplus")]
pub const STACK_DEPTH: u8 = 25; // Nano S+ and Nano X have more memory
#[cfg(target_os = "nanox")]
pub const STACK_DEPTH: u8 = 25;
#[cfg(not(any(target_os = "nanos", target_os = "nanox", target_os = "nanosplus")))]
pub const STACK_DEPTH: u8 = 25;

pub const SBOR_LEADING_BYTE: u8 = 0x4d; // MANIFEST_SBOR_V1_PAYLOAD_PREFIX

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
struct Flags(u8);

impl Flags {
    const SKIP_START_END: u8 = 0x80;
    const FLIP_FLOP: u8 = 0x40;
    const PHASE_PTR_MASK: u8 = 0x3F;

    const fn new() -> Self {
        Self(0)
    }

    pub fn skip_start_end(&self) -> bool {
        self.0 & Self::SKIP_START_END != 0
    }

    pub fn set_skip_start_end(&mut self, value: bool) {
        self.0 &= !Self::SKIP_START_END;

        if value {
            self.0 |= Self::SKIP_START_END;
        }
    }

    pub fn flip_flop(&self) -> bool {
        if self.0 & Self::FLIP_FLOP == 0 {
            false
        } else {
            true
        }
    }

    pub fn flip(&mut self) {
        self.0 ^= Self::FLIP_FLOP;
    }

    pub fn phase_ptr(&self) -> u8 {
        self.0 & Self::PHASE_PTR_MASK
    }

    pub fn set_phase_ptr(&mut self, phase_ptr: u8) {
        self.0 &= !Self::PHASE_PTR_MASK;
        self.0 |= phase_ptr & Self::PHASE_PTR_MASK;
    }

    pub fn increment_phase_ptr(&mut self) {
        if self.0 & Self::PHASE_PTR_MASK < Self::PHASE_PTR_MASK {
            self.0 += 1;
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct State {
    items_to_read: u32,
    active_type_id: u8,
    key_type_id: u8,     // Map key type ID
    element_type_id: u8, // Map value type ID; Array/Tuple/Enum - element type ID
    flags: Flags,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DecodingOutcome {
    Done(usize),
    NeedMoreData(usize),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SborEvent {
    Start {
        type_id: u8,
        nesting_level: u8,
        fixed_size: u8,
    },
    Len(u32),
    ElementType {
        kind: SubTypeKind,
        type_id: u8,
    },
    Discriminator(u8),
    Data(u8),
    End {
        type_id: u8,
        nesting_level: u8,
    },
    InputByte(u8),
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SubTypeKind {
    Element,
    Key,
    Value,
}

pub trait SborEventHandler {
    fn handle(&mut self, evt: SborEvent);
}

#[repr(C, packed)]
pub struct SborDecoder {
    stack: [State; STACK_DEPTH as usize],
    byte_count: usize,
    len_acc: usize,
    head: u8,
    len_shift: u8,
    expect_leading_byte: bool,
}

impl SborDecoder {
    pub const fn new(expect_leading_byte: bool) -> Self {
        Self {
            stack: [State::new(); STACK_DEPTH as usize],
            byte_count: 0,
            head: 0,
            expect_leading_byte: expect_leading_byte,
            len_acc: 0,
            len_shift: 0,
        }
    }

    pub fn reset(&mut self) {
        self.byte_count = 0;
        self.head = 0;
        self.stack[0] = State::new();
        self.expect_leading_byte = true;
    }

    #[inline]
    fn head(&mut self) -> &mut State {
        &mut self.stack[self.head as usize]
    }

    pub fn push(&mut self) -> Result<(), DecoderError> {
        if self.head == STACK_DEPTH - 1 {
            let byte_count = self.byte_count;
            return Err(DecoderError::StackOverflow(byte_count));
        }
        self.head += 1;
        self.stack[self.head as usize] = State::new();

        Ok(())
    }

    pub fn pop(&mut self) -> Result<(), DecoderError> {
        if self.head == 0 {
            return Err(DecoderError::StackUnderflow(self.byte_count));
        }
        self.head -= 1;

        Ok(())
    }

    pub fn decode(
        &mut self,
        handler: &mut impl SborEventHandler,
        input: &[u8],
    ) -> Result<DecodingOutcome, DecoderError> {
        for byte in input {
            self.decode_byte(handler, *byte, true)?;
        }

        Ok(self.decoding_outcome())
    }

    pub fn decode_byte(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
        count_input: bool,
    ) -> Result<(), DecoderError> {
        if count_input {
            self.byte_count += 1;

            if self.expect_leading_byte && self.byte_count == 1 {
                if byte == SBOR_LEADING_BYTE {
                    return Ok(());
                }
            }
        }

        let result = match self.head().phase() {
            DecoderPhase::ReadingTypeId => self.read_type_id(handler, byte),
            DecoderPhase::ReadingLen => self.read_len(handler, byte),
            DecoderPhase::ReadingElementTypeId => {
                self.read_sub_type_id(handler, SubTypeKind::Element, byte)
            }
            DecoderPhase::ReadingKeyTypeId => {
                self.read_sub_type_id(handler, SubTypeKind::Key, byte)
            }
            DecoderPhase::ReadingValueTypeId => {
                self.read_sub_type_id(handler, SubTypeKind::Value, byte)
            }
            DecoderPhase::ReadingData => self.read_data(handler, byte),
            DecoderPhase::ReadingDiscriminator => self.read_discriminator(handler, byte),
            DecoderPhase::ReadingNFLDiscriminator => self.read_nfl_discriminator(handler, byte),
            DecoderPhase::ReadingAddressDiscriminator => {
                self.read_address_discriminator(handler, byte)
            }
        };

        if count_input {
            handler.handle(SborEvent::InputByte(byte))
        }

        result
    }

    fn read_discriminator(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
    ) -> Result<(), DecoderError> {
        handler.handle(SborEvent::Discriminator(byte));
        self.advance_phase(handler)
    }

    fn read_nfl_discriminator(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
    ) -> Result<(), DecoderError> {
        handler.handle(SborEvent::Discriminator(byte));

        match byte {
            NFL_STRING | NFL_BYTES => {}                         // read len
            NFL_INTEGER => self.read_len(handler, INTEGER_LEN)?, // simulate reading len and skip phase
            NFL_RUID => self.read_len(handler, RUID_LEN)?,       // simulate and skip phase
            _ => return Err(DecoderError::UnknownDiscriminator(self.byte_count, byte)),
        }

        self.advance_phase(handler)
    }

    fn read_address_discriminator(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
    ) -> Result<(), DecoderError> {
        handler.handle(SborEvent::Discriminator(byte));

        match byte {
            ADDRESS_STATIC => self.read_len(handler, ADDRESS_STATIC_LEN),
            ADDRESS_NAMED => self.read_len(handler, ADDRESS_NAMED_LEN),
            _ => Err(DecoderError::UnknownDiscriminator(self.byte_count, byte)),
        }
    }

    fn decoding_outcome(&mut self) -> DecodingOutcome {
        if self.head == 0 && self.head().phase() == DecoderPhase::ReadingTypeId {
            DecodingOutcome::Done(self.byte_count)
        } else {
            DecodingOutcome::NeedMoreData(self.byte_count)
        }
    }

    fn advance_phase(&mut self, handler: &mut impl SborEventHandler) -> Result<(), DecoderError> {
        if self.head().is_last_phase() {
            {
                let level = self.head;
                let id = self.head().active_type_id;

                if !self.head().skip_start_end() {
                    handler.handle(SborEvent::End {
                        type_id: id,
                        nesting_level: level,
                    });
                }
            }

            self.head().active_type_id = TYPE_NONE;
            self.head().reset_phase();

            if self.head > 0 {
                self.pop()?;
            }
        } else {
            self.head().advance_phase();
        }

        Ok(())
    }

    fn read_type_id(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
    ) -> Result<(), DecoderError> {
        let byte_count = self.byte_count;
        self.head().read_type_id(byte, byte_count)?;

        let size = self.size();

        if !self.head().skip_start_end() {
            handler.handle(SborEvent::Start {
                type_id: byte,
                nesting_level: self.head,
                fixed_size: size,
            });
        }

        self.advance_phase(handler)
    }

    fn read_len(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
    ) -> Result<(), DecoderError> {
        if self.read_encoded_len(byte)? {
            handler.handle(SborEvent::Len(self.head().items_to_read));
            self.advance_phase(handler)?;

            // Automatically skip reading data if len is zero
            self.check_end_of_data_read(handler)
        } else {
            Ok(())
        }
    }

    fn read_encoded_len(&mut self, byte: u8) -> Result<bool, DecoderError> {
        let byte_count = self.byte_count;

        self.len_acc |= ((byte & 0x7F) as usize) << self.len_shift;

        if byte < 0x80 {
            self.head().items_to_read = self.len_acc as u32;
            self.len_acc = 0;
            self.len_shift = 0;
            return Ok(true);
        }

        self.len_shift += 7;
        if self.len_shift >= 28 {
            return Err(DecoderError::InvalidLen(byte_count, byte));
        }

        Ok(false)
    }

    fn read_sub_type_id(
        &mut self,
        handler: &mut impl SborEventHandler,
        sub_type: SubTypeKind,
        byte: u8,
    ) -> Result<(), DecoderError> {
        let byte_count = self.byte_count;
        self.head()
            .read_sub_type_id(handler, byte, sub_type, byte_count)?;
        self.advance_phase(handler)
    }

    fn read_data(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
    ) -> Result<(), DecoderError> {
        let byte_count = self.byte_count;

        match self.head().active_type_id {
            // fixed/variable len components with raw bytes payload
            // Unit..String | Custom types
            0x00..=0x0c | 0x80..=0xff => {
                handler.handle(SborEvent::Data(byte));
                self.read_single_data_byte(byte)
            }

            // variable length components with fields payload
            TYPE_TUPLE | TYPE_ENUM => {
                self.head().decrement_items_to_read(byte_count)?; // Increment field count
                self.push()?; // Start new field
                self.decode_byte(handler, byte, false) // Read first byte (field type id)
            }

            // variable length component with flip/flop payload (key/value)
            TYPE_MAP => {
                let type_id = if !self.head().flip_flop() {
                    // Key
                    self.head().key_type_id
                } else {
                    // Value
                    self.head().decrement_items_to_read(byte_count)?; // Increment entry count
                    self.head().element_type_id
                };
                self.head().flip();

                self.push()?; // Start key or value content read

                self.decode_byte(handler, type_id, false)?; // Set element type
                self.decode_byte(handler, byte, false) // Decode first byte of data
            }

            // variable length components with fixed payload type
            TYPE_ARRAY => {
                self.head().decrement_items_to_read(byte_count)?; // Increment element count
                let type_id = self.head().element_type_id; // Prepare element type

                self.push()?; // Start new element

                match type_id {
                    // do not report start/end of each element for byte arrays
                    TYPE_U8 | TYPE_I8 => {
                        self.head().set_skip_start_end(true);
                    }
                    _ => {}
                }

                self.decode_byte(handler, type_id, false)?; // Set element type
                self.decode_byte(handler, byte, false) // Decode first byte of data
            }

            _ => Err(DecoderError::InvalidState(byte_count)),
        }?;

        self.check_end_of_data_read(handler)
    }

    fn check_end_of_data_read(
        &mut self,
        handler: &mut impl SborEventHandler,
    ) -> Result<(), DecoderError> {
        while self.head().all_read() && self.head().is_read_data_phase() {
            self.advance_phase(handler)?
        }

        Ok(())
    }

    fn read_single_data_byte(&mut self, _byte: u8) -> Result<(), DecoderError> {
        let byte_count = self.byte_count;
        self.head().decrement_items_to_read(byte_count)
    }

    fn size(&mut self) -> u8 {
        // Safe to unwrap because we already checked that type_id is valid
        to_type_info(self.head().active_type_id).unwrap().fixed_len
    }
}

impl State {
    const fn new() -> Self {
        Self {
            active_type_id: TYPE_NONE,
            items_to_read: 0,
            element_type_id: TYPE_NONE,
            key_type_id: TYPE_NONE,
            flags: Flags::new(),
        }
    }

    pub fn advance_phase(&mut self) {
        self.flags.increment_phase_ptr();
    }

    pub fn reset_phase(&mut self) {
        self.flags.set_phase_ptr(0);
    }

    #[inline]
    pub fn phase(&self) -> DecoderPhase {
        self.phases()[self.flags.phase_ptr() as usize]
    }

    #[inline]
    pub fn skip_start_end(&mut self) -> bool {
        self.flags.skip_start_end()
    }

    #[inline]
    pub fn set_skip_start_end(&mut self, value: bool) {
        self.flags.set_skip_start_end(value);
    }

    #[inline]
    pub fn flip_flop(&mut self) -> bool {
        self.flags.flip_flop()
    }

    #[inline]
    pub fn flip(&mut self) {
        self.flags.flip();
    }

    #[inline]
    fn phases(&self) -> &[DecoderPhase] {
        &to_type_info(self.active_type_id).unwrap().next_phases
    }

    #[inline]
    fn is_last_phase(&self) -> bool {
        let len = self.phases().len() as u8;
        self.flags.phase_ptr() == len - 1
    }

    fn is_read_data_phase(&self) -> bool {
        self.phase() == DecoderPhase::ReadingData
    }

    fn read_type_id(&mut self, byte: u8, byte_count: usize) -> Result<(), DecoderError> {
        match to_type_info(byte) {
            None => Err(DecoderError::UnknownType(byte_count, byte)),
            Some(type_info) => {
                self.active_type_id = byte;
                self.items_to_read = type_info.fixed_len as u32;

                Ok(())
            }
        }
    }

    fn read_sub_type_id(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
        sub_type: SubTypeKind,
        byte_count: usize,
    ) -> Result<(), DecoderError> {
        match to_type_info(byte) {
            None => Err(DecoderError::UnknownSubType(byte_count, byte)),
            Some(_) => {
                match sub_type {
                    SubTypeKind::Key => self.key_type_id = byte,
                    SubTypeKind::Value => self.element_type_id = byte,
                    SubTypeKind::Element => self.element_type_id = byte,
                }
                handler.handle(SborEvent::ElementType {
                    kind: sub_type,
                    type_id: byte,
                });
                Ok(())
            }
        }
    }

    fn all_read(&mut self) -> bool {
        self.items_to_read == 0
    }

    fn decrement_items_to_read(&mut self, byte_count: usize) -> Result<(), DecoderError> {
        if self.items_to_read == 0 {
            Err(DecoderError::InvalidState(byte_count))
        } else {
            self.items_to_read -= 1;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use core::fmt::Display;
    use core::fmt::Formatter;
    use core::fmt::Result;

    use super::*;
    use crate::tx_intent_test_data::tests::*;

    #[cfg(test)]
    impl Display for SborEvent {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                SborEvent::Start {
                    type_id,
                    nesting_level,
                    fixed_size,
                } => {
                    write!(
                        f,
                        "{}SborEvent::Start{{ type_id: {}, nesting_level: {}, fixed_size: {}}},",
                        " ".repeat(*nesting_level as usize),
                        *type_id,
                        *nesting_level,
                        *fixed_size
                    )
                }
                SborEvent::ElementType { kind, type_id } => {
                    write!(
                        f,
                        "SborEvent::ElementType{{ kind: {:?}, type_id: {:#02x}}},",
                        kind, type_id
                    )
                }
                SborEvent::Len(len) => {
                    write!(f, "SborEvent::Len({}),", *len)
                }
                SborEvent::Discriminator(byte) => {
                    write!(f, "SborEvent::Name({:#02x}),", *byte)
                }
                SborEvent::Data(byte) => {
                    write!(f, "SborEvent::Data({:#02x}),", *byte)
                }
                SborEvent::End {
                    type_id,
                    nesting_level,
                } => {
                    write!(
                        f,
                        "{}SborEvent::End{{type_id: {}, nesting_level: {}}},",
                        " ".repeat(*nesting_level as usize),
                        *type_id,
                        *nesting_level
                    )
                }
                SborEvent::InputByte(byte) => {
                    write!(f, "SborEvent::InputByte({:#02x}),", *byte)
                }
            }
        }
    }

    #[derive(Debug)]
    struct EventCollector {
        collected: [SborEvent; Self::LENGTH],
        count: usize,
    }

    impl SborEventHandler for EventCollector {
        fn handle(&mut self, evt: SborEvent) {
            assert_ne!(
                self.count,
                self.collected.len(),
                "evt = {}, count = {}",
                evt,
                self.count
            );

            if let SborEvent::InputByte(_) = evt {
                return;
            }

            self.collected[self.count] = evt;
            self.count += 1;
        }
    }

    impl EventCollector {
        pub const LENGTH: usize = 3500;

        pub fn new() -> Self {
            Self {
                collected: [SborEvent::Len(0); Self::LENGTH],
                count: 0,
            }
        }

        pub fn compare(&self, vb: &[SborEvent]) -> bool {
            assert_eq!(
                self.count,
                vb.len(),
                "Different length: actual {}, expected {}",
                self.count,
                vb.len()
            );
            let mut cnt = 0;

            self.collected[..self.count].iter().zip(vb).all(|(a, b)| {
                assert_eq!(*a, *b, "Elements are not equal at index {}", cnt);
                cnt += 1;
                true
            })
        }

        pub fn print(&self) {
            // for i in 0..self.count {
            //     println!("{}", self.collected[i]);
            // }
            // println!("Total {} events", self.count);
        }
    }

    fn check_decoding(input: &[u8], event_list: &[SborEvent]) {
        let mut decoder = SborDecoder::new(false);

        let mut handler = EventCollector::new();

        match decoder.decode(&mut handler, &input) {
            Ok(outcome) => {
                assert_eq!(outcome, DecodingOutcome::Done(input.len()))
            }
            Err(err) => {
                assert!(false, "Should not return an error {:?}", err)
            }
        }

        handler.compare(&event_list);
    }

    #[test]
    pub fn test_fixed_length_types_decoding() {
        let input: [u8; 76] = [
            33, 0, // unit
            1, 1, // bool
            2, 1, // i8
            3, 1, 0, // i16
            4, 1, 0, 0, 0, // i32
            5, 1, 0, 0, 0, 0, 0, 0, 0, // i64
            6, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // i128
            7, 1, // u8
            8, 1, 0, // u16
            9, 1, 0, 0, 0, // u32
            10, 1, 0, 0, 0, 0, 0, 0, 0, // u64
            11, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // u128
        ];

        check_decoding(
            &input,
            &[
                SborEvent::Start {
                    type_id: 33,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::Len(0x0),
                SborEvent::End {
                    type_id: 33,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 1,
                    nesting_level: 0,
                    fixed_size: 1,
                },
                SborEvent::Data(0x1),
                SborEvent::End {
                    type_id: 1,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 2,
                    nesting_level: 0,
                    fixed_size: 1,
                },
                SborEvent::Data(0x1),
                SborEvent::End {
                    type_id: 2,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 3,
                    nesting_level: 0,
                    fixed_size: 2,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 3,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 4,
                    nesting_level: 0,
                    fixed_size: 4,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 4,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 5,
                    nesting_level: 0,
                    fixed_size: 8,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 5,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 6,
                    nesting_level: 0,
                    fixed_size: 16,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 6,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 7,
                    nesting_level: 0,
                    fixed_size: 1,
                },
                SborEvent::Data(0x1),
                SborEvent::End {
                    type_id: 7,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 8,
                    nesting_level: 0,
                    fixed_size: 2,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 8,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 0,
                    fixed_size: 4,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 10,
                    nesting_level: 0,
                    fixed_size: 8,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 10,
                    nesting_level: 0,
                },
                SborEvent::Start {
                    type_id: 11,
                    nesting_level: 0,
                    fixed_size: 16,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 11,
                    nesting_level: 0,
                },
            ],
        )
    }

    #[test]
    pub fn test_string_decoding() {
        let input: [u8; 7] = [
            12, 5, 104, 101, 108, 108, 111, // string
        ];

        check_decoding(
            &input,
            &[
                SborEvent::Start {
                    type_id: 12,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::Len(5),
                SborEvent::Data(0x68),
                SborEvent::Data(0x65),
                SborEvent::Data(0x6c),
                SborEvent::Data(0x6c),
                SborEvent::Data(0x6f),
                SborEvent::End {
                    type_id: 12,
                    nesting_level: 0,
                },
            ],
        )
    }

    #[test]
    pub fn test_array_decoding() {
        let input: [u8; 35] = [
            32, 9, 3, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, // array
            32, 9, 3, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, // vec
            32, 7, 2, 1, 2, // set
        ];

        check_decoding(
            &input,
            &[
                // Array
                SborEvent::Start {
                    type_id: 32,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::ElementType {
                    kind: SubTypeKind::Element,
                    type_id: 9,
                },
                SborEvent::Len(3),
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x01),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x02),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x03),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::End {
                    type_id: 32,
                    nesting_level: 0,
                },
                // Vec
                SborEvent::Start {
                    type_id: 32,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::ElementType {
                    kind: SubTypeKind::Element,
                    type_id: 9,
                },
                SborEvent::Len(3),
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x01),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x02),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x03),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::Data(0x00),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::End {
                    type_id: 32,
                    nesting_level: 0,
                },
                // Set
                SborEvent::Start {
                    type_id: 32,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::ElementType {
                    kind: SubTypeKind::Element,
                    type_id: 7,
                },
                SborEvent::Len(2),
                SborEvent::Data(0x01),
                SborEvent::Data(0x02),
                SborEvent::End {
                    type_id: 32,
                    nesting_level: 0,
                },
            ],
        )
    }

    #[test]
    pub fn test_map_decoding() {
        let input: [u8; 8] = [
            35, 7, 7, 2, 1, 2, 3, 4, // map
        ];

        check_decoding(
            &input,
            &[
                SborEvent::Start {
                    type_id: 35,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::ElementType {
                    kind: SubTypeKind::Key,
                    type_id: 7,
                },
                SborEvent::ElementType {
                    kind: SubTypeKind::Value,
                    type_id: 7,
                },
                SborEvent::Len(2),
                // Key 0
                SborEvent::Start {
                    type_id: 7,
                    nesting_level: 1,
                    fixed_size: 1,
                },
                SborEvent::Data(1),
                SborEvent::End {
                    type_id: 7,
                    nesting_level: 1,
                },
                // Value 0
                SborEvent::Start {
                    type_id: 7,
                    nesting_level: 1,
                    fixed_size: 1,
                },
                SborEvent::Data(2),
                SborEvent::End {
                    type_id: 7,
                    nesting_level: 1,
                },
                // Key 1
                SborEvent::Start {
                    type_id: 7,
                    nesting_level: 1,
                    fixed_size: 1,
                },
                SborEvent::Data(3),
                SborEvent::End {
                    type_id: 7,
                    nesting_level: 1,
                },
                // Value 1
                SborEvent::Start {
                    type_id: 7,
                    nesting_level: 1,
                    fixed_size: 1,
                },
                SborEvent::Data(4),
                SborEvent::End {
                    type_id: 7,
                    nesting_level: 1,
                },
                SborEvent::End {
                    type_id: 35,
                    nesting_level: 0,
                },
            ],
        )
    }

    #[test]
    pub fn test_tuple_decoding() {
        let input: [u8; 12] = [
            33, 2, 9, 1, 0, 0, 0, 9, 2, 0, 0, 0, // tuple
        ];

        check_decoding(
            &input,
            &[
                SborEvent::Start {
                    type_id: 33,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::Len(2),
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x2),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::End {
                    type_id: 33,
                    nesting_level: 0,
                },
            ],
        )
    }

    #[test]
    pub fn test_enum_decoding() {
        let input: [u8; 29] = [
            34, 0, 0, // None
            34, 1, 1, 9, 1, 0, 0, 0, // Some<T>
            34, 0, 1, 9, 1, 0, 0, 0, // Ok<T>
            34, 1, 1, 12, 5, 104, 101, 108, 108, 111, // Err<T>
        ];

        check_decoding(
            &input,
            &[
                // None
                SborEvent::Start {
                    type_id: 34,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::Discriminator(0),
                SborEvent::Len(0),
                SborEvent::End {
                    type_id: 34,
                    nesting_level: 0,
                },
                // Some<T>
                SborEvent::Start {
                    type_id: 34,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::Discriminator(1),
                SborEvent::Len(1),
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::End {
                    type_id: 34,
                    nesting_level: 0,
                },
                // Ok<T>
                SborEvent::Start {
                    type_id: 34,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::Discriminator(0),
                SborEvent::Len(1),
                SborEvent::Start {
                    type_id: 9,
                    nesting_level: 1,
                    fixed_size: 4,
                },
                SborEvent::Data(0x1),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::Data(0x0),
                SborEvent::End {
                    type_id: 9,
                    nesting_level: 1,
                },
                SborEvent::End {
                    type_id: 34,
                    nesting_level: 0,
                },
                // Err<T>
                SborEvent::Start {
                    type_id: 34,
                    nesting_level: 0,
                    fixed_size: 0,
                },
                SborEvent::Discriminator(1),
                SborEvent::Len(1),
                SborEvent::Start {
                    type_id: 12,
                    nesting_level: 1,
                    fixed_size: 0,
                },
                SborEvent::Len(5),
                SborEvent::Data(104),
                SborEvent::Data(101),
                SborEvent::Data(108),
                SborEvent::Data(108),
                SborEvent::Data(111),
                SborEvent::End {
                    type_id: 12,
                    nesting_level: 1,
                },
                SborEvent::End {
                    type_id: 34,
                    nesting_level: 0,
                },
            ],
        )
    }

    const CHUNK_SIZE: usize = 113;

    fn check_partial_decoding(input: &[u8]) {
        let mut decoder = SborDecoder::new(true);
        let mut handler = EventCollector::new();

        let mut start = 0;
        let mut end = core::cmp::min(CHUNK_SIZE, input.len());

        while start < input.len() {
            match decoder.decode(&mut handler, &input[start..end]) {
                Ok(outcome) => {
                    if end - start == CHUNK_SIZE && end < input.len() {
                        assert_eq!(outcome, DecodingOutcome::NeedMoreData(end));
                    } else {
                        assert_eq!(outcome, DecodingOutcome::Done(input.len()))
                    }
                }
                Err(err) => {
                    assert!(false, "Should not return an error {:?}", err)
                }
            }

            start += CHUNK_SIZE;
            end += CHUNK_SIZE;

            if end > input.len() {
                end = input.len();
            }
        }
        handler.print();
    }

    // ---------------------------------------------------- Full TX Intent
    #[test]
    pub fn test_address_allocation() {
        check_partial_decoding(&TX_ADDRESS_ALLOCATION);
    }
    #[test]
    pub fn test_call_function() {
        check_partial_decoding(&TX_CALL_FUNCTION);
    }
    #[test]
    pub fn test_call_method() {
        check_partial_decoding(&TX_CALL_METHOD);
    }
    #[test]
    pub fn test_create_access_controller() {
        check_partial_decoding(&TX_CREATE_ACCESS_CONTROLLER);
    }
    #[test]
    pub fn test_create_account() {
        check_partial_decoding(&TX_CREATE_ACCOUNT);
    }
    #[test]
    pub fn test_create_fungible_resource_with_initial_supply() {
        check_partial_decoding(&TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY);
    }
    #[test]
    pub fn test_create_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(&TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY);
    }
    #[test]
    pub fn test_create_identity() {
        check_partial_decoding(&TX_CREATE_IDENTITY);
    }
    #[test]
    pub fn test_create_non_fungible_resource_with_initial_supply() {
        check_partial_decoding(&TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY);
    }
    #[test]
    pub fn test_create_non_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(&TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY);
    }
    #[test]
    pub fn test_create_validator() {
        check_partial_decoding(&TX_CREATE_VALIDATOR);
    }
    #[test]
    pub fn test_metadata() {
        check_partial_decoding(&TX_METADATA);
    }
    #[test]
    pub fn test_mint_fungible() {
        check_partial_decoding(&TX_MINT_FUNGIBLE);
    }
    #[test]
    pub fn test_mint_non_fungible() {
        check_partial_decoding(&TX_MINT_NON_FUNGIBLE);
    }
    #[test]
    pub fn test_publish_package() {
        check_partial_decoding(&TX_PUBLISH_PACKAGE);
    }
    #[test]
    pub fn test_resource_auth_zone() {
        check_partial_decoding(&TX_RESOURCE_AUTH_ZONE);
    }
    #[test]
    pub fn test_resource_recall() {
        check_partial_decoding(&TX_RESOURCE_RECALL);
    }
    #[test]
    pub fn test_resource_worktop() {
        check_partial_decoding(&TX_RESOURCE_WORKTOP);
    }
    #[test]
    pub fn test_royalty() {
        check_partial_decoding(&TX_ROYALTY);
    }
    #[test]
    pub fn test_simple_transfer() {
        check_partial_decoding(&TX_SIMPLE_TRANSFER);
    }
    #[test]
    pub fn test_simple_transfer_nft() {
        check_partial_decoding(&TX_SIMPLE_TRANSFER_NFT);
    }
    #[test]
    pub fn test_simple_transfer_nft_by_id() {
        check_partial_decoding(&TX_SIMPLE_TRANSFER_NFT_BY_ID);
    }
    #[test]
    pub fn test_simple_transfer_with_multiple_locked_fees() {
        check_partial_decoding(&TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES);
    }
    #[test]
    pub fn test_access_rule() {
        check_partial_decoding(&TX_ACCESS_RULE);
    }
    #[test]
    pub fn test_values() {
        check_partial_decoding(&TX_VALUES);
    }
    #[test]
    pub fn test_vault_freeze() {
        check_partial_decoding(&TX_VAULT_FREEZE);
    }
    #[test]
    pub fn test_hc_intent() {
        check_partial_decoding(&TX_HC_INTENT);
    }
}
