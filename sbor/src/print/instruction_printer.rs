use crate::bech32::network::*;
use crate::instruction::InstructionInfo;
use crate::instruction_extractor::ExtractorEvent;
use crate::math::Decimal;
use crate::print::address::*;
use crate::print::array::*;
use crate::print::custom_types::*;
use crate::print::decimals::*;
use crate::print::enums::*;
use crate::print::map::MAP_PARAMETER_PRINTER;
use crate::print::non_fungible::*;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::primitives::*;
use crate::print::state::{ParameterPrinterState, ValueState};
use crate::print::tty::TTY;
use crate::print::tuple::TUPLE_PARAMETER_PRINTER;
use crate::print::tx_summary_detector::Address;
use crate::sbor_decoder::{SborEvent, SubTypeKind};
use crate::type_info::*;

pub struct InstructionPrinter<T: Copy> {
    active_instruction: Option<InstructionInfo>,
    pub state: ParameterPrinterState<T>,
}

impl<T: Copy> InstructionPrinter<T> {
    pub fn new(network_id: NetworkId, tty: TTY<T>) -> Self {
        Self {
            active_instruction: None,
            state: ParameterPrinterState::new(network_id, tty),
        }
    }

    pub fn handle(&mut self, event: ExtractorEvent) {
        match event {
            ExtractorEvent::InstructionStart(info, count, total) => {
                self.start_instruction(info, count, total)
            }
            ExtractorEvent::ParameterStart(event, ..) => self.parameter_data(event),
            ExtractorEvent::ParameterData(data) => self.parameter_data(data),
            ExtractorEvent::ParameterEnd(event, ..) => self.parameter_end(event),
            ExtractorEvent::InstructionEnd => self.instruction_end(),
            // Error conditions
            ExtractorEvent::UnknownInstruction(..)
            | ExtractorEvent::InvalidEventSequence
            | ExtractorEvent::UnknownParameterType(..) => self.handle_error(),
        };
    }

    pub fn reset(&mut self) {
        self.active_instruction = None;
        self.state.reset();
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.state.set_network(network_id);
    }

    pub fn set_show_instructions(&mut self, show: bool) {
        self.state.set_show_instructions(show);
    }

    pub fn set_tty(&mut self, tty: TTY<T>) {
        self.state.set_tty(tty);
    }

    #[cfg(test)]
    fn get_tty(&self) -> &TTY<T> {
        self.state.get_tty()
    }

    fn handle_error(&mut self) {
        self.state.start();
        self.state.print_text(b"Unable to decode transaction intent. Either, input is invalid or application is outdated.");
        self.state.end();
    }

    fn start_instruction(&mut self, info: InstructionInfo, count: u32, total: u32) {
        self.active_instruction = Some(info);
        self.state.start();

        print_u32(&mut self.state.title, count + 1);
        self.state.title.extend_from_slice(b" of ");
        print_u32(&mut self.state.title, total);

        self.state.print_text(info.name);
        self.state.print_space();
    }

    fn instruction_end(&mut self) {
        if let Some(..) = self.active_instruction {
            self.state.end();
        }
    }

    fn parameter_data(&mut self, source_event: SborEvent) {
        match source_event {
            SborEvent::Start {
                type_id,
                nesting_level,
                ..
            } => {
                if self.state.stack.is_not_empty() {
                    Dispatcher::subcomponent_start(&mut self.state);
                }

                self.state.nesting_level = nesting_level;
                self.state.stack.push(ValueState::new(type_id));
                Dispatcher::start(&mut self.state);
            }
            SborEvent::ElementType { kind, type_id } => {
                match kind {
                    SubTypeKind::Key => self.active_value_state().key_type_id = type_id,
                    SubTypeKind::Value => self.active_value_state().element_type_id = type_id,
                    SubTypeKind::Element => self.active_value_state().element_type_id = type_id,
                }
                Dispatcher::handle_data(&mut self.state, source_event);
            }
            SborEvent::Discriminator(discriminator) => {
                self.active_value_state().key_type_id = discriminator;
                Dispatcher::handle_data(&mut self.state, source_event);
            }
            SborEvent::End {
                type_id: _,
                nesting_level,
            } => {
                Dispatcher::end(&mut self.state);
                self.state.nesting_level = nesting_level;
                self.state.stack.pop().expect("Stack can't be empty");

                if self.state.stack.is_not_empty() {
                    Dispatcher::subcomponent_end(&mut self.state);
                } else {
                    self.state.print_space();
                }

                self.state.data.clear();
            }
            _ => {
                Dispatcher::handle_data(&mut self.state, source_event);
            }
        }
    }

    fn active_value_state(&mut self) -> &mut ValueState {
        self.state.active_state()
    }

    fn parameter_end(&mut self, event: SborEvent) {
        self.parameter_data(event);
        self.state.reset();
    }

    pub fn format_decimal(&mut self, value: &Decimal, suffix: &[u8]) -> &[u8] {
        self.state.data.clear();
        value.format(&mut self.state.data);
        self.state.data.extend_from_slice(suffix);
        self.state.data.as_slice()
    }

    pub fn format_address(&mut self, address: &Address) -> &[u8] {
        self.state.format_address(address)
    }
}

struct Dispatcher;

// Workaround for not working vtables
impl Dispatcher {
    pub fn handle_data<T: Copy>(state: &mut ParameterPrinterState<T>, event: SborEvent) {
        let discriminator = state.active_state().main_type_id;
        match discriminator {
            TYPE_BOOL => BOOL_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_I8 => I8_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_I16 => I16_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_I32 => I32_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_I64 => I64_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_I128 => I128_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_U8 => U8_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_U16 => U16_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_U32 => U32_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_U64 => U64_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_U128 => U128_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_STRING => STRING_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_ARRAY => ARRAY_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_TUPLE => TUPLE_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_ENUM => ENUM_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_MAP => MAP_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_ADDRESS => ADDRESS_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_BUCKET => BUCKET_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_PROOF => PROOF_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_EXPRESSION => EXPRESSION_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_BLOB => BLOB_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_DECIMAL => DECIMAL_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_PRECISE_DECIMAL => PRECISE_DECIMAL_PARAMETER_PRINTER.handle_data(state, event),
            TYPE_NON_FUNGIBLE_LOCAL_ID => {
                NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER.handle_data(state, event)
            }
            TYPE_ADDRESS_RESERVATION => {
                ADDRESS_RESERVATION_PARAMETER_PRINTER.handle_data(state, event)
            }
            _ => IGNORED_PARAMETER_PRINTER.handle_data(state, event),
        };
    }

