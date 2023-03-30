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

pub struct InstructionPrinter<'a> {
    active_instruction: Option<InstructionInfo>,
    state: ParameterPrinterState<'a>,
}

impl InstructionHandler for InstructionPrinter<'_> {
    fn handle(&mut self, event: ExtractorEvent) {
        match event {
            ExtractorEvent::InstructionStart(info) => self.start_instruction(info),
            ExtractorEvent::ParameterStart(event, ..) => {
                self.parameter_start(event)
            }
            ExtractorEvent::ParameterData(data) => self.parameter_data(data),
            ExtractorEvent::ParameterEnd(event, ..) => self.parameter_end(event),
            ExtractorEvent::InstructionEnd => self.instruction_end(),
            // Error conditions
            ExtractorEvent::UnknownInstruction(..)
            | ExtractorEvent::InvalidEventSequence
            | ExtractorEvent::UnknownParameterType(..) => self.handle_error(),
        }
    }
}

impl<'a> InstructionPrinter<'a> {
    pub fn new(tty: &'a mut dyn TTY, network_id: NetworkId) -> Self {
        Self {
            active_instruction: None,
            state: ParameterPrinterState::new(network_id, tty),
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.state.set_network(network_id);
    }

    pub fn handle_error(&mut self) {
        self.state.tty.start();
        self.state.tty.print_text(b"Unable to decode transaction intent. Either, input is invalid or application is outdated.");
        self.state.tty.end();
    }

    pub fn start_instruction(&mut self, info: InstructionInfo) {
        self.active_instruction = Some(info);
        self.state.tty.start();
        self.state.tty.print_text(info.name);
        self.state.tty.print_space();
    }

    pub fn instruction_end(&mut self) {
        if let Some(..) = self.active_instruction {
            self.state.tty.end();
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
                    self.get_printer().subcomponent_start(&mut self.state);
                }

                self.state.nesting_level = nesting_level;
                self.state.stack.push(ValueState::new(type_id));
                self.get_printer().start(&mut self.state);
            }
            SborEvent::ElementType { kind, type_id } => {
                match kind {
                    SubTypeKind::Key => self.active_value_state().key_type_id = type_id,
                    SubTypeKind::Value => self.active_value_state().element_type_id = type_id,
                    SubTypeKind::Element => self.active_value_state().element_type_id = type_id,
                }
                self.get_printer()
                    .handle_data(&mut self.state, source_event);
            }
            SborEvent::Discriminator(discriminator) => {
                self.active_value_state().key_type_id = discriminator;
                self.get_printer()
                    .handle_data(&mut self.state, source_event);
            }
            SborEvent::End {
                type_id: _,
                nesting_level,
            } => {
                self.get_printer().end(&mut self.state);
                self.state.nesting_level = nesting_level;
                self.state.stack.pop().expect("Stack can't be empty");

                if self.state.stack.is_not_empty() {
                    self.get_printer().subcomponent_end(&mut self.state);
                } else {
                    self.state.tty.print_space();
                }

                self.state.data.clear();
            }
            _ => {
                self.get_printer()
                    .handle_data(&mut self.state, source_event);
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

    fn get_printer(&mut self) -> &'static dyn ParameterPrinter {
        get_printer_for_discriminator(self.active_value_state().main_type_id)
    }
}

pub fn get_printer_for_discriminator(discriminator: u8) -> &'static dyn ParameterPrinter {
    match discriminator {
        // Generic types
        TYPE_BOOL => &BOOL_PARAMETER_PRINTER,
        TYPE_I8 => &I8_PARAMETER_PRINTER,
        TYPE_I16 => &I16_PARAMETER_PRINTER,
        TYPE_I32 => &I32_PARAMETER_PRINTER,
        TYPE_I64 => &I64_PARAMETER_PRINTER,
        TYPE_I128 => &I128_PARAMETER_PRINTER,
        TYPE_U8 => &U8_PARAMETER_PRINTER,
        TYPE_U16 => &U16_PARAMETER_PRINTER,
        TYPE_U32 => &U32_PARAMETER_PRINTER,
        TYPE_U64 => &U64_PARAMETER_PRINTER,
        TYPE_U128 => &U128_PARAMETER_PRINTER,
        TYPE_STRING => &STRING_PARAMETER_PRINTER,
        TYPE_ARRAY => &ARRAY_PARAMETER_PRINTER,
        TYPE_TUPLE => &TUPLE_PARAMETER_PRINTER,
        TYPE_ENUM => &ENUM_PARAMETER_PRINTER,
        TYPE_MAP => &MAP_PARAMETER_PRINTER,
        // Custom types
        TYPE_ADDRESS => &ADDRESS_PARAMETER_PRINTER,
        TYPE_BUCKET => &BUCKET_PARAMETER_PRINTER,
        TYPE_PROOF => &PROOF_PARAMETER_PRINTER,
        TYPE_EXPRESSION => &EXPRESSION_PARAMETER_PRINTER,
        TYPE_BLOB => &BLOB_PARAMETER_PRINTER,
        TYPE_DECIMAL => &DECIMAL_PARAMETER_PRINTER,
        TYPE_PRECISE_DECIMAL => &PRECISE_DECIMAL_PARAMETER_PRINTER,
        TYPE_NON_FUNGIBLE_LOCAL_ID => &NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER,
        _ => &IGNORED_PARAMETER_PRINTER,
    }
}

