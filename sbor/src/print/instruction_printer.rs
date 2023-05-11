use crate::bech32::network::*;
use crate::instruction::InstructionInfo;
use crate::instruction_extractor::{ExtractorEvent, InstructionHandler};
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
use crate::sbor_decoder::{SborEvent, SubTypeKind};
use crate::type_info::*;

pub struct InstructionPrinter {
    active_instruction: Option<InstructionInfo>,
    pub state: ParameterPrinterState,
}

impl InstructionHandler for InstructionPrinter {
    fn handle(&mut self, event: ExtractorEvent) {
        match event {
            ExtractorEvent::InstructionStart(info) => self.start_instruction(info),
            ExtractorEvent::ParameterStart(event, ..) => self.parameter_start(event),
            ExtractorEvent::ParameterData(data) => self.parameter_data(data),
            ExtractorEvent::ParameterEnd(event, ..) => self.parameter_end(event),
            ExtractorEvent::InstructionEnd => self.instruction_end(),
            // Error conditions
            ExtractorEvent::UnknownInstruction(..)
            | ExtractorEvent::InvalidEventSequence
            | ExtractorEvent::UnknownParameterType(..) => self.handle_error(),
        };
    }
}

impl InstructionPrinter {
    pub fn new(network_id: NetworkId, tty: TTY) -> Self {
        Self {
            active_instruction: None,
            state: ParameterPrinterState::new(network_id, tty),
        }
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

    pub fn set_tty(&mut self, tty: TTY) {
        self.state.set_tty(tty);
    }

    pub fn handle_error(&mut self) {
        self.state.start();
        self.state.print_text(b"Unable to decode transaction intent. Either, input is invalid or application is outdated.");
        self.state.end();
    }

    pub fn start_instruction(&mut self, info: InstructionInfo) {
        self.active_instruction = Some(info);
        self.state.start();
        self.state.print_text(info.name);
        self.state.print_space();
    }

    pub fn instruction_end(&mut self) {
        if let Some(..) = self.active_instruction {
            self.state.end();
        }

        self.active_instruction = None;
    }

    pub fn parameter_start(&mut self, event: SborEvent) {
        self.parameter_data(event);
    }

    pub fn parameter_data(&mut self, source_event: SborEvent) {
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

    pub fn parameter_end(&mut self, event: SborEvent) {
        self.parameter_data(event);
        self.state.reset();
    }
}

struct Dispatcher;

// Workaround for not working vtables
impl Dispatcher {
    pub fn handle_data(state: &mut ParameterPrinterState, event: SborEvent) {
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

    pub fn start(state: &mut ParameterPrinterState) {
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

    pub fn end(state: &mut ParameterPrinterState) {
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

    pub fn subcomponent_start(state: &mut ParameterPrinterState) {
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

    pub fn subcomponent_end(state: &mut ParameterPrinterState) {
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
    use crate::instruction::Instruction;
    use crate::instruction_extractor::*;
    use crate::print::tty::TTY;
    use crate::sbor_decoder::*;
    use crate::tx_intent_test_data::tests::*;

    use super::*;

    #[derive(Clone)]
    struct TestPrinter {}

    impl TestPrinter {
        pub fn new() -> TTY {
            TTY {
                show_message: Self::show_message,
            }
        }
        fn show_message(message: &[u8]) {
            println!("{}", from_utf8(message).unwrap());
        }
    }

    struct InstructionProcessor {
        extractor: InstructionExtractor,
        handler: InstructionFormatter,
    }

    const SIZE: usize = 20;

    struct InstructionFormatter {
        instruction_count: usize,
        instructions: [Instruction; SIZE],
        printer: InstructionPrinter,
    }

    impl InstructionProcessor {
        pub fn new(tty: TTY) -> Self {
            Self {
                extractor: InstructionExtractor::new(),
                handler: InstructionFormatter::new(tty),
            }
        }
    }

    impl InstructionFormatter {
        pub fn new(tty: TTY) -> Self {
            Self {
                instruction_count: 0,
                instructions: [Instruction::TakeFromWorktop; SIZE],
                printer: InstructionPrinter::new(NetworkId::Simulator, tty),
            }
        }

        pub fn verify(&self, expected: &[Instruction]) {
            assert_eq!(self.instruction_count, expected.len());
            let mut cnt = 0;
            self.instructions[..self.instruction_count]
                .iter()
                .zip(expected)
                .all(|(a, b)| {
                    assert_eq!(*a, *b, "Elements are not equal at index {}", cnt);
                    cnt += 1;
                    true
                });
        }
    }

    impl SborEventHandler for InstructionProcessor {
        fn handle(&mut self, evt: SborEvent) {
            self.extractor.handle_event(&mut self.handler, evt);
        }
    }

    impl InstructionHandler for InstructionFormatter {
        fn handle(&mut self, event: ExtractorEvent) {
            if let ExtractorEvent::InstructionStart(info) = event {
                self.instructions[self.instruction_count] = info.instruction;
                self.instruction_count += 1;
                //println!("Instruction::{:?},", info.instruction);
            }

            self.printer.handle(event);

            //println!("Event: {:?}", event);
        }
    }

    const CHUNK_SIZE: usize = 255;

    fn check_partial_decoding(input: &[u8], expected_instructions: &[Instruction]) {
        let mut decoder = SborDecoder::new(true);
        let mut handler = InstructionProcessor::new(TestPrinter::new());

        let mut start = 0;
        let mut end = min(input.len(), CHUNK_SIZE);

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

            if end >= input.len() {
                end = input.len();
            }
            // println!("start, end, len = {}, {}, {}", start, end, input.len());
        }

        //println!("Total {} instructions", handler.handler.instruction_count);
        handler.handler.verify(expected_instructions);
        println!();
    }

    #[test]
    pub fn test_access_rule() {
        check_partial_decoding(&TX_ACCESS_RULE, &[Instruction::SetMethodAccessRule]);
    }

    #[test]
    pub fn test_call_function() {
        check_partial_decoding(&TX_CALL_FUNCTION, &[Instruction::CallFunction]);
    }

    #[test]
    pub fn test_call_method() {
        check_partial_decoding(&TX_CALL_METHOD, &[Instruction::CallMethod]);
    }

    #[test]
    pub fn test_create_access_controller() {
        check_partial_decoding(
            &TX_CREATE_ACCESS_CONTROLLER,
            &[Instruction::TakeFromWorktop, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_create_account() {
        check_partial_decoding(
            &TX_CREATE_ACCOUNT,
            &[Instruction::CallFunction, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_create_fungible_resource_with_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
            &[
                Instruction::CallMethod,
                Instruction::CallFunction,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_create_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
            &[Instruction::CallMethod, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_create_identity() {
        check_partial_decoding(
            &TX_CREATE_IDENTITY,
            &[Instruction::CallFunction, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_create_non_fungible_resource_with_no_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
            &[Instruction::CallMethod, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_metadata() {
        check_partial_decoding(
            &TX_METADATA,
            &[
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::SetMetadata,
                Instruction::RemoveMetadata,
                Instruction::RemoveMetadata,
                Instruction::RemoveMetadata,
            ],
        );
    }

    #[test]
    pub fn test_mint_fungible() {
        check_partial_decoding(
            &TX_MINT_FUNGIBLE,
            &[
                Instruction::CallMethod,
                Instruction::CallMethod,
                Instruction::MintFungible,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_mint_non_fungible() {
        check_partial_decoding(
            &TX_MINT_NON_FUNGIBLE,
            &[
                Instruction::CallMethod,
                Instruction::CallMethod,
                Instruction::MintNonFungible,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_publish_package() {
        check_partial_decoding(
            &TX_PUBLISH_PACKAGE,
            &[Instruction::CallMethod, Instruction::PublishPackage],
        );
    }

    #[test]
    pub fn test_resource_recall() {
        check_partial_decoding(&TX_RESOURCE_RECALL, &[Instruction::RecallResource]);
    }

    #[test]
    pub fn test_resource_worktop() {
        check_partial_decoding(
            &TX_RESOURCE_WORKTOP,
            &[
                Instruction::CallMethod,
                Instruction::TakeFromWorktopByAmount,
                Instruction::CallMethod,
                Instruction::AssertWorktopContainsByAmount,
                Instruction::AssertWorktopContains,
                Instruction::TakeFromWorktop,
                Instruction::ReturnToWorktop,
                Instruction::TakeFromWorktopByIds,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_royalty() {
        check_partial_decoding(
            &TX_ROYALTY,
            &[
                Instruction::SetPackageRoyaltyConfig,
                Instruction::SetComponentRoyaltyConfig,
                Instruction::ClaimPackageRoyalty,
                Instruction::ClaimComponentRoyalty,
            ],
        );
    }

    #[test]
    pub fn test_values() {
        check_partial_decoding(
            &TX_VALUES,
            &[
                Instruction::TakeFromWorktop,
                Instruction::CreateProofFromAuthZone,
                Instruction::CallMethod,
                Instruction::CallMethod,
            ],
        );
    }
}
