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
use crate::print::tx_printer::Address;
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
    use crate::print::tx_printer::{Address, DetectedTxType, TxIntentPrinter};
    use crate::sbor_decoder::*;
    use crate::static_vec::AsSlice;
    use crate::tx_intent_test_data::tests::*;

    use super::*;

    const CHUNK_SIZE: usize = 255;
    const MAX_OUTPUT_SIZE: usize = 4096;

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
        ins_printer: InstructionPrinter<T>,
        tx_printer: TxIntentPrinter,
    }

    impl<T: Copy> SborEventHandler for InstructionProcessor<T> {
        fn handle(&mut self, evt: SborEvent) {
            let mut fanout = Fanout::new(&mut self.ins_printer, &mut self.tx_printer);
            self.extractor.handle_event(&mut fanout, evt);
        }
    }

    impl<T: Copy + AsSlice<u8>> InstructionProcessor<T> {
        pub fn new(tty: TTY<T>) -> Self {
            Self {
                extractor: InstructionExtractor::new(),
                ins_printer: InstructionPrinter::new(NetworkId::LocalNet, tty),
                tx_printer: TxIntentPrinter::new(NetworkId::LocalNet),
            }
        }

        pub fn set_intent_type(&mut self, intent_type: TxIntentType) {
            self.tx_printer.set_intent_type(intent_type);
        }

        pub fn verify(&self, expected: &[u8], expected_type: &DetectedTxType) {
            let mut cnt = 0;
            let output = from_utf8(self.ins_printer.get_tty().data.as_slice()).unwrap();

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

            let detected = self.tx_printer.get_detected_tx_type();

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
        check_partial_decoding_with_type(input, expected_text, expected_type, TxIntentType::General);
    }

    fn check_partial_decoding_with_type(input: &[u8], expected_text: &[u8], expected_type: &DetectedTxType, intent_type: TxIntentType) {
        let mut decoder = SborDecoder::new(true);
        let mut processor = InstructionProcessor::new(TestTTY::new_tty());

        processor.set_intent_type(intent_type);

        let mut start = 0;
        let mut end = min(input.len(), CHUNK_SIZE);

        while start < input.len() {
            match decoder.decode(&mut processor, &input[start..end]) {
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

            if end >= input.len() {
                end = input.len();
            }
        }

        processor.verify(expected_text, expected_type);
        println!();
    }

    #[test]
    pub fn test_access_rule() {
        check_partial_decoding(&TX_ACCESS_RULE,
br##"
1 of 1: SetMethodAccessRule Address(resource_loc1qxntya3nlyju8zsj8h86fz8ma5yl8smwjlg9tckkqvrsgdkay9) Tuple(Enum(0u8), "test", ) Enum(0u8)
"##, &DetectedTxType::Other(None));
    }

    #[test]
    pub fn test_call_function() {
        check_partial_decoding(&TX_CALL_FUNCTION,
br##"
1 of 1: CallFunction Address(package_loc1qr46xrzzzlgvqccwqptp9ujlqncamd6kexux05essnuqkl9yed) "BlueprintName" "f" Tuple("string", )
"##, &DetectedTxType::Other(None));
    }

    #[test]
    pub fn test_call_method() {
        check_partial_decoding(&TX_CALL_METHOD,
br##"
 1 of 1: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "complicated_method" Tuple(Decimal(1), PreciseDecimal(2), )
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_create_access_controller() {
        check_partial_decoding(&TX_CREATE_ACCESS_CONTROLLER,
br##"
1 of 2: TakeFromWorktop Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
2 of 2: CallFunction Address(package_loc1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqrqt2nzcw) "AccessController" "create_global" Tuple(Bucket(0u32), Tuple(Enum(0u8), Enum(0u8), Enum(0u8), ), Enum(0u8), )
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_create_account() {
        check_partial_decoding(&TX_CREATE_ACCOUNT,
br##"
1 of 1: CallFunction Address(package_loc1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzs3k5qxm) "Account" "create_global" Tuple(Enum(0u8), )
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_create_fungible_resource_with_initial_supply() {
        check_partial_decoding(&TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
br##"
1 of 3: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "lock_fee" Tuple(Decimal(10), )
2 of 3: CallFunction Address(package_loc1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs092ash) "FungibleResourceManager" "create_with_initial_supply" Tuple(18u8, Map<String, String>({"description", "A very innovative and important resource"}, {"name", "MyResource"}, {"symbol", "RSRC"}, ), Map<Enum, Tuple>({Enum(4u8), Tuple(Enum(0u8), Enum(1u8), )}, {Enum(5u8), Tuple(Enum(0u8), Enum(1u8), )}, ), Decimal(12), )
3 of 3: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "deposit_batch" Tuple(Expression(00), )
"##, &DetectedTxType::Other(Some(Decimal::whole(10))))
    }

    #[test]
    pub fn test_create_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(&TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
br##"
1 of 2: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "lock_fee" Tuple(Decimal(10), )
2 of 2: CallFunction Address(package_loc1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs092ash) "FungibleResourceManager" "create" Tuple(18u8, Map<String, String>({"description", "A very innovative and important resource"}, {"name", "MyResource"}, {"symbol", "RSRC"}, ), Map<Enum, Tuple>({Enum(4u8), Tuple(Enum(0u8), Enum(1u8), )}, {Enum(5u8), Tuple(Enum(0u8), Enum(1u8), )}, ), )
"##, &DetectedTxType::Other(Some(Decimal::whole(10))))
    }

    #[test]
    pub fn test_create_identity() {
        check_partial_decoding(&TX_CREATE_IDENTITY,
br##"
1 of 1: CallFunction Address(package_loc1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpq4edlwz) "Identity" "create" Tuple(Enum(0u8), )
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_create_non_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(&TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
br##"
1 of 2: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "lock_fee" Tuple(Decimal(10), )
2 of 2: CallFunction Address(package_loc1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs092ash) "NonFungibleResourceManager" "create" Tuple(Enum(1u8), Tuple(Tuple(Array<Enum>(), Array<Tuple>(), Array<Enum>(), ), Enum(0u8, 64u8), Array<String>(), ), Map<String, String>({"description", "A very innovative and important resource"}, {"name", "MyResource"}, ), Map<Enum, Tuple>({Enum(4u8), Tuple(Enum(0u8), Enum(1u8), )}, {Enum(5u8), Tuple(Enum(0u8), Enum(1u8), )}, ), )
"##, &DetectedTxType::Other(Some(Decimal::whole(10))))
    }

    #[test]
    pub fn test_metadata() {
        check_partial_decoding(&TX_METADATA,
br##"
1 of 20: SetMetadata Address(package_loc1qr46xrzzzlgvqccwqptp9ujlqncamd6kexux05essnuqkl9yed) "field_name" Enum(0u8, Enum(0u8, "v"))
2 of 20: SetMetadata Address(account_loc1qnkhnw506drsfhrjrzaw4aj2yrucezvj2w7jqqqm5zdssp8axp) "field_name" Enum(0u8, Enum(0u8, "v"))
3 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(0u8, "v"))
4 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(1u8, true))
5 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(2u8, 123u8))
6 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(3u8, 123u32))
7 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(4u8, 123u64))
8 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(5u8, -123i32))
9 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(6u8, -123i64))
10 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(7u8, Decimal(10.5)))
11 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(8u8, Address(account_loc1qnkhnw506drsfhrjrzaw4aj2yrucezvj2w7jqqqm5zdssp8axp)))
12 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(9u8, Enum(0u8, Bytes(0000000000000000000000000000000000000000000000000000000000000000ff))))
13 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(10u8, Tuple(Address(resource_loc1qxntya3nlyju8zsj8h86fz8ma5yl8smwjlg9tckkqvrsgdkay9), <some_string>, )))
14 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(11u8, <some_string>))
15 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(12u8, Tuple(10000i64, )))
16 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(0u8, Enum(13u8, "https://radixdlt.com"))
17 of 20: SetMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name" Enum(1u8, Array<Enum>(Enum(0u8, "some_string"), Enum(0u8, "another_string"), Enum(0u8, "yet_another_string"), ))
18 of 20: RemoveMetadata Address(package_loc1qr46xrzzzlgvqccwqptp9ujlqncamd6kexux05essnuqkl9yed) "field_name"
19 of 20: RemoveMetadata Address(account_loc1qnkhnw506drsfhrjrzaw4aj2yrucezvj2w7jqqqm5zdssp8axp) "field_name"
20 of 20: RemoveMetadata Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0) "field_name"
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_mint_fungible() {
        check_partial_decoding(&TX_MINT_FUNGIBLE,
br##"
1 of 4: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "lock_fee" Tuple(Decimal(10), )
2 of 4: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "create_proof_by_amount" Tuple(Address(resource_loc1q9g995jh0x0eaf3672kac6ruq9rr2jvwy4d82qw3cd3q3du4e4), Decimal(1), )
3 of 4: MintFungible Address(resource_loc1qtvh6xzsalqrfn57w7tsn6n5jhs6h7tvmzc5a6ysypsquz4ut5) Decimal(12)
4 of 4: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "deposit_batch" Tuple(Expression(00), )
"##, &DetectedTxType::Other(Some(Decimal::whole(10))))
    }

    #[test]
    pub fn test_mint_non_fungible() {
        check_partial_decoding(&TX_MINT_NON_FUNGIBLE,
br##"
1 of 4: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "lock_fee" Tuple(Decimal(10), )
2 of 4: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "create_proof_by_amount" Tuple(Address(resource_loc1q9g995jh0x0eaf3672kac6ruq9rr2jvwy4d82qw3cd3q3du4e4), Decimal(1), )
3 of 4: MintNonFungible Address(resource_loc1qtvh6xzsalqrfn57w7tsn6n5jhs6h7tvmzc5a6ysypsquz4ut5) Tuple(Map<NonFungibleLocalId, Tuple>({#12u64#, Tuple(Tuple(), )}, ), )
4 of 4: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "deposit_batch" Tuple(Expression(00), )
"##, &DetectedTxType::Other(Some(Decimal::whole(10))))
    }

    #[test]
    pub fn test_publish_package() {
        check_partial_decoding(&TX_PUBLISH_PACKAGE,
br##"
1 of 2: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "lock_fee" Tuple(Decimal(10), )
2 of 2: PublishPackage Blob(a710f0959d8e139b3c1ca74ac4fcb9a95ada2c82e7f563304c5487e0117095c0) Blob(554d6e3a49e90d3be279e7ff394a01d9603cc13aa701c11c1f291f6264aa5791) Map<String, Tuple>() Map<String, String>() Tuple(Map<Tuple, Enum>({Tuple(Enum(0u8), "claim_royalty", ), Enum(0u8, Enum(2u8, Enum(0u8, Enum(0u8, Enum(0u8, Tuple(Address(resource_loc1qgjfp996zpttrx4mcs2zlh5u6rym3q7f596qj9capczq3e98kv), #1u64#, ))))))}, {Tuple(Enum(0u8), "set_royalty_config", ), Enum(0u8, Enum(2u8, Enum(0u8, Enum(0u8, Enum(0u8, Tuple(Address(resource_loc1qgjfp996zpttrx4mcs2zlh5u6rym3q7f596qj9capczq3e98kv), #1u64#, ))))))}, {Tuple(Enum(2u8), "get", ), Enum(0u8, Enum(0u8))}, {Tuple(Enum(2u8), "set", ), Enum(0u8, Enum(2u8, Enum(0u8, Enum(0u8, Enum(0u8, Tuple(Address(resource_loc1qgjfp996zpttrx4mcs2zlh5u6rym3q7f596qj9capczq3e98kv), #1u64#, ))))))}, ), Map<String, Enum>(), Enum(1u8), Map<Tuple, Enum>({Tuple(Enum(0u8), "claim_royalty", ), Enum(2u8, Enum(0u8, Enum(0u8, Enum(0u8, Tuple(Address(resource_loc1qgjfp996zpttrx4mcs2zlh5u6rym3q7f596qj9capczq3e98kv), #1u64#, )))))}, {Tuple(Enum(0u8), "set_royalty_config", ), Enum(2u8, Enum(0u8, Enum(0u8, Enum(0u8, Tuple(Address(resource_loc1qgjfp996zpttrx4mcs2zlh5u6rym3q7f596qj9capczq3e98kv), #1u64#, )))))}, {Tuple(Enum(2u8), "get", ), Enum(2u8, Enum(0u8, Enum(0u8, Enum(0u8, Tuple(Address(resource_loc1qgjfp996zpttrx4mcs2zlh5u6rym3q7f596qj9capczq3e98kv), #1u64#, )))))}, {Tuple(Enum(2u8), "set", ), Enum(2u8, Enum(0u8, Enum(0u8, Enum(0u8, Tuple(Address(resource_loc1qgjfp996zpttrx4mcs2zlh5u6rym3q7f596qj9capczq3e98kv), #1u64#, )))))}, ), Map<String, Enum>(), Enum(1u8), )
"##, &DetectedTxType::Other(Some(Decimal::whole(10))))
    }

    #[test]
    pub fn test_resource_recall() {
        check_partial_decoding(&TX_RESOURCE_RECALL,
br##"
1 of 1: RecallResource Bytes(62b2c217e32e5b4754c08219ef16389761356eaccbf6f6bdbfa44d00000000) Decimal(1.2)
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_resource_worktop() {
        check_partial_decoding(&TX_RESOURCE_WORKTOP,
br##"
1 of 9: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "withdraw" Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), Decimal(5), )
2 of 9: TakeFromWorktopByAmount Decimal(2) Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
3 of 9: CallMethod Address(component_loc1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlkstnntd9) "buy_gumball" Tuple(Bucket(0u32), )
4 of 9: AssertWorktopContainsByAmount Decimal(3) Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
5 of 9: AssertWorktopContains Address(resource_loc1q2ym536cwvvf3cy9p777t4qjczqwf79hagp3wn93srvsxk57w0)
6 of 9: TakeFromWorktop Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
7 of 9: ReturnToWorktop Bucket(1u32)
8 of 9: TakeFromWorktopByIds Array<NonFungibleLocalId>(#1u64#, ) Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
9 of 9: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "deposit_batch" Tuple(Expression(00), )
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_royalty() {
        check_partial_decoding(&TX_ROYALTY,
br##"
1 of 4: SetPackageRoyaltyConfig Address(package_loc1qr46xrzzzlgvqccwqptp9ujlqncamd6kexux05essnuqkl9yed) Map<String, Tuple>({"Blueprint", Tuple(Map<String, U32>({"method", 1u32}, ), 0u32, )}, )
2 of 4: SetComponentRoyaltyConfig Address(account_loc1qnkhnw506drsfhrjrzaw4aj2yrucezvj2w7jqqqm5zdssp8axp) Tuple(Map<String, U32>({"method", 1u32}, ), 0u32, )
3 of 4: ClaimPackageRoyalty Address(package_loc1qr46xrzzzlgvqccwqptp9ujlqncamd6kexux05essnuqkl9yed)
4 of 4: ClaimComponentRoyalty Address(account_loc1qnkhnw506drsfhrjrzaw4aj2yrucezvj2w7jqqqm5zdssp8axp)
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_values() {
        check_partial_decoding(&TX_VALUES,
br##"
1 of 4: TakeFromWorktop Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
2 of 4: CreateProofFromAuthZone Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
3 of 4: CallMethod Address(component_loc1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlkstnntd9) "aliases" Tuple(Enum(0u8), Enum(0u8), Enum(1u8, "hello"), Enum(1u8, "hello"), Enum(0u8, "test"), Enum(0u8, "test"), Enum(1u8, "test123"), Enum(1u8, "test123"), Enum(0u8), Enum(1u8, "a"), Enum(0u8, "b"), Enum(1u8, "c"), Bytes(deadbeef), Bytes(050aff), Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), <value>, ), Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), #123u64#, ), Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), #456u64#, ), Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), [031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f], ), Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), #1234567890u64#, ), Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), #1u64#, ), Array<Array>(Bytes(dead), Bytes(050aff), ), Array<Array>(Bytes(dead), Bytes(050aff), ), Array<Tuple>(Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), <value>, ), Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), #1u64#, ), ), Array<Tuple>(Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), <value>, ), Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), #1u64#, ), ), )
4 of 4: CallMethod Address(component_loc1qd8djmepmq7hxqaakt9rl3hkce532px42s8eh4qmqlkstnntd9) "custom_types" Tuple(Address(package_loc1qr46xrzzzlgvqccwqptp9ujlqncamd6kexux05essnuqkl9yed), Address(account_loc1qnkhnw506drsfhrjrzaw4aj2yrucezvj2w7jqqqm5zdssp8axp), Address(epochmanager_loc1q5qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqsr3qky), Address(clock_loc1quqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqp0cxlt), Address(validator_loc1qcqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqsaz50gm), Address(accesscontroller_loc1p5qszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqsf5nrka), Bucket(0u32), Proof(1u32), Expression(00), Blob(a710f0959d8e139b3c1ca74ac4fcb9a95ada2c82e7f563304c5487e0117095c0), Decimal(1.2), PreciseDecimal(1.2), <SomeId>, #12u64#, [031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f], {43968a72-5954-45da-9678-8659dd399faa}, )
"##, &DetectedTxType::Other(None))
    }

    #[test]
    pub fn test_simple_transfer() {
        check_partial_decoding_with_type(&TX_SIMPLE_TRANSFER,
                               br##"
1 of 4: CallMethod Address(account_loc1p9j7zjlzzxfpc9w8dewfavme6tyl3lzl2sevfwtk0jlqu600lh) "lock_fee" Tuple(Decimal(10), )
2 of 4: CallMethod Address(account_loc1p9j7zjlzzxfpc9w8dewfavme6tyl3lzl2sevfwtk0jlqu600lh) "withdraw" Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), Decimal(123), )
3 of 4: TakeFromWorktopByAmount Decimal(123) Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
4 of 4: CallMethod Address(account_loc1pxhyn798qaehnxz6qwyj6jx5qm296j4j5uuqh4av7h5saywq8x) "deposit" Tuple(Bucket(0u32), )
"##, &DetectedTxType::Transfer {
                fee: Some(Decimal::whole(10)),
                src_address: Address::from_array([0x09, 0x65, 0xe1, 0x4b, 0xe2, 0x11, 0x92, 0x1c, 0x15, 0xc7, 0x6e, 0x5c, 0x9e, 0xb3, 0x79, 0xd2, 0xc9, 0xf8, 0xfc, 0x5f, 0x54, 0x32, 0xc4, 0xb9, 0x76, 0x7c, 0xbe,]),
                dst_address: Address::from_array([0x09, 0xae, 0x49, 0xf8, 0xa7, 0x07, 0x73, 0x79, 0x98, 0x5a, 0x03, 0x89, 0x2d, 0x48, 0xd4, 0x06, 0xd4, 0x5d, 0x4a, 0xb2, 0xa7, 0x38, 0x0b, 0xd7, 0xac, 0xf5, 0xe9,]),
                res_address: Address::from_array([0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,]),
                amount: Decimal::whole(123)
            }, TxIntentType::Transfer)
    }

    #[test]
    pub fn test_simple_transfer_nft() {
        check_partial_decoding_with_type(&TX_SIMPLE_TRANSFER_NFT,
                                         br##"
1 of 4: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "lock_fee" Tuple(Decimal(10), )
2 of 4: CallMethod Address(account_loc1qjy5fakwygc45fkyhyxxulsf5zfae0ycez0x05et9hqsshmat9) "withdraw_non_fungibles" Tuple(Address(resource_loc1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpwwehl), Array<NonFungibleLocalId>(#1u64#, #2u64#, ), )
3 of 4: TakeFromWorktopByAmount Decimal(2) Address(resource_loc1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpwwehl)
4 of 4: CallMethod Address(account_loc1pxhyn798qaehnxz6qwyj6jx5qm296j4j5uuqh4av7h5saywq8x) "deposit" Tuple(Bucket(0u32), )
"##, &DetectedTxType::Transfer {
                fee: Some(Decimal::whole(10)),
                src_address: Address::from_array([0x04, 0x89, 0x44, 0xf6, 0xce, 0x22, 0x31, 0x5a, 0x26, 0xc4, 0xb9, 0x0c, 0x6e, 0x7e, 0x09, 0xa0, 0x93, 0xdc, 0xbc, 0x98, 0xc8, 0x9e, 0x67, 0xd3, 0x2b, 0x2d, 0xc1,]),
                dst_address: Address::from_array([0x09, 0xae, 0x49, 0xf8, 0xa7, 0x07, 0x73, 0x79, 0x98, 0x5a, 0x03, 0x89, 0x2d, 0x48, 0xd4, 0x06, 0xd4, 0x5d, 0x4a, 0xb2, 0xa7, 0x38, 0x0b, 0xd7, 0xac, 0xf5, 0xe9,]),
                res_address: Address::from_array([0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,]),
                amount: Decimal::whole(2)
            }, TxIntentType::Transfer)
    }




    #[test]
    pub fn test_simple_transfer_with_multiple_locked_fees() {
        check_partial_decoding_with_type(&TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES,
                               br##"
1 of 5: CallMethod Address(account_loc1p9j7zjlzzxfpc9w8dewfavme6tyl3lzl2sevfwtk0jlqu600lh) "lock_fee" Tuple(Decimal(1.2), )
2 of 5: CallMethod Address(account_loc1p9j7zjlzzxfpc9w8dewfavme6tyl3lzl2sevfwtk0jlqu600lh) "withdraw" Tuple(Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q), Decimal(123), )
3 of 5: TakeFromWorktopByAmount Decimal(123) Address(resource_loc1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq7qej9q)
4 of 5: CallMethod Address(account_loc1pxhyn798qaehnxz6qwyj6jx5qm296j4j5uuqh4av7h5saywq8x) "deposit" Tuple(Bucket(0u32), )
5 of 5: CallMethod Address(account_loc1p9j7zjlzzxfpc9w8dewfavme6tyl3lzl2sevfwtk0jlqu600lh) "lock_fee" Tuple(Decimal(3.4), )
"##, &DetectedTxType::Transfer {
                fee: Some(Decimal::new(4600000000000000000u128)),
                src_address: Address::from_array([0x09, 0x65, 0xe1, 0x4b, 0xe2, 0x11, 0x92, 0x1c, 0x15, 0xc7, 0x6e, 0x5c, 0x9e, 0xb3, 0x79, 0xd2, 0xc9, 0xf8, 0xfc, 0x5f, 0x54, 0x32, 0xc4, 0xb9, 0x76, 0x7c, 0xbe,]),
                dst_address: Address::from_array([0x09, 0xae, 0x49, 0xf8, 0xa7, 0x07, 0x73, 0x79, 0x98, 0x5a, 0x03, 0x89, 0x2d, 0x48, 0xd4, 0x06, 0xd4, 0x5d, 0x4a, 0xb2, 0xa7, 0x38, 0x0b, 0xd7, 0xac, 0xf5, 0xe9,]),
                res_address: Address::from_array([0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,]),
                amount: Decimal::whole(123)
            }, TxIntentType::Transfer)
    }
}