    pub fn start<T: Copy>(state: &mut ParameterPrinterState<T>) {
        let discriminator = state.active_state().main_type_id;
        match discriminator {
            TYPE_BOOL => BOOL_PARAMETER_PRINTER.start(state),
            TYPE_I8 => I8_PARAMETER_PRINTER.start(state),
            TYPE_I16 => I16_PARAMETER_PRINTER.start(state),
            TYPE_I32 => I32_PARAMETER_PRINTER.start(state),
            TYPE_I64 => I64_PARAMETER_PRINTER.start(state),
            TYPE_I128 => I128_PARAMETER_PRINTER.start(state),
            TYPE_U8 => U8_PARAMETER_PRINTER.start(state),
            TYPE_U16 => U16_PARAMETER_PRINTER.start(state),
            TYPE_U32 => U32_PARAMETER_PRINTER.start(state),
            TYPE_U64 => U64_PARAMETER_PRINTER.start(state),
            TYPE_U128 => U128_PARAMETER_PRINTER.start(state),
            TYPE_STRING => STRING_PARAMETER_PRINTER.start(state),
            TYPE_ARRAY => ARRAY_PARAMETER_PRINTER.start(state),
            TYPE_TUPLE => TUPLE_PARAMETER_PRINTER.start(state),
            TYPE_ENUM => ENUM_PARAMETER_PRINTER.start(state),
            TYPE_MAP => MAP_PARAMETER_PRINTER.start(state),
            TYPE_ADDRESS => ADDRESS_PARAMETER_PRINTER.start(state),
            TYPE_BUCKET => BUCKET_PARAMETER_PRINTER.start(state),
            TYPE_PROOF => PROOF_PARAMETER_PRINTER.start(state),
            TYPE_EXPRESSION => EXPRESSION_PARAMETER_PRINTER.start(state),
            TYPE_BLOB => BLOB_PARAMETER_PRINTER.start(state),
            TYPE_DECIMAL => DECIMAL_PARAMETER_PRINTER.start(state),
            TYPE_PRECISE_DECIMAL => PRECISE_DECIMAL_PARAMETER_PRINTER.start(state),
            TYPE_NON_FUNGIBLE_LOCAL_ID => NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER.start(state),
            TYPE_ADDRESS_RESERVATION => ADDRESS_RESERVATION_PARAMETER_PRINTER.start(state),
            _ => IGNORED_PARAMETER_PRINTER.start(state),
        };
    }

    pub fn end<T: Copy>(state: &mut ParameterPrinterState<T>) {
        let discriminator = state.active_state().main_type_id;
        match discriminator {
            TYPE_BOOL => BOOL_PARAMETER_PRINTER.end(state),
            TYPE_I8 => I8_PARAMETER_PRINTER.end(state),
            TYPE_I16 => I16_PARAMETER_PRINTER.end(state),
            TYPE_I32 => I32_PARAMETER_PRINTER.end(state),
            TYPE_I64 => I64_PARAMETER_PRINTER.end(state),
            TYPE_I128 => I128_PARAMETER_PRINTER.end(state),
            TYPE_U8 => U8_PARAMETER_PRINTER.end(state),
            TYPE_U16 => U16_PARAMETER_PRINTER.end(state),
            TYPE_U32 => U32_PARAMETER_PRINTER.end(state),
            TYPE_U64 => U64_PARAMETER_PRINTER.end(state),
            TYPE_U128 => U128_PARAMETER_PRINTER.end(state),
            TYPE_STRING => STRING_PARAMETER_PRINTER.end(state),
            TYPE_ARRAY => ARRAY_PARAMETER_PRINTER.end(state),
            TYPE_TUPLE => TUPLE_PARAMETER_PRINTER.end(state),
            TYPE_ENUM => ENUM_PARAMETER_PRINTER.end(state),
            TYPE_MAP => MAP_PARAMETER_PRINTER.end(state),
            TYPE_ADDRESS => ADDRESS_PARAMETER_PRINTER.end(state),
            TYPE_BUCKET => BUCKET_PARAMETER_PRINTER.end(state),
            TYPE_PROOF => PROOF_PARAMETER_PRINTER.end(state),
            TYPE_EXPRESSION => EXPRESSION_PARAMETER_PRINTER.end(state),
            TYPE_BLOB => BLOB_PARAMETER_PRINTER.end(state),
            TYPE_DECIMAL => DECIMAL_PARAMETER_PRINTER.end(state),
            TYPE_PRECISE_DECIMAL => PRECISE_DECIMAL_PARAMETER_PRINTER.end(state),
            TYPE_NON_FUNGIBLE_LOCAL_ID => NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER.end(state),
            TYPE_ADDRESS_RESERVATION => ADDRESS_RESERVATION_PARAMETER_PRINTER.end(state),
            _ => IGNORED_PARAMETER_PRINTER.end(state),
        };
    }

    pub fn subcomponent_start<T: Copy>(state: &mut ParameterPrinterState<T>) {
        let discriminator = state.active_state().main_type_id;
        match discriminator {
            TYPE_BOOL => BOOL_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_I8 => I8_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_I16 => I16_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_I32 => I32_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_I64 => I64_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_I128 => I128_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_U8 => U8_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_U16 => U16_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_U32 => U32_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_U64 => U64_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_U128 => U128_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_STRING => STRING_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_ARRAY => ARRAY_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_TUPLE => TUPLE_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_ENUM => ENUM_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_MAP => MAP_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_ADDRESS => ADDRESS_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_BUCKET => BUCKET_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_PROOF => PROOF_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_EXPRESSION => EXPRESSION_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_BLOB => BLOB_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_DECIMAL => DECIMAL_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_PRECISE_DECIMAL => PRECISE_DECIMAL_PARAMETER_PRINTER.subcomponent_start(state),
            TYPE_NON_FUNGIBLE_LOCAL_ID => {
                NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER.subcomponent_start(state)
            }
            TYPE_ADDRESS_RESERVATION => {
                ADDRESS_RESERVATION_PARAMETER_PRINTER.subcomponent_start(state)
            }
            _ => IGNORED_PARAMETER_PRINTER.subcomponent_start(state),
        };
    }

