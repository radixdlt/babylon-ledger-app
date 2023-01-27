// SBOR decoder

use crate::decoder_error::DecoderError;
use crate::sbor_notifications::SborEvent;
use crate::type_info::*;
use core::default::Default;
use core::option::Option::{None, Some};
use core::result::Result;
use core::result::Result::{Err, Ok};

pub const STACK_DEPTH: u8 = 48;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    items_to_read: u32,
    items_read: u32,
    len_acc: usize,
    phase_ptr: u8,
    element_type_id: u8,
    key_type_id: u8,
    value_type_id: u8,
    len_shift: u8,
    skip_start_end: bool,
    active_type: TypeInfo,
    phase: DecoderPhase,
    flip_flop: FlipFlopState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DecodingOutcome {
    Done(usize),
    NeedMoreData(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum SubTypeKind {
    Element,
    Key,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum FlipFlopState {
    Key,
    Value,
}

impl Default for State {
    fn default() -> Self {
        Self {
            phase: DecoderPhase::ReadingTypeId,
            phase_ptr: 0,
            active_type: NONE_TYPE_INFO,
            items_to_read: 0,
            items_read: 0,
            element_type_id: TYPE_NONE,
            key_type_id: TYPE_NONE,
            value_type_id: TYPE_NONE,
            len_shift: 0,
            len_acc: 0,
            skip_start_end: false,
            flip_flop: FlipFlopState::Key,
        }
    }
}

pub trait SborEventHandler {
    fn handle(&mut self, evt: SborEvent);
}

pub struct SborDecoder {
    stack: [State; STACK_DEPTH as usize],
    byte_count: usize,
    head: u8,
}

impl SborDecoder {
    pub fn new() -> Self {
        Self {
            stack: [State::default(); STACK_DEPTH as usize],
            byte_count: 0,
            head: 0,
        }
    }

    #[inline]
    fn head(&mut self) -> &mut State {
        &mut self.stack[self.head as usize]
    }

    #[inline]
    fn phase(&mut self) -> DecoderPhase {
        self.head().phase
    }

    pub fn push(&mut self) -> Result<(), DecoderError> {
        if self.head == STACK_DEPTH - 1 {
            let byte_count = self.byte_count;
            return Err(DecoderError::StackOverflow(byte_count));
        }
        self.head += 1;
        self.stack[self.head as usize] = State::default();

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
        }

        match self.phase() {
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
            DecoderPhase::ReadingDiscriminator => {
                handler.handle(SborEvent::Discriminator(byte));
                self.advance_phase(handler)
            }
        }
    }

    fn decoding_outcome(&mut self) -> DecodingOutcome {
        if self.head == 0 && self.phase() == DecoderPhase::ReadingTypeId {
            DecodingOutcome::Done(self.byte_count)
        } else {
            DecodingOutcome::NeedMoreData(self.byte_count)
        }
    }

    fn advance_phase(&mut self, handler: &mut impl SborEventHandler) -> Result<(), DecoderError> {
        if self.head().is_last_phase() {
            {
                let level = self.head;
                let id = self.head().active_type.type_id;

                if !self.head().skip_start_end {
                    handler.handle(SborEvent::End {
                        type_id: id,
                        nesting_level: level,
                    });
                }
            }

            self.head().phase = DecoderPhase::ReadingTypeId;
            self.head().phase_ptr = 0;

            if self.head > 0 {
                self.pop()?;
            }
        } else {
            let mut head = self.head();
            head.phase_ptr += 1;
            head.phase = head.active_type.next_phases[head.phase_ptr as usize];
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

        if !self.head().skip_start_end {
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
        let byte_count = self.byte_count;
        if self.head().read_len(byte, byte_count)? {
            handler.handle(SborEvent::Len(self.head().items_to_read));
            self.advance_phase(handler)?;

            // Automatically skip reading data if len is zero
            self.check_end_of_data_read(handler)
        } else {
            Ok(())
        }
    }

    fn read_sub_type_id(
        &mut self,
        handler: &mut impl SborEventHandler,
        sub_type: SubTypeKind,
        byte: u8,
    ) -> Result<(), DecoderError> {
        let byte_count = self.byte_count;
        self.head().read_sub_type_id(byte, sub_type, byte_count)?;
        self.advance_phase(handler)
    }

    fn read_data(
        &mut self,
        handler: &mut impl SborEventHandler,
        byte: u8,
    ) -> Result<(), DecoderError> {
        let byte_count = self.byte_count;

        match self.head().active_type.type_id {
            // fixed/variable len components with raw bytes payload
            // Unit..String | Custom types
            0x00..=0x0c | 0x80..=0xff => {
                handler.handle(SborEvent::Data(byte));
                self.read_single_data_byte(byte)
            }

            // variable length components with fields payload
            TYPE_TUPLE | TYPE_ENUM => {
                self.head().increment_items_read(byte_count)?; // Increment field count
                self.push()?; // Start new field
                self.decode_byte(handler, byte, false) // Read first byte (field type id)
            }

            // variable length component with flip/flop payload (key/value)
            TYPE_MAP => {
                let type_id = match self.head().flip_flop {
                    FlipFlopState::Key => {
                        self.head().flip_flop = FlipFlopState::Value;
                        self.head().key_type_id
                    }
                    FlipFlopState::Value => {
                        self.head().flip_flop = FlipFlopState::Key;
                        self.head().increment_items_read(byte_count)?; // Increment entry count
                        self.head().value_type_id
                    }
                };

                self.push()?; // Start key or value content read

                self.decode_byte(handler, type_id, false)?; // Set element type
                self.decode_byte(handler, byte, false) // Decode first byte of data
            }

            // variable length components with fixed payload type
            TYPE_ARRAY => {
                self.head().increment_items_read(byte_count)?; // Increment element count
                let type_id = self.head().element_type_id; // Prepare element type

                self.push()?; // Start new element

                match type_id {
                    // do not report start/end of each element for byte arrays
                    // instead they are reported like strings or enum name
                    TYPE_U8 | TYPE_I8 => {
                        self.head().skip_start_end = true;
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
        self.head().increment_items_read(byte_count)
    }

    fn size(&mut self) -> u8 {
        self.head().active_type.fixed_len
    }
}

impl State {
    #[inline]
    fn is_last_phase(&mut self) -> bool {
        self.phase_ptr == (self.active_type.next_phases.len() - 1) as u8
    }

    fn is_read_data_phase(&mut self) -> bool {
        self.phase == DecoderPhase::ReadingData
    }

    fn read_type_id(&mut self, byte: u8, byte_count: usize) -> Result<(), DecoderError> {
        match to_type_info(byte) {
            None => Err(DecoderError::InvalidInput(byte_count, byte)),
            Some(type_info) => {
                self.active_type = type_info;
                self.items_to_read = self.active_type.fixed_len as u32;
                self.items_read = 0;

                Ok(())
            }
        }
    }

    fn read_len(&mut self, byte: u8, byte_count: usize) -> Result<bool, DecoderError> {
        self.len_acc |= ((byte & 0x7F) as usize) << self.len_shift;

        if byte < 0x80 {
            self.items_read = 0;
            self.items_to_read = self.len_acc as u32;
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
        byte: u8,
        sub_type: SubTypeKind,
        byte_count: usize,
    ) -> Result<(), DecoderError> {
        match to_type_info(byte) {
            None => Err(DecoderError::InvalidInput(byte_count, byte)),
            Some(_) => {
                match sub_type {
                    SubTypeKind::Element => self.element_type_id = byte,
                    SubTypeKind::Key => self.key_type_id = byte,
                    SubTypeKind::Value => self.value_type_id = byte,
                }
                Ok(())
            }
        }
    }

    fn all_read(&mut self) -> bool {
        self.items_read == self.items_to_read
    }

    fn increment_items_read(&mut self, byte_count: usize) -> Result<(), DecoderError> {
        self.items_read += 1;

        if self.items_to_read < self.items_read {
            Err(DecoderError::InvalidState(byte_count))
        } else {
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

    use crate::sbor_decoder::{DecodingOutcome, SborDecoder, SborEventHandler};
    use crate::sbor_notifications::SborEvent;

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
            }
        }
    }

    #[derive(Debug)]
    struct EventCollector {
        collected: [SborEvent; 1600],
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
            self.collected[self.count] = evt;
            self.count += 1;
        }
    }

    impl EventCollector {
        pub fn new() -> Self {
            Self {
                collected: [SborEvent::Len(0); 1600],
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

            self.collected[0..self.count].iter().zip(vb).all(|(a, b)| {
                assert_eq!(*a, *b, "Elements are not equal at index {}", cnt);
                cnt += 1;
                true
            })
        }
    }

    fn check_decoding(input: &[u8], event_list: &[SborEvent]) {
        let mut decoder = SborDecoder::new();

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

    // #[test]
    // pub fn test_partial_decoding() {
    //     let input: [u8; 1408] = [
    //         0x10, 0x02, 0x10, 0x09, 0x07, 0x01, 0x07, 0xf2, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x0a, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x05,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x0e, 0x45, 0x63, 0x64, 0x73, 0x61,
    //         0x53, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x6b, 0x31, 0x01, 0xb1, 0x02, 0x79, 0xbe,
    //         0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87, 0x0b, 0x07,
    //         0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16, 0xf8,
    //         0x17, 0x98, 0x01, 0x00, 0x09, 0x40, 0x42, 0x0f, 0x00, 0x09, 0x05, 0x00, 0x00, 0x00,
    //         0x10, 0x02, 0x20, 0x11, 0x12, 0x0a, 0x43, 0x61, 0x6c, 0x6c, 0x4d, 0x65, 0x74, 0x68,
    //         0x6f, 0x64, 0x02, 0x10, 0x02, 0x11, 0x06, 0x47, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x01,
    //         0x81, 0x02, 0x18, 0x43, 0x2e, 0x83, 0x1c, 0xae, 0xec, 0x2d, 0xde, 0xb0, 0xd1, 0xb4,
    //         0x58, 0x77, 0x61, 0xd4, 0x2e, 0x77, 0x78, 0x05, 0x81, 0xea, 0x2e, 0xd6, 0x1b, 0x91,
    //         0x0c, 0x08, 0x66, 0x72, 0x65, 0x65, 0x5f, 0x78, 0x72, 0x64, 0x20, 0x07, 0x02, 0x10,
    //         0x00, 0x0a, 0x43, 0x61, 0x6c, 0x6c, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02, 0x10,
    //         0x02, 0x11, 0x09, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x01, 0x20,
    //         0x07, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0c, 0x08, 0x66, 0x72,
    //         0x65, 0x65, 0x5f, 0x78, 0x72, 0x64, 0x20, 0x07, 0x02, 0x10, 0x00, 0x0f, 0x54, 0x61,
    //         0x6b, 0x65, 0x46, 0x72, 0x6f, 0x6d, 0x57, 0x6f, 0x72, 0x6b, 0x74, 0x6f, 0x70, 0x01,
    //         0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
    //         0x17, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x46, 0x72,
    //         0x6f, 0x6d, 0x41, 0x75, 0x74, 0x68, 0x5a, 0x6f, 0x6e, 0x65, 0x01, 0x82, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x10, 0x43, 0x61,
    //         0x6c, 0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64,
    //         0x02, 0x10, 0x02, 0x11, 0x06, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x01, 0x09, 0x00,
    //         0x02, 0x00, 0x00, 0x0c, 0x14, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75,
    //         0x72, 0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x07, 0x02,
    //         0x10, 0x00, 0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x4d,
    //         0x65, 0x74, 0x68, 0x6f, 0x64, 0x02, 0x10, 0x02, 0x11, 0x06, 0x42, 0x75, 0x63, 0x6b,
    //         0x65, 0x74, 0x01, 0x09, 0x01, 0x00, 0x00, 0x00, 0x0c, 0x14, 0x67, 0x65, 0x74, 0x5f,
    //         0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65,
    //         0x73, 0x73, 0x20, 0x07, 0x02, 0x10, 0x00, 0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e, 0x61,
    //         0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02, 0x10, 0x02, 0x11,
    //         0x06, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x01, 0x09, 0x01, 0x02, 0x00, 0x00, 0x0c,
    //         0x14, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f,
    //         0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x07, 0x02, 0x10, 0x00, 0x10, 0x43,
    //         0x61, 0x6c, 0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f,
    //         0x64, 0x02, 0x10, 0x02, 0x11, 0x06, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x01, 0x09,
    //         0x01, 0x00, 0x00, 0x00, 0x0c, 0x14, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x6f,
    //         0x75, 0x72, 0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x07,
    //         0x02, 0x10, 0x00, 0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65,
    //         0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02, 0x10, 0x02, 0x11, 0x0d, 0x41, 0x75, 0x74,
    //         0x68, 0x5a, 0x6f, 0x6e, 0x65, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x01, 0x09, 0x01, 0x00,
    //         0x00, 0x00, 0x0c, 0x05, 0x64, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x07, 0x02, 0x10, 0x00,
    //         0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74,
    //         0x68, 0x6f, 0x64, 0x02, 0x10, 0x02, 0x11, 0x07, 0x57, 0x6f, 0x72, 0x6b, 0x74, 0x6f,
    //         0x70, 0x00, 0x0c, 0x05, 0x64, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x07, 0x02, 0x10, 0x00,
    //         0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74,
    //         0x68, 0x6f, 0x64, 0x02, 0x10, 0x02, 0x11, 0x0d, 0x4b, 0x65, 0x79, 0x56, 0x61, 0x6c,
    //         0x75, 0x65, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x01, 0x20, 0x07, 0x24, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x05, 0x0c, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x20,
    //         0x07, 0x02, 0x10, 0x00, 0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76,
    //         0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02, 0x10, 0x02, 0x11, 0x10, 0x4e, 0x6f,
    //         0x6e, 0x46, 0x75, 0x6e, 0x67, 0x69, 0x62, 0x6c, 0x65, 0x53, 0x74, 0x6f, 0x72, 0x65,
    //         0x01, 0x20, 0x07, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0c, 0x06,
    //         0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x20, 0x07, 0x02, 0x10, 0x00, 0x10, 0x43, 0x61,
    //         0x6c, 0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64,
    //         0x02, 0x10, 0x02, 0x11, 0x09, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    //         0x01, 0x20, 0x07, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0c, 0x10,
    //         0x61, 0x64, 0x64, 0x5f, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x63, 0x68, 0x65,
    //         0x63, 0x6b, 0x20, 0x07, 0x02, 0x10, 0x00, 0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e, 0x61,
    //         0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02, 0x10, 0x02, 0x11,
    //         0x0c, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x01,
    //         0x20, 0x07, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0c, 0x14, 0x67,
    //         0x65, 0x74, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    //         0x5f, 0x68, 0x61, 0x73, 0x68, 0x20, 0x07, 0x02, 0x10, 0x00, 0x10, 0x43, 0x61, 0x6c,
    //         0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02,
    //         0x10, 0x02, 0x11, 0x05, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x01, 0x20, 0x07, 0x24, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0c, 0x14, 0x67, 0x65, 0x74, 0x5f, 0x72,
    //         0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    //         0x73, 0x20, 0x07, 0x02, 0x10, 0x00, 0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e, 0x61, 0x74,
    //         0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02, 0x10, 0x02, 0x11, 0x0f,
    //         0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65,
    //         0x72, 0x01, 0x20, 0x07, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x0c,
    //         0x04, 0x62, 0x75, 0x72, 0x6e, 0x20, 0x07, 0x02, 0x10, 0x00, 0x10, 0x43, 0x61, 0x6c,
    //         0x6c, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02,
    //         0x10, 0x02, 0x11, 0x07, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x01, 0x20, 0x07,
    //         0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x6d, 0x65, 0x74,
    //         0x68, 0x6f, 0x64, 0x20, 0x07, 0x02, 0x10, 0x00, 0x10, 0x43, 0x61, 0x6c, 0x6c, 0x4e,
    //         0x61, 0x74, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x02, 0x10, 0x02,
    //         0x11, 0x06, 0x47, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x01, 0x11, 0x08, 0x52, 0x65, 0x73,
    //         0x6f, 0x75, 0x72, 0x63, 0x65, 0x01, 0x82, 0x00, 0xf1, 0x58, 0x3c, 0xea, 0xb9, 0x56,
    //         0x3b, 0x76, 0x24, 0x1a, 0x2e, 0xe1, 0xf5, 0x04, 0xfe, 0xe3, 0x06, 0xcf, 0x2f, 0xe6,
    //         0xb4, 0x7b, 0xaa, 0x04, 0xd6, 0x0b, 0x0c, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64,
    //         0x20, 0x07, 0x02, 0x10, 0x00, 0x20, 0x20, 0x00,
    //     ];
    //
    //     let mut decoder = SborDecoder::new();
    //     let mut handler = EventCollector::new();
    //
    //     let mut start = 0;
    //     let mut end = 13;
    //
    //     while start < input.len() {
    //         match decoder.decode(&mut handler, &input[start..end]) {
    //             Ok(outcome) => {
    //                 if end - start == 13 {
    //                     assert_eq!(outcome, DecodingOutcome::NeedMoreData(end));
    //                 } else {
    //                     assert_eq!(outcome, DecodingOutcome::Done(input.len()))
    //                 }
    //             }
    //             Err(_err) => {
    //                 assert!(false, "Should not return an error")
    //             }
    //         }
    //
    //         start += 13;
    //         end += 13;
    //
    //         if end > input.len() {
    //             end = input.len();
    //         }
    //     }
    // }
}
