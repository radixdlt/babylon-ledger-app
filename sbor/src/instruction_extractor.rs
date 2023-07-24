// Process events received from decoder and extract data related to instructions

use crate::instruction::{to_instruction, InstructionInfo};
use crate::sbor_decoder::SborEvent;
use crate::type_info::{to_type_info, TypeInfo, TYPE_ENUM, TYPE_NONE, TYPE_TUPLE};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ExtractorPhase {
    Init,
    IntentShell,
    HeaderShell,
    InstructionShell,
    Instruction,
    InstructionParameter,
    Done,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum InstructionPhase {
    WaitForDiscriminator,
    WaitForParameterCount,
    Done,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ExtractorEvent {
    InstructionStart(InstructionInfo, u32, u32),
    ParameterStart(SborEvent, u32, TypeInfo),
    ParameterData(SborEvent),
    ParameterEnd(SborEvent),
    InstructionEnd,
    UnknownInstruction(u8),
    InvalidEventSequence,
    UnknownParameterType(u8),
}

pub trait InstructionHandler {
    fn handle(&mut self, event: ExtractorEvent);
}

pub struct InstructionExtractor {
    phase: ExtractorPhase,
    instruction_phase: InstructionPhase,
    parameter_count: u32,
    parameters_total: u32,
    instruction_total: u32,
    instruction_count: u32,
    discriminator: u8,
}

impl InstructionExtractor {
    pub const fn new() -> Self {
        Self {
            phase: ExtractorPhase::Init,
            instruction_phase: InstructionPhase::Done,
            parameter_count: 0,
            parameters_total: 0,
            instruction_total: 0,
            instruction_count: 0,
            discriminator: 0,
        }
    }

    pub fn reset(&mut self) {
        self.phase = ExtractorPhase::Init;
        self.instruction_phase = InstructionPhase::Done;
        self.parameter_count = 0;
        self.parameters_total = 0;
        self.instruction_total = 0;
        self.instruction_count = 0;
        self.discriminator = 0;
    }

    pub fn handle_event(&mut self, handler: &mut impl InstructionHandler, event: SborEvent) {
        match self.phase {
            ExtractorPhase::Init => {
                if Self::is_start(event, TYPE_ENUM, 0) {
                    self.phase = ExtractorPhase::IntentShell;
                }
            }
            ExtractorPhase::IntentShell => {
                if Self::is_start(event, TYPE_TUPLE, 1) {
                    self.phase = ExtractorPhase::HeaderShell;
                }
            }
            ExtractorPhase::HeaderShell => {
                if Self::is_end(event, TYPE_TUPLE, 1) {
                    self.phase = ExtractorPhase::InstructionShell;
                }
            }
            ExtractorPhase::InstructionShell => {
                match event {
                    SborEvent::Len(len) if self.instruction_total == 0 => {
                        self.instruction_total = len;
                        self.instruction_count = 0;
                    }
                    _ => {}
                };

                if Self::is_start(event, TYPE_ENUM, 2) {
                    self.phase = ExtractorPhase::Instruction;
                    self.start_instruction();
                }

                if Self::is_end(event, TYPE_NONE, 1) {
                    self.phase = ExtractorPhase::Done;
                }
            }
            ExtractorPhase::Instruction => {
                if !self.process_instruction_state(handler, event) {
                    return;
                }

                if Self::is_start(event, TYPE_NONE, 3) {
                    self.phase = ExtractorPhase::InstructionParameter;
                    self.process_parameter_start(handler, event);
                }

                if Self::is_end(event, TYPE_NONE, 2) {
                    self.phase = ExtractorPhase::InstructionShell;
                    self.instruction_count += 1;
                }
            }
            ExtractorPhase::InstructionParameter => {
                if Self::is_end(event, TYPE_NONE, 3) {
                    self.phase = ExtractorPhase::Instruction;
                    handler.handle(ExtractorEvent::ParameterEnd(event));
                    self.parameter_count += 1;

                    if self.parameter_count == self.parameters_total {
                        handler.handle(ExtractorEvent::InstructionEnd);
                    }
                } else {
                    handler.handle(ExtractorEvent::ParameterData(event));
                }
            }
            ExtractorPhase::Done => {}
        };
    }

    fn process_parameter_start(&mut self, handler: &mut impl InstructionHandler, event: SborEvent) {
        if let SborEvent::Start { type_id, .. } = event {
            match to_type_info(type_id) {
                Some(type_info) => {
                    handler.handle(ExtractorEvent::ParameterStart(
                        event,
                        self.parameter_count,
                        type_info,
                    ));
                }

                None => {
                    handler.handle(ExtractorEvent::UnknownParameterType(type_id));
                    self.phase = ExtractorPhase::Done;
                }
            }
        } else {
            // Something wrong with instruction encoding
            handler.handle(ExtractorEvent::InvalidEventSequence);
            self.phase = ExtractorPhase::Done;
        };
    }

    fn start_instruction(&mut self) {
        self.instruction_phase = InstructionPhase::WaitForDiscriminator;
        self.parameter_count = 0;
        self.parameters_total = 0;
    }

    fn process_instruction_state(
        &mut self,
        handler: &mut impl InstructionHandler,
        event: SborEvent,
    ) -> bool {
        match (self.instruction_phase, event) {
            (InstructionPhase::WaitForDiscriminator, SborEvent::Discriminator(discriminator)) => {
                self.discriminator = discriminator;
                self.instruction_phase = InstructionPhase::WaitForParameterCount;
            }
            (InstructionPhase::WaitForParameterCount, SborEvent::Len(len)) => {
                match to_instruction(self.discriminator) {
                    Some(info) => {
                        handler.handle(ExtractorEvent::InstructionStart(
                            info,
                            self.instruction_count,
                            self.instruction_total,
                        ));
                        self.parameters_total = len;
                        self.instruction_phase = InstructionPhase::Done;
                    }
                    None => {
                        // Unknown instruction
                        handler.handle(ExtractorEvent::UnknownInstruction(self.discriminator));
                        self.phase = ExtractorPhase::Done;
                        return false;
                    }
                }
            }
            _ => {}
        };
        true
    }

    fn is_start(event: SborEvent, expected_type: u8, nesting: u8) -> bool {
        match event {
            SborEvent::Start {
                type_id,
                nesting_level,
                fixed_size: _u8,
            } if (type_id == expected_type || expected_type == TYPE_NONE)
                && nesting_level == nesting =>
            {
                true
            }
            _ => false,
        }
    }

    fn is_end(event: SborEvent, expected_type: u8, nesting: u8) -> bool {
        match event {
            SborEvent::End {
                type_id,
                nesting_level,
            } if (type_id == expected_type || expected_type == TYPE_NONE)
                && nesting_level == nesting =>
            {
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;
    use crate::sbor_decoder::*;
    use crate::tx_intent_test_data::tests::*;

    struct InstructionProcessor {
        extractor: InstructionExtractor,
        handler: InstructionFormatter,
    }

    struct InstructionFormatter {
        instruction_count: usize,
        instructions: [Instruction; Self::SIZE],
    }

    impl InstructionProcessor {
        pub fn new() -> Self {
            Self {
                extractor: InstructionExtractor::new(),
                handler: InstructionFormatter::new(),
            }
        }
    }

    impl InstructionFormatter {
        pub const SIZE: usize = 50;
        pub fn new() -> Self {
            Self {
                instruction_count: 0,
                instructions: [Instruction::TakeFromWorktop; Self::SIZE],
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
            //            println!("{:?},", evt);
            self.extractor.handle_event(&mut self.handler, evt);
        }
    }

    impl InstructionHandler for InstructionFormatter {
        fn handle(&mut self, event: ExtractorEvent) {
            if let ExtractorEvent::InstructionStart(info, count, total) = event {
                self.instructions[self.instruction_count] = info.instruction;
                self.instruction_count += 1;
                println!(
                    "Instruction::{:?} {} of {},",
                    info.instruction,
                    count + 1,
                    total
                );
            } else {
                // println!("Event: {:?}", event);
            }
        }
    }

    const CHUNK_SIZE: usize = 113;

    fn check_partial_decoding(input: &[u8], expected_instructions: &[Instruction]) {
        let mut decoder = SborDecoder::new(true);
        let mut handler = InstructionProcessor::new();

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

        println!("Total {} instructions", handler.handler.instruction_count);
        handler.handler.verify(expected_instructions);
        println!();
    }

    #[test]
    pub fn test_access_rule() {
        check_partial_decoding(
            &TX_ACCESS_RULE,
            &[
                Instruction::CallRoleAssignmentMethod,
                Instruction::CallRoleAssignmentMethod,
                Instruction::CallRoleAssignmentMethod,
            ],
        );
    }

    #[test]
    pub fn test_call_function() {
        check_partial_decoding(&TX_CALL_FUNCTION, &[Instruction::CallFunction]);
    }

    #[test]
    pub fn test_call_method() {
        check_partial_decoding(
            &TX_CALL_METHOD,
            &[
                Instruction::CallMethod,
                Instruction::CallRoyaltyMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallRoleAssignmentMethod,
            ],
        );
    }

    #[test]
    pub fn test_create_access_controller() {
        check_partial_decoding(
            &TX_CREATE_ACCESS_CONTROLLER,
            &[Instruction::TakeAllFromWorktop, Instruction::CallFunction],
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
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
                Instruction::CallMetadataMethod,
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
                Instruction::CallMethod,
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
                Instruction::CallMethod,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_publish_package() {
        check_partial_decoding(
            &TX_PUBLISH_PACKAGE,
            &[Instruction::CallMethod, Instruction::CallFunction],
        );
    }

    #[test]
    pub fn test_resource_recall() {
        check_partial_decoding(&TX_RESOURCE_RECALL, &[Instruction::CallDirectVaultMethod]);
    }

    #[test]
    pub fn test_resource_worktop() {
        check_partial_decoding(
            &TX_RESOURCE_WORKTOP,
            &[
                Instruction::CallMethod,
                Instruction::TakeFromWorktop,
                Instruction::CallMethod,
                Instruction::AssertWorktopContainsAny,
                Instruction::AssertWorktopContains,
                Instruction::TakeAllFromWorktop,
                Instruction::ReturnToWorktop,
                Instruction::TakeNonFungiblesFromWorktop,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_royalty() {
        check_partial_decoding(
            &TX_ROYALTY,
            &[
                Instruction::CallRoyaltyMethod,
                Instruction::CallRoyaltyMethod,
                Instruction::CallMethod,
                Instruction::CallRoyaltyMethod,
            ],
        );
    }

    #[test]
    pub fn test_values() {
        check_partial_decoding(
            &TX_VALUES,
            &[
                Instruction::TakeAllFromWorktop,
                Instruction::CreateProofFromAuthZoneOfAll,
                Instruction::CallMethod,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_address_allocation() {
        check_partial_decoding(
            &TX_ADDRESS_ALLOCATION,
            &[
                Instruction::CallMethod,
                Instruction::AllocateGlobalAddress,
                Instruction::CallFunction,
                Instruction::CallFunction,
            ],
        );
    }

    #[test]
    pub fn test_create_non_fungible_resource_with_initial_supply() {
        check_partial_decoding(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
            &[
                Instruction::CallMethod,
                Instruction::CallFunction,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_create_validator() {
        check_partial_decoding(
            &TX_CREATE_VALIDATOR,
            &[
                Instruction::CallMethod,
                Instruction::TakeFromWorktop,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_resource_auth_zone() {
        check_partial_decoding(
            &TX_RESOURCE_AUTH_ZONE,
            &[
                Instruction::CallMethod,
                Instruction::TakeAllFromWorktop,
                Instruction::CreateProofFromBucketOfAmount,
                Instruction::CreateProofFromBucketOfNonFungibles,
                Instruction::CreateProofFromBucketOfAll,
                Instruction::CloneProof,
                Instruction::DropProof,
                Instruction::DropProof,
                Instruction::ClearAuthZone,
                Instruction::CallMethod,
                Instruction::PopFromAuthZone,
                Instruction::DropProof,
                Instruction::CallMethod,
                Instruction::CreateProofFromAuthZoneOfAmount,
                Instruction::CreateProofFromAuthZoneOfNonFungibles,
                Instruction::CreateProofFromAuthZoneOfAll,
                Instruction::ClearAuthZone,
                Instruction::ClearSignatureProofs,
                Instruction::DropAllProofs,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_simple_transfer() {
        check_partial_decoding(
            &TX_SIMPLE_TRANSFER,
            &[
                Instruction::CallMethod,
                Instruction::CallMethod,
                Instruction::TakeFromWorktop,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_simple_transfer_nft() {
        check_partial_decoding(
            &TX_SIMPLE_TRANSFER_NFT,
            &[
                Instruction::CallMethod,
                Instruction::CallMethod,
                Instruction::TakeFromWorktop,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_simple_transfer_nft_by_id() {
        check_partial_decoding(
            &TX_SIMPLE_TRANSFER_NFT_BY_ID,
            &[
                Instruction::CallMethod,
                Instruction::CallMethod,
                Instruction::TakeNonFungiblesFromWorktop,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_simple_transfer_with_multiple_locked_fees() {
        check_partial_decoding(
            &TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES,
            &[
                Instruction::CallMethod,
                Instruction::CallMethod,
                Instruction::TakeFromWorktop,
                Instruction::CallMethod,
                Instruction::CallMethod,
            ],
        );
    }

    #[test]
    pub fn test_vault_freeze() {
        check_partial_decoding(
            &TX_VAULT_FREEZE,
            &[
                Instruction::CallDirectVaultMethod,
                Instruction::CallDirectVaultMethod,
                Instruction::CallDirectVaultMethod,
                Instruction::CallDirectVaultMethod,
                Instruction::CallDirectVaultMethod,
                Instruction::CallDirectVaultMethod,
                Instruction::CallDirectVaultMethod,
                Instruction::CallDirectVaultMethod,
            ],
        );
    }

    #[test]
    pub fn test_hc_intent() {
        check_partial_decoding(&TX_HC_INTENT, &[Instruction::ClearAuthZone]);
    }
}