    pub fn subcomponent_end<T: Copy>(state: &mut ParameterPrinterState<T>) {
        let discriminator = state.active_state().main_type_id;
        match discriminator {
            TYPE_BOOL => BOOL_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_I8 => I8_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_I16 => I16_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_I32 => I32_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_I64 => I64_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_I128 => I128_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_U8 => U8_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_U16 => U16_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_U32 => U32_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_U64 => U64_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_U128 => U128_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_STRING => STRING_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_ARRAY => ARRAY_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_TUPLE => TUPLE_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_ENUM => ENUM_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_MAP => MAP_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_ADDRESS => ADDRESS_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_BUCKET => BUCKET_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_PROOF => PROOF_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_EXPRESSION => EXPRESSION_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_BLOB => BLOB_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_DECIMAL => DECIMAL_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_PRECISE_DECIMAL => PRECISE_DECIMAL_PARAMETER_PRINTER.subcomponent_end(state),
            TYPE_NON_FUNGIBLE_LOCAL_ID => {
                NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER.subcomponent_end(state)
            }
            TYPE_ADDRESS_RESERVATION => {
                ADDRESS_RESERVATION_PARAMETER_PRINTER.subcomponent_end(state)
            }
            _ => IGNORED_PARAMETER_PRINTER.subcomponent_end(state),
        };
    }
}

#[cfg(test)]
mod tests {
    use core::cmp::min;
    use core::str::from_utf8;

    use crate::bech32::network::NetworkId;
    use crate::instruction_extractor::*;
    use crate::print::fanout::Fanout;
    use crate::print::tty::TTY;
    use crate::print::tx_intent_type::TxIntentType;
    use crate::print::tx_summary_detector::{Address, DetectedTxType, TxSummaryDetector};
    use crate::sbor_decoder::*;
    use crate::static_vec::AsSlice;
    use crate::tx_intent_test_data::tests::*;

    use super::*;

    const CHUNK_SIZE: usize = 255;
    const MAX_OUTPUT_SIZE: usize = 16384;

    #[derive(Copy, Clone, Debug)]
    pub struct TestTTY;

    #[derive(Copy, Clone, Debug)]
    pub struct OutputContainer {
        data: [u8; MAX_OUTPUT_SIZE],
        counter: usize,
    }

    impl AsSlice<u8> for OutputContainer {
        fn as_slice(&self) -> &[u8] {
            &self.data[..self.counter]
        }
    }

    impl OutputContainer {
        pub const fn new() -> Self {
            Self {
                data: [0; MAX_OUTPUT_SIZE],
                counter: 0,
            }
        }

        pub fn extend_from_slice(&mut self, data: &[u8]) {
            for &byte in data {
                self.data[self.counter] = byte;
                self.counter += 1;
            }
        }

        pub fn push(&mut self, byte: u8) {
            self.data[self.counter] = byte;
            self.counter += 1;
        }
    }

    impl TestTTY {
        pub const fn new_tty() -> TTY<OutputContainer> {
            TTY {
                data: OutputContainer::new(),
                show_message: Self::show_message,
            }
        }

        fn show_message(data: &mut OutputContainer, title: &[u8], message: &[u8]) {
            data.extend_from_slice(title);
            data.push(b':');
            data.push(b' ');
            data.extend_from_slice(message);
            data.push(b'\n');
        }
    }

    pub struct InstructionProcessor<T: Copy> {
        extractor: InstructionExtractor,
        printer: InstructionPrinter<T>,
        detector: TxSummaryDetector,
        counter: usize,
    }

    impl<T: Copy> SborEventHandler for InstructionProcessor<T> {
        fn handle(&mut self, evt: SborEvent) {
            match evt {
                SborEvent::InputByte(..) => self.counter += 1,
                _ => {}
            }
            let mut fanout = Fanout::new(&mut self.printer, &mut self.detector);
            self.extractor.handle_event(&mut fanout, evt);
        }
    }

    impl<T: Copy + AsSlice<u8>> InstructionProcessor<T> {
        pub fn new(tty: TTY<T>) -> Self {
            Self {
                extractor: InstructionExtractor::new(),
                printer: InstructionPrinter::new(NetworkId::LocalNet, tty),
                detector: TxSummaryDetector::new(),
                counter: 0,
            }
        }

        pub fn get_count(&self) -> usize {
            self.counter
        }

        pub fn set_intent_type(&mut self, intent_type: TxIntentType) {
            self.detector.set_intent_type(intent_type);
        }

        pub fn verify(&self, expected: &[u8], expected_type: &DetectedTxType) {
            let mut cnt = 0;
            let output = from_utf8(self.printer.get_tty().data.as_slice()).unwrap();

            if expected.len() < 10 {
                println!("Output:\n|{}|", output);
            }
            assert!(expected.len() > 10);

            output
                .split('\n')
                .zip(from_utf8(expected).unwrap().split('\n').skip(1))
                .all(|(a, b)| {
                    assert_eq!(
                        a.trim(),
                        b.trim(),
                        "Elements are not equal at index {}",
                        cnt
                    );
                    cnt += 1;
                    true
                });

            match expected_type {
                DetectedTxType::Transfer { fee, .. } | DetectedTxType::Other(fee) => match fee {
                    Some(fee) => {
                        println!("Expected Fee: {}", fee);
                    }
                    None => {}
                },
                _ => {}
            }

            let detected = self.detector.get_detected_tx_type();

            match detected {
                DetectedTxType::Transfer { fee, .. } | DetectedTxType::Other(fee) => match fee {
                    Some(fee) => {
                        println!("Detected Fee: {}", fee);
                    }
                    None => {}
                },
                _ => {}
            }

            assert!(
                detected.is_same(expected_type),
                "Detected tx type {:?} does not match expected {:?}",
                detected,
                expected_type
            );
        }
    }