#[cfg(test)]
mod tests {
    use core::cmp::min;

    use crate::bech32::network::NetworkId;
    use crate::instruction::Instruction;
    use crate::instruction_extractor::*;
    use crate::print::tty::TTY;
    use crate::sbor_decoder::*;
    use crate::tx_intent_test_data::tests::*;

    use super::*;

    #[derive(Copy, Clone)]
    struct TestPrinter {}

    impl TTY for TestPrinter {
        fn print_byte(&mut self, byte: u8) {
            print!("{}", char::from(byte));
        }

        fn start(&mut self) {
        }

        fn end(&mut self) {
            println!();
        }
    }

    struct InstructionProcessor<'a> {
        extractor: InstructionExtractor,
        handler: InstructionFormatter<'a>,
    }

    const SIZE: usize = 20;

    struct InstructionFormatter<'a> {
        instruction_count: usize,
        instructions: [Instruction; SIZE],
        printer: InstructionPrinter<'a>,
    }

    impl<'a> InstructionProcessor<'a> {
        pub fn new(tty: &'a mut dyn TTY) -> Self {
            Self {
                extractor: InstructionExtractor::new(),
                handler: InstructionFormatter::new(tty),
            }
        }
    }

    impl<'a> InstructionFormatter<'a> {
        pub fn new(tty: &'a mut dyn TTY) -> Self {
            Self {
                instruction_count: 0,
                instructions: [Instruction::TakeFromWorktop; SIZE],
                printer: InstructionPrinter::new(tty, NetworkId::Simulator),
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

    impl<'a> SborEventHandler for InstructionProcessor<'a> {
        fn handle(&mut self, evt: SborEvent) {
            self.extractor.handle_event(&mut self.handler, evt);
        }
    }

    impl InstructionHandler for InstructionFormatter<'_> {
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
        let mut tty = TestPrinter {};
        let mut decoder = SborDecoder::new(true);
        let mut handler = InstructionProcessor::new(&mut tty);

        let mut start = 0;
        let mut end = min(input.len(), CHUNK_SIZE);

        while start < input.len() {
            match decoder.decode(&mut handler, &input[start..end]) {
                Ok(outcome) => {
                    if end - start == CHUNK_SIZE {
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
    pub fn test_assert_access_rule() {
        check_partial_decoding(
            &TX_ASSERT_ACCESS_RULE,
            &[Instruction::CallMethod, Instruction::AssertAccessRule],
        );
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