    fn check_partial_decoding(input: &[u8], expected_text: &[u8], expected_type: &DetectedTxType) {
        check_partial_decoding_with_type(
            input,
            expected_text,
            expected_type,
            TxIntentType::General,
        );
    }

    fn check_partial_decoding_with_type(
        input: &[u8],
        expected_text: &[u8],
        expected_type: &DetectedTxType,
        intent_type: TxIntentType,
    ) {
        let mut decoder = SborDecoder::new(true);
        let mut processor = InstructionProcessor::new(TestTTY::new_tty());

        processor.set_intent_type(intent_type);

        let mut start = 0;
        let mut end = min(input.len(), CHUNK_SIZE);

        while start < input.len() {
            match decoder.decode(&mut processor, &input[start..end]) {
                Ok(outcome) => {
                    if end - start <= CHUNK_SIZE && end < input.len() {
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

            if end >= input.len() {
                end = input.len();
            }
        }

        processor.verify(expected_text, expected_type);

        let num_bytes = processor.get_count();

        assert_eq!(
            num_bytes,
            input.len() - 1,
            "Number of bytes processed does not match input length"
        );

        println!();
    }

    #[test]
    fn test_access_rule() {
        check_partial_decoding(
            &TX_ACCESS_RULE,
br##"
1 of 6: CallAccessRulesMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set_owner_role" Tuple(Enum<0u8>(), )
2 of 6: CallAccessRulesMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "lock_owner_role" Tuple()
3 of 6: CallAccessRulesMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set_and_lock_owner_role" Tuple(Enum<0u8>(), )
4 of 6: CallAccessRulesMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set_role" Tuple(Enum<0u8>(), "hello", Enum<0u8>(), )
5 of 6: CallAccessRulesMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "lock_role" Tuple(Enum<0u8>(), "hello", )
6 of 6: CallAccessRulesMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set_and_lock_role" Tuple(Enum<0u8>(), "hello", Enum<0u8>(), )
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_address_allocation() {
        check_partial_decoding(
            &TX_ADDRESS_ALLOCATION,
br##"
1 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 4: AllocateGlobalAddress Address(package_loc1pkgxxxxxxxxxpackgexxxxxxxxx000726633226xxxxxxxxxwqy6uc) "Package"
3 of 4: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxpackgexxxxxxxxx000726633226xxxxxxxxxwqy6uc), ) "Package" "publish_wasm_advanced" Tuple(Enum<1u8>(AddressReservation(0u32), ), Blob(a710f0959d8e139b3c1ca74ac4fcb9a95ada2c82e7f563304c5487e0117095c0), Tuple(Map<String, Tuple>(), ), Map<String, Tuple>(), Enum<0u8>(), )
4 of 4: CallFunction Enum<1u8>(0u32, ) "BlueprintName" "no_such_function" Tuple(Decimal(1), Address(0u32), )
"##,
            &DetectedTxType::Other(Some(Decimal::whole(500))),
        );
    }
    #[test]
    fn test_call_function() {
        check_partial_decoding(
            &TX_CALL_FUNCTION,
br##"
1 of 1: CallFunction Enum<0u8>(Address(package_loc1p4r4955skdjq9swg8s5jguvcjvyj7tsxct87a9z6sw76cdfdmytuxt), ) "BlueprintName" "f" Tuple("string", )
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_call_method() {
        check_partial_decoding(
            &TX_CALL_METHOD,
br##"
1 of 4: CallMethod Enum<0u8>(Address(component_loc1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve2jtvlp), ) "complicated_method" Tuple(Decimal(1), PreciseDecimal(2), )
2 of 4: CallRoyaltyMethod Enum<0u8>(Address(component_loc1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve2jtvlp), ) "set_royalty" Tuple("my_method", Enum<0u8>(), )
3 of 4: CallMetadataMethod Enum<0u8>(Address(component_loc1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve2jtvlp), ) "get" Tuple("HelloWorld", )
4 of 4: CallAccessRulesMethod Enum<0u8>(Address(component_loc1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve2jtvlp), ) "get_role" Tuple(Enum<0u8>(), "hello", )
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_create_access_controller() {
        check_partial_decoding(
            &TX_CREATE_ACCESS_CONTROLLER,
br##"
1 of 2: TakeAllFromWorktop Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq)
2 of 2: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxcntrlrxxxxxxxxx000648572295xxxxxxxxxhwh0tz), ) "AccessController" "create_global" Tuple(Bucket(0u32), Tuple(Enum<1u8>(), Enum<1u8>(), Enum<1u8>(), ), Enum<0u8>(), )
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_create_account() {
        check_partial_decoding(
            &TX_CREATE_ACCOUNT,
br##"
1 of 2: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxaccntxxxxxxxxxx000929625493xxxxxxxxxj9yll8), ) "Account" "create_advanced" Tuple(Enum<2u8>(Enum<0u8>(), ), )
2 of 2: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxaccntxxxxxxxxxx000929625493xxxxxxxxxj9yll8), ) "Account" "create" Tuple()
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_create_fungible_resource_with_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
br##"
1 of 3: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 3: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxresrcexxxxxxxxx000538436477xxxxxxxxxvyv0vc), ) "FungibleResourceManager" "create_with_initial_supply" Tuple(Enum<0u8>(), false, 18u8, Decimal(12), Tuple(Enum<1u8>(Tuple(Tuple(Enum<1u8>(Enum<0u8>(), ), true, ), Tuple(Enum<1u8>(Enum<1u8>(), ), true, ), ), ), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), ), Tuple(Map<String, Tuple>({"name", Tuple(Enum<1u8>(Enum<0u8>("MyResource", ), ), true, )}, {"symbol", Tuple(Enum<1u8>(Enum<0u8>("RSRC", ), ), true, )}, {"description", Tuple(Enum<1u8>(Enum<0u8>("A very innovative and important resource", ), ), true, )}, ), Map<String, Tuple>(), ), Enum<0u8>(), )
3 of 3: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "deposit_batch" Tuple(Expression(ENTIRE_WORKTOP), )
"##,
            &DetectedTxType::Other(Some(Decimal::whole(500))),
        );
    }
    #[test]
    fn test_create_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
br##"
1 of 2: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 2: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxresrcexxxxxxxxx000538436477xxxxxxxxxvyv0vc), ) "FungibleResourceManager" "create" Tuple(Enum<0u8>(), false, 18u8, Tuple(Enum<1u8>(Tuple(Tuple(Enum<1u8>(Enum<0u8>(), ), true, ), Tuple(Enum<1u8>(Enum<1u8>(), ), true, ), ), ), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), ), Tuple(Map<String, Tuple>({"name", Tuple(Enum<1u8>(Enum<0u8>("MyResource", ), ), true, )}, {"symbol", Tuple(Enum<1u8>(Enum<0u8>("RSRC", ), ), true, )}, {"description", Tuple(Enum<1u8>(Enum<0u8>("A very innovative and important resource", ), ), true, )}, ), Map<String, Tuple>(), ), Enum<0u8>(), )
"##,
            &DetectedTxType::Other(Some(Decimal::whole(500))),
        );
    }
    #[test]
    fn test_create_identity() {
        check_partial_decoding(
            &TX_CREATE_IDENTITY,
br##"
1 of 2: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxdntyxxxxxxxxxxx008560783089xxxxxxxxxzwhgj8), ) "Identity" "create_advanced" Tuple(Enum<0u8>(), )
2 of 2: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxdntyxxxxxxxxxxx008560783089xxxxxxxxxzwhgj8), ) "Identity" "create" Tuple()
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_create_non_fungible_resource_with_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
br##"
1 of 3: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 3: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxresrcexxxxxxxxx000538436477xxxxxxxxxvyv0vc), ) "NonFungibleResourceManager" "create_with_initial_supply" Tuple(Enum<0u8>(), Enum<1u8>(), false, Tuple(Tuple(Array<Enum>(), Array<Tuple>(), Array<Enum>(), ), Enum<0u8>(64u8, ), Array<String>(), ), Map<NonFungibleLocalId, Tuple>({#12u64#, Tuple(Tuple("Hello World", Decimal(12), ), )}, ), Tuple(Enum<1u8>(Tuple(Tuple(Enum<1u8>(Enum<0u8>(), ), true, ), Tuple(Enum<1u8>(Enum<1u8>(), ), true, ), ), ), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), ), Tuple(Map<String, Tuple>({"name", Tuple(Enum<1u8>(Enum<0u8>("MyResource", ), ), true, )}, {"description", Tuple(Enum<1u8>(Enum<0u8>("A very innovative and important resource", ), ), false, )}, ), Map<String, Tuple>(), ), Enum<0u8>(), )
3 of 3: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "deposit_batch" Tuple(Expression(ENTIRE_WORKTOP), )
"##,
            &DetectedTxType::Other(Some(Decimal::whole(500))),
        );
    }
    #[test]
    fn test_create_non_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
br##"
1 of 2: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 2: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxresrcexxxxxxxxx000538436477xxxxxxxxxvyv0vc), ) "NonFungibleResourceManager" "create" Tuple(Enum<0u8>(), Enum<1u8>(), false, Tuple(Tuple(Array<Enum>(), Array<Tuple>(), Array<Enum>(), ), Enum<0u8>(64u8, ), Array<String>(), ), Tuple(Enum<1u8>(Tuple(Tuple(Enum<1u8>(Enum<0u8>(), ), true, ), Tuple(Enum<1u8>(Enum<1u8>(), ), true, ), ), ), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), Enum<0u8>(), ), Tuple(Map<String, Tuple>({"name", Tuple(Enum<1u8>(Enum<0u8>("MyResource", ), ), true, )}, {"description", Tuple(Enum<1u8>(Enum<0u8>("A very innovative and important resource", ), ), false, )}, ), Map<String, Tuple>(), ), Enum<0u8>(), )
"##,
            &DetectedTxType::Other(Some(Decimal::whole(500))),
        );
    }
    #[test]
    fn test_create_validator() {
        check_partial_decoding(
            &TX_CREATE_VALIDATOR,
br##"
1 of 3: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "withdraw" Tuple(Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv), Decimal(1000), )
2 of 3: TakeFromWorktop Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv) Decimal(1000)
3 of 3: CallMethod Enum<0u8>(Address(consensusmanager_loc1scxxxxxxxxxxcnsmgrxxxxxxxxx000999665565xxxxxxxxxhwvhuz), ) "create_validator" Tuple(Bytes(02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5), Decimal(1), Bucket(0u32), )
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_metadata() {
        check_partial_decoding(
            &TX_METADATA,
br##"
1 of 25: CallMetadataMethod Enum<0u8>(Address(package_loc1p4r4955skdjq9swg8s5jguvcjvyj7tsxct87a9z6sw76cdfdmytuxt), ) "set" Tuple("field_name", Enum<0u8>("Metadata string value, eg description", ), )
2 of 25: CallMetadataMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "set" Tuple("field_name", Enum<0u8>("Metadata string value, eg description", ), )
3 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<0u8>("Metadata string value, eg description", ), )
4 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<1u8>(true, ), )
5 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<2u8>(123u8, ), )
6 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<3u8>(123u32, ), )
7 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<4u8>(123u64, ), )
8 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<5u8>(-123i32, ), )
9 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<6u8>(-123i64, ), )
10 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<7u8>(Decimal(10.5), ), )
11 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<8u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ), )
12 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<9u8>(Enum<0u8>(Bytes(0000000000000000000000000000000000000000000000000000000000000000ff), ), ), )
13 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<10u8>(Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), <some_string>, ), ), )
14 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<11u8>(<some_string>, ), )
15 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<12u8>(10000i64, ), )
16 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<13u8>("https://radixdlt.com/index.html", ), )
17 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<14u8>("https://radixdlt.com", ), )
18 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<15u8>(Enum<0u8>(Bytes(0000000000000000000000000000000000000000000000000000000000), ), ), )
19 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "set" Tuple("field_name", Enum<128u8>(Array<String>("some_string", "another_string", "yet_another_string", ), ), )
20 of 25: CallMetadataMethod Enum<0u8>(Address(package_loc1p4r4955skdjq9swg8s5jguvcjvyj7tsxct87a9z6sw76cdfdmytuxt), ) "lock" Tuple("field_name", )
21 of 25: CallMetadataMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock" Tuple("field_name", )
22 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "lock" Tuple("field_name", )
23 of 25: CallMetadataMethod Enum<0u8>(Address(package_loc1p4r4955skdjq9swg8s5jguvcjvyj7tsxct87a9z6sw76cdfdmytuxt), ) "remove" Tuple("field_name", )
24 of 25: CallMetadataMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "remove" Tuple("field_name", )
25 of 25: CallMetadataMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "remove" Tuple("field_name", )
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_mint_fungible() {
        check_partial_decoding(
            &TX_MINT_FUNGIBLE,
br##"
1 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "create_proof_of_amount" Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), Decimal(1), )
3 of 4: CallMethod Enum<0u8>(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), ) "mint" Tuple(Decimal(12), )
4 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "deposit_batch" Tuple(Expression(ENTIRE_WORKTOP), )
"##,
            &DetectedTxType::Other(Some(Decimal::whole(500))),
        );
    }
    #[test]
    fn test_mint_non_fungible() {
        check_partial_decoding(
            &TX_MINT_NON_FUNGIBLE,
br##"
1 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "create_proof_of_amount" Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), Decimal(1), )
3 of 4: CallMethod Enum<0u8>(Address(resource_loc1nfhtg7ttszgjwysfglx8jcjtvv8q02fg9s2y6qpnvtw5jsy3l6u6k8), ) "mint" Tuple(Map<NonFungibleLocalId, Tuple>({#12u64#, Tuple(Tuple(), )}, ), )
4 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "deposit_batch" Tuple(Expression(ENTIRE_WORKTOP), )
"##,
            &DetectedTxType::Other(Some(Decimal::whole(500))),
        );
    }
    #[test]
    fn test_publish_package() {
        check_partial_decoding(
            &TX_PUBLISH_PACKAGE,
br##"
1 of 2: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(5000), )
2 of 2: CallFunction Enum<0u8>(Address(package_loc1pkgxxxxxxxxxpackgexxxxxxxxx000726633226xxxxxxxxxwqy6uc), ) "Package" "publish_wasm_advanced" Tuple(Enum<0u8>(), Blob(a710f0959d8e139b3c1ca74ac4fcb9a95ada2c82e7f563304c5487e0117095c0), Tuple(Map<String, Tuple>(), ), Map<String, Tuple>(), Enum<0u8>(), )
"##,
            &DetectedTxType::Other(Some(Decimal::whole(5000))),
        );
    }
    #[test]
    fn test_resource_auth_zone() {
        check_partial_decoding(
            &TX_RESOURCE_AUTH_ZONE,
br##"
1 of 22: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "withdraw" Tuple(Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv), Decimal(5), )
2 of 22: TakeAllFromWorktop Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv)
3 of 22: CreateProofFromBucket Bucket(0u32)
4 of 22: CreateProofFromBucketOfAmount Bucket(0u32) Decimal(1)
5 of 22: CreateProofFromBucketOfNonFungibles Bucket(0u32) Array<NonFungibleLocalId>(#123u64#, )
6 of 22: CreateProofFromBucketOfAll Bucket(0u32)
7 of 22: CloneProof Proof(0u32)
8 of 22: DropProof Proof(0u32)
9 of 22: DropProof Proof(4u32)
11 of 22: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "create_proof_of_amount" Tuple(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), Decimal(5), )
13 of 22: DropProof Proof(5u32)
14 of 22: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "create_proof_of_amount" Tuple(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), Decimal(5), )
15 of 22: CreateProofFromAuthZone Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al)
16 of 22: CreateProofFromAuthZoneOfAmount Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al) Decimal(1)
17 of 22: CreateProofFromAuthZoneOfNonFungibles Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq) Array<NonFungibleLocalId>(#123u64#, )
18 of 22: CreateProofFromAuthZoneOfAll Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq)
22 of 22: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "deposit_batch" Tuple(Expression(ENTIRE_WORKTOP), )"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_resource_recall() {
        check_partial_decoding(
            &TX_RESOURCE_RECALL,
br##"
1 of 1: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "recall" Tuple(Decimal(1.2), )
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_resource_worktop() {
        check_partial_decoding(
            &TX_RESOURCE_WORKTOP,
br##"
1 of 9: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "withdraw" Tuple(Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv), Decimal(5), )
2 of 9: TakeFromWorktop Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv) Decimal(2)
3 of 9: CallMethod Enum<0u8>(Address(component_loc1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve2jtvlp), ) "buy_gumball" Tuple(Bucket(0u32), )
4 of 9: AssertWorktopContainsAny Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al)
5 of 9: AssertWorktopContains Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al) Decimal(3)
6 of 9: TakeAllFromWorktop Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv)
7 of 9: ReturnToWorktop Bucket(1u32)
8 of 9: TakeNonFungiblesFromWorktop Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq) Array<NonFungibleLocalId>(#1u64#, )
9 of 9: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "deposit_batch" Tuple(Expression(ENTIRE_WORKTOP), )
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_royalty() {
        check_partial_decoding(
            &TX_ROYALTY,
br##"
1 of 4: CallRoyaltyMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "set_royalty" Tuple("my_method", Enum<0u8>(), )
2 of 4: CallRoyaltyMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_royalty" Tuple("my_method", )
3 of 4: CallMethod Enum<0u8>(Address(package_loc1p4r4955skdjq9swg8s5jguvcjvyj7tsxct87a9z6sw76cdfdmytuxt), ) "PackageRoyalty_claim_royalties" Tuple()
4 of 4: CallRoyaltyMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "claim_royalties" Tuple()
"##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_simple_transfer_nft_by_id() {
        check_partial_decoding_with_type(
            &TX_SIMPLE_TRANSFER_NFT_BY_ID,
br##"
1 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "withdraw_non_fungibles" Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), Array<NonFungibleLocalId>(#1u64#, #2u64#, #3u64#, ), )
3 of 4: TakeNonFungiblesFromWorktop Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq) Array<NonFungibleLocalId>(#1u64#, #2u64#, #3u64#, )
4 of 4: CallMethod Enum<0u8>(Address(account_loc1cyzfj6p254jy6lhr237s7pcp8qqz6c8ahq9mn6nkdjxxxat5pjq9xc), ) "try_deposit_or_abort" Tuple(Bucket(0u32), )
"##,
            &DetectedTxType::Transfer {
                fee: Some(Decimal::whole(500)),
                src_address: Address::from_array([
                    0xc1, 0x18, 0x83, 0x46, 0x2f, 0x39, 0x79, 0x6d, 0xa8, 0x3f, 0x2f, 0x82, 0xca,
                    0xef, 0xa6, 0x79, 0xaa, 0xf1, 0xf1, 0x89, 0x25, 0x7e, 0xbd, 0x3c, 0x8c, 0x27,
                    0x7d, 0x5a, 0xe1, 0x99,
                ]),
                dst_address: Address::from_array([
                    0xc1, 0x04, 0x99, 0x68, 0x2a, 0xa5, 0x64, 0x4d, 0x7e, 0xe3, 0x54, 0x7d, 0x0f,
                    0x07, 0x01, 0x38, 0x00, 0x2d, 0x60, 0xfd, 0xb8, 0x0b, 0xb9, 0xea, 0x76, 0x6c,
                    0x8c, 0x63, 0x75, 0x74,
                ]),
                res_address: Address::from_array([
                    0x9a, 0x2c, 0xb6, 0x13, 0x39, 0x9b, 0x18, 0x0c, 0xae, 0x60, 0x71, 0x21, 0x96,
                    0x60, 0xd4, 0x49, 0x19, 0xfd, 0x5c, 0x19, 0x8e, 0x89, 0x11, 0xda, 0xed, 0x4b,
                    0x67, 0xae, 0x09, 0x8b,
                ]),
                amount: Decimal::whole(3),
            },
            TxIntentType::Transfer,
        );
    }
    #[test]
    fn test_simple_transfer_nft() {
        check_partial_decoding_with_type(
            &TX_SIMPLE_TRANSFER_NFT,
br##"
1 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "withdraw_non_fungibles" Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), Array<NonFungibleLocalId>(#1u64#, #2u64#, ), )
3 of 4: TakeFromWorktop Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq) Decimal(2)
4 of 4: CallMethod Enum<0u8>(Address(account_loc1cyzfj6p254jy6lhr237s7pcp8qqz6c8ahq9mn6nkdjxxxat5pjq9xc), ) "try_deposit_or_abort" Tuple(Bucket(0u32), )
"##,
            &DetectedTxType::Transfer {
                fee: Some(Decimal::whole(500)),
                src_address: Address::from_array([
                    0xc1, 0x18, 0x83, 0x46, 0x2f, 0x39, 0x79, 0x6d, 0xa8, 0x3f, 0x2f, 0x82, 0xca,
                    0xef, 0xa6, 0x79, 0xaa, 0xf1, 0xf1, 0x89, 0x25, 0x7e, 0xbd, 0x3c, 0x8c, 0x27,
                    0x7d, 0x5a, 0xe1, 0x99,
                ]),
                dst_address: Address::from_array([
                    0xc1, 0x04, 0x99, 0x68, 0x2a, 0xa5, 0x64, 0x4d, 0x7e, 0xe3, 0x54, 0x7d, 0x0f,
                    0x07, 0x01, 0x38, 0x00, 0x2d, 0x60, 0xfd, 0xb8, 0x0b, 0xb9, 0xea, 0x76, 0x6c,
                    0x8c, 0x63, 0x75, 0x74,
                ]),
                res_address: Address::from_array([
                    0x9a, 0x2c, 0xb6, 0x13, 0x39, 0x9b, 0x18, 0x0c, 0xae, 0x60, 0x71, 0x21, 0x96,
                    0x60, 0xd4, 0x49, 0x19, 0xfd, 0x5c, 0x19, 0x8e, 0x89, 0x11, 0xda, 0xed, 0x4b,
                    0x67, 0xae, 0x09, 0x8b,
                ]),
                amount: Decimal::whole(2),
            },
            TxIntentType::Transfer,
        );
    }
    #[test]
    fn test_simple_transfer() {
        check_partial_decoding_with_type(
            &TX_SIMPLE_TRANSFER,
br##"
1 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(500), )
2 of 4: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "withdraw" Tuple(Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al), Decimal(123), )
3 of 4: TakeFromWorktop Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al) Decimal(123)
4 of 4: CallMethod Enum<0u8>(Address(account_loc1cyzfj6p254jy6lhr237s7pcp8qqz6c8ahq9mn6nkdjxxxat5pjq9xc), ) "try_deposit_or_abort" Tuple(Bucket(0u32), )
"##,
            &DetectedTxType::Transfer {
                fee: Some(Decimal::whole(500)),
                src_address: Address::from_array([
                    0xc1, 0x18, 0x83, 0x46, 0x2f, 0x39, 0x79, 0x6d, 0xa8, 0x3f, 0x2f, 0x82, 0xca,
                    0xef, 0xa6, 0x79, 0xaa, 0xf1, 0xf1, 0x89, 0x25, 0x7e, 0xbd, 0x3c, 0x8c, 0x27,
                    0x7d, 0x5a, 0xe1, 0x99,
                ]),
                dst_address: Address::from_array([
                    0xc1, 0x04, 0x99, 0x68, 0x2a, 0xa5, 0x64, 0x4d, 0x7e, 0xe3, 0x54, 0x7d, 0x0f,
                    0x07, 0x01, 0x38, 0x00, 0x2d, 0x60, 0xfd, 0xb8, 0x0b, 0xb9, 0xea, 0x76, 0x6c,
                    0x8c, 0x63, 0x75, 0x74,
                ]),
                res_address: Address::from_array([
                    0x5d, 0xd8, 0xee, 0x1d, 0xb7, 0xd7, 0xed, 0x52, 0x17, 0x73, 0x5e, 0x77, 0x66,
                    0x49, 0x54, 0x77, 0xfe, 0x03, 0xf5, 0xa0, 0xaa, 0xa1, 0x61, 0x71, 0x21, 0xae,
                    0xce, 0xe3, 0x1b, 0x99,
                ]),
                amount: Decimal::whole(123),
            },
            TxIntentType::Transfer,
        );
    }
    #[test]
    fn test_simple_transfer_with_multiple_locked_fees() {
        check_partial_decoding_with_type(
            &TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES,
br##"
1 of 5: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(1.2), )
2 of 5: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "withdraw" Tuple(Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv), Decimal(123), )
3 of 5: TakeFromWorktop Address(resource_loc1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxvq32hv) Decimal(123)
4 of 5: CallMethod Enum<0u8>(Address(account_loc1cyzfj6p254jy6lhr237s7pcp8qqz6c8ahq9mn6nkdjxxxat5pjq9xc), ) "try_deposit_or_abort" Tuple(Bucket(0u32), )
5 of 5: CallMethod Enum<0u8>(Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), ) "lock_fee" Tuple(Decimal(3.4), )
"##,
            &DetectedTxType::Transfer {
                fee: Some(Decimal::new(4600000000000000000u128)),
                src_address: Address::from_array([
                    0xc1, 0x18, 0x83, 0x46, 0x2f, 0x39, 0x79, 0x6d, 0xa8, 0x3f, 0x2f, 0x82, 0xca,
                    0xef, 0xa6, 0x79, 0xaa, 0xf1, 0xf1, 0x89, 0x25, 0x7e, 0xbd, 0x3c, 0x8c, 0x27,
                    0x7d, 0x5a, 0xe1, 0x99,
                ]),
                dst_address: Address::from_array([
                    0xc1, 0x04, 0x99, 0x68, 0x2a, 0xa5, 0x64, 0x4d, 0x7e, 0xe3, 0x54, 0x7d, 0x0f,
                    0x07, 0x01, 0x38, 0x00, 0x2d, 0x60, 0xfd, 0xb8, 0x0b, 0xb9, 0xea, 0x76, 0x6c,
                    0x8c, 0x63, 0x75, 0x74,
                ]),
                res_address: Address::from_array([
                    0x5d, 0xa6, 0x63, 0x18, 0xc6, 0x31, 0x8c, 0x61, 0xf5, 0xa6, 0x1b, 0x4c, 0x63,
                    0x18, 0xc6, 0x31, 0x8c, 0xf7, 0x94, 0xaa, 0x8d, 0x29, 0x5f, 0x14, 0xe6, 0x31,
                    0x8c, 0x63, 0x18, 0xc6,
                ]),
                amount: Decimal::whole(123),
            },
            TxIntentType::Transfer,
        );
    }
    #[test]
    fn test_values() {
        check_partial_decoding(
            &TX_VALUES,
br##"
1 of 4: TakeAllFromWorktop Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al)
2 of 4: CreateProofFromAuthZone Address(resource_loc1thvwu8dh6lk4y9mntemkvj25wllq8adq42skzufp4m8wxxue22t7al)
3 of 4: CallMethod Enum<0u8>(Address(component_loc1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve2jtvlp), ) "aliases" Tuple(Enum<0u8>(), Enum<0u8>(), Enum<1u8>("hello", ), Enum<1u8>("hello", ), Enum<0u8>("test", ), Enum<0u8>("test", ), Enum<1u8>("test123", ), Enum<1u8>("test123", ), Enum<0u8>(), Enum<1u8>("a", ), Enum<0u8>("b", ), Enum<1u8>("c", ), Bytes(deadbeef), Bytes(050aff), Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), <value>, ), Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), #123u64#, ), Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), #456u64#, ), Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), [031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f], ), Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), #1234567890u64#, ), Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), #1u64#, ), Array<Array>(Bytes(dead), Bytes(050aff), ), Array<Array>(Bytes(dead), Bytes(050aff), ), Array<Tuple>(Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), <value>, ), Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), #1u64#, ), ), Array<Tuple>(Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), <value>, ), Tuple(Address(resource_loc1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt98ehnq), #1u64#, ), ), Array<Enum>(Enum<1u8>("hello", ), ), Array<Enum>(Enum<1u8>(), Enum<0u8>(), ), Array<Map>(Map<U8, U16>(), ), Map<U8, U16>({1u8, 5u16}, ), )
4 of 4: CallMethod Enum<0u8>(Address(component_loc1cqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve2jtvlp), ) "custom_types" Tuple(Address(package_loc1p4r4955skdjq9swg8s5jguvcjvyj7tsxct87a9z6sw76cdfdmytuxt), Address(account_loc1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveyghrta), Address(consensusmanager_loc1scxxxxxxxxxxcnsmgrxxxxxxxxx000999665565xxxxxxxxxhwvhuz), Address(validator_loc1svzs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9l4e6kj), Address(accesscontroller_loc1cvvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveht3nek), Bucket(0u32), Proof(0u32), Expression(ENTIRE_WORKTOP), Blob(a710f0959d8e139b3c1ca74ac4fcb9a95ada2c82e7f563304c5487e0117095c0), Decimal(1.2), PreciseDecimal(1.2), <SomeId>, #12u64#, [031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f], {1111111111111111-1111111111111111-1111111111111111-1111111111111111}, ) "##,
            &DetectedTxType::Other(None),
        );
    }
    #[test]
    fn test_vault_freeze() {
        check_partial_decoding(
            &TX_VAULT_FREEZE,
br##"
1 of 8: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "freeze" Tuple(Tuple(1u32, ), )
2 of 8: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "freeze" Tuple(Tuple(2u32, ), )
3 of 8: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "freeze" Tuple(Tuple(4u32, ), )
4 of 8: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "freeze" Tuple(Tuple(7u32, ), )
5 of 8: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "unfreeze" Tuple(Tuple(1u32, ), )
6 of 8: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "unfreeze" Tuple(Tuple(2u32, ), )
7 of 8: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "unfreeze" Tuple(Tuple(4u32, ), )
8 of 8: CallDirectVaultMethod Address(internal_vault_loc1tqvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cveaha8d5) "unfreeze" Tuple(Tuple(7u32, ), )
"##,
            &DetectedTxType::Other(None),
        );
    }
}
