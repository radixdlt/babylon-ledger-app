/// Instructions recognized by instruction extractor
/// Keep in sync with
/// https://raw.githubusercontent.com/radixdlt/radixdlt-scrypto/develop/transaction/src/model/instruction.rs
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    TakeAllFromWorktop = INSTRUCTION_TAKE_ALL_FROM_WORKTOP_DISCRIMINATOR,
    TakeFromWorktop = INSTRUCTION_TAKE_FROM_WORKTOP_DISCRIMINATOR,
    TakeNonFungiblesFromWorktop = INSTRUCTION_TAKE_NON_FUNGIBLES_FROM_WORKTOP_DISCRIMINATOR,
    ReturnToWorktop = INSTRUCTION_RETURN_TO_WORKTOP_DISCRIMINATOR,
    AssertWorktopContainsAny = INSTRUCTION_ASSERT_WORKTOP_CONTAINS_ANY_DISCRIMINATOR,
    AssertWorktopContains = INSTRUCTION_ASSERT_WORKTOP_CONTAINS_DISCRIMINATOR,
    AssertWorktopContainsNonFungibles =
        INSTRUCTION_ASSERT_WORKTOP_CONTAINS_NON_FUNGIBLES_DISCRIMINATOR,
    PopFromAuthZone = INSTRUCTION_POP_FROM_AUTH_ZONE_DISCRIMINATOR,
    PushToAuthZone = INSTRUCTION_PUSH_TO_AUTH_ZONE_DISCRIMINATOR,
    CreateProofFromAuthZoneOfAmount =
        INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_AMOUNT_DISCRIMINATOR,
    CreateProofFromAuthZoneOfNonFungibles =
        INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_NON_FUNGIBLES_DISCRIMINATOR,
    CreateProofFromAuthZoneOfAll = INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_ALL_DISCRIMINATOR,
    DropAuthZoneProofs = INSTRUCTION_DROP_AUTH_ZONE_PROOFS_DISCRIMINATOR,
    DropAuthZoneRegularProofs = INSTRUCTION_DROP_AUTH_ZONE_REGULAR_PROOFS_DISCRIMINATOR,
    DropAuthZoneSignatureProofs = INSTRUCTION_DROP_AUTH_ZONE_SIGNATURE_PROOFS_DISCRIMINATOR,
    CreateProofFromBucketOfAmount = INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_AMOUNT_DISCRIMINATOR,
    CreateProofFromBucketOfNonFungibles =
        INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_NON_FUNGIBLES_DISCRIMINATOR,
    CreateProofFromBucketOfAll = INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_ALL_DISCRIMINATOR,
    BurnResource = INSTRUCTION_BURN_RESOURCE_DISCRIMINATOR,
    CloneProof = INSTRUCTION_CLONE_PROOF_DISCRIMINATOR,
    DropProof = INSTRUCTION_DROP_PROOF_DISCRIMINATOR,
    CallFunction = INSTRUCTION_CALL_FUNCTION_DISCRIMINATOR,
    CallMethod = INSTRUCTION_CALL_METHOD_DISCRIMINATOR,
    CallRoyaltyMethod = INSTRUCTION_CALL_ROYALTY_METHOD_DISCRIMINATOR,
    CallMetadataMethod = INSTRUCTION_CALL_METADATA_METHOD_DISCRIMINATOR,
    CallRoleAssignmentMethod = INSTRUCTION_CALL_ROLE_ASSIGNMENT_METHOD_DISCRIMINATOR,
    CallDirectVaultMethod = INSTRUCTION_CALL_DIRECT_VAULT_METHOD_DISCRIMINATOR,
    DropNamedProofs = INSTRUCTION_DROP_NAMED_PROOFS_DISCRIMINATOR,
    DropAllProofs = INSTRUCTION_DROP_ALL_PROOFS_DISCRIMINATOR,
    AllocateGlobalAddress = INSTRUCTION_ALLOCATE_GLOBAL_ADDRESS_DISCRIMINATOR,
}

#[derive(Copy, Clone, Debug)]
pub struct InstructionInfo {
    pub instruction: Instruction,
    pub name: &'static [u8],
}

pub fn to_instruction(input: u8) -> Option<InstructionInfo> {
    match input {
        INSTRUCTION_TAKE_ALL_FROM_WORKTOP_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::TakeAllFromWorktop,
            name: b"TakeAllFromWorktop",
        }),
        INSTRUCTION_TAKE_FROM_WORKTOP_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktop,
            name: b"TakeFromWorktop",
        }),
        INSTRUCTION_TAKE_NON_FUNGIBLES_FROM_WORKTOP_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::TakeNonFungiblesFromWorktop,
            name: b"TakeNonFungiblesFromWorktop",
        }),
        INSTRUCTION_RETURN_TO_WORKTOP_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::ReturnToWorktop,
            name: b"ReturnToWorktop",
        }),
        INSTRUCTION_ASSERT_WORKTOP_CONTAINS_ANY_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContainsAny,
            name: b"AssertWorktopContainsAny",
        }),
        INSTRUCTION_ASSERT_WORKTOP_CONTAINS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContains,
            name: b"AssertWorktopContains",
        }),
        INSTRUCTION_ASSERT_WORKTOP_CONTAINS_NON_FUNGIBLES_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContainsNonFungibles,
            name: b"AssertWorktopContainsNonFungibles",
        }),
        INSTRUCTION_POP_FROM_AUTH_ZONE_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::PopFromAuthZone,
            name: b"PopFromAuthZone",
        }),
        INSTRUCTION_PUSH_TO_AUTH_ZONE_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::PushToAuthZone,
            name: b"PushToAuthZone",
        }),
        INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_AMOUNT_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZoneOfAmount,
            name: b"CreateProofFromAuthZoneOfAmount",
        }),
        INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_NON_FUNGIBLES_DISCRIMINATOR => {
            Some(InstructionInfo {
                instruction: Instruction::CreateProofFromAuthZoneOfNonFungibles,
                name: b"CreateProofFromAuthZoneOfNonFungibles",
            })
        }
        INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_ALL_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZoneOfAll,
            name: b"CreateProofFromAuthZoneOfAll",
        }),
        INSTRUCTION_DROP_AUTH_ZONE_PROOFS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::DropAuthZoneProofs,
            name: b"DropAuthZoneProofs",
        }),
        INSTRUCTION_DROP_AUTH_ZONE_REGULAR_PROOFS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::DropAuthZoneRegularProofs,
            name: b"DropAuthZoneRegularProofs",
        }),
        INSTRUCTION_DROP_AUTH_ZONE_SIGNATURE_PROOFS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::DropAuthZoneSignatureProofs,
            name: b"DropAuthZoneSignatureProofs",
        }),
        INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_AMOUNT_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromBucketOfAmount,
            name: b"CreateProofFromBucketOfAmount",
        }),
        INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_NON_FUNGIBLES_DISCRIMINATOR => {
            Some(InstructionInfo {
                instruction: Instruction::CreateProofFromBucketOfNonFungibles,
                name: b"CreateProofFromBucketOfNonFungibles",
            })
        }
        INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_ALL_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromBucketOfAll,
            name: b"CreateProofFromBucketOfAll",
        }),
        INSTRUCTION_BURN_RESOURCE_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::BurnResource,
            name: b"BurnResource",
        }),
        INSTRUCTION_CLONE_PROOF_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CloneProof,
            name: b"CloneProof",
        }),
        INSTRUCTION_DROP_PROOF_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::DropProof,
            name: b"DropProof",
        }),
        INSTRUCTION_CALL_FUNCTION_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CallFunction,
            name: b"CallFunction",
        }),
        INSTRUCTION_CALL_METHOD_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CallMethod,
            name: b"CallMethod",
        }),
        INSTRUCTION_CALL_ROYALTY_METHOD_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CallRoyaltyMethod,
            name: b"CallRoyaltyMethod",
        }),
        INSTRUCTION_CALL_METADATA_METHOD_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CallMetadataMethod,
            name: b"CallMetadataMethod",
        }),
        INSTRUCTION_CALL_ROLE_ASSIGNMENT_METHOD_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CallRoleAssignmentMethod,
            name: b"CallRoleAssignmentMethod",
        }),
        INSTRUCTION_CALL_DIRECT_VAULT_METHOD_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CallDirectVaultMethod,
            name: b"CallDirectVaultMethod",
        }),
        INSTRUCTION_DROP_NAMED_PROOFS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::DropNamedProofs,
            name: b"DropNamedProofs",
        }),
        INSTRUCTION_DROP_ALL_PROOFS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::DropAllProofs,
            name: b"DropAllProofs",
        }),
        INSTRUCTION_ALLOCATE_GLOBAL_ADDRESS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::AllocateGlobalAddress,
            name: b"AllocateGlobalAddress",
        }),

        _ => None,
    }
}
// From transaction/src/model/v1/instruction.rs
//==============
// Worktop
//==============
pub const INSTRUCTION_TAKE_FROM_WORKTOP_DISCRIMINATOR: u8 = 0x00;
pub const INSTRUCTION_TAKE_NON_FUNGIBLES_FROM_WORKTOP_DISCRIMINATOR: u8 = 0x01;
pub const INSTRUCTION_TAKE_ALL_FROM_WORKTOP_DISCRIMINATOR: u8 = 0x02;
pub const INSTRUCTION_RETURN_TO_WORKTOP_DISCRIMINATOR: u8 = 0x03;
pub const INSTRUCTION_ASSERT_WORKTOP_CONTAINS_DISCRIMINATOR: u8 = 0x04;
pub const INSTRUCTION_ASSERT_WORKTOP_CONTAINS_NON_FUNGIBLES_DISCRIMINATOR: u8 = 0x05;
pub const INSTRUCTION_ASSERT_WORKTOP_CONTAINS_ANY_DISCRIMINATOR: u8 = 0x06;

//==============
// Auth zone
//==============
pub const INSTRUCTION_POP_FROM_AUTH_ZONE_DISCRIMINATOR: u8 = 0x10;
pub const INSTRUCTION_PUSH_TO_AUTH_ZONE_DISCRIMINATOR: u8 = 0x11;
pub const INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_AMOUNT_DISCRIMINATOR: u8 = 0x14;
pub const INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_NON_FUNGIBLES_DISCRIMINATOR: u8 = 0x15;
pub const INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_ALL_DISCRIMINATOR: u8 = 0x16;
pub const INSTRUCTION_DROP_AUTH_ZONE_PROOFS_DISCRIMINATOR: u8 = 0x12;
pub const INSTRUCTION_DROP_AUTH_ZONE_REGULAR_PROOFS_DISCRIMINATOR: u8 = 0x13;
pub const INSTRUCTION_DROP_AUTH_ZONE_SIGNATURE_PROOFS_DISCRIMINATOR: u8 = 0x17;

//==============
// Named bucket
//==============
pub const INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_AMOUNT_DISCRIMINATOR: u8 = 0x21;
pub const INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_NON_FUNGIBLES_DISCRIMINATOR: u8 = 0x22;
pub const INSTRUCTION_CREATE_PROOF_FROM_BUCKET_OF_ALL_DISCRIMINATOR: u8 = 0x23;
pub const INSTRUCTION_BURN_RESOURCE_DISCRIMINATOR: u8 = 0x24;

//==============
// Named proof
//==============
pub const INSTRUCTION_CLONE_PROOF_DISCRIMINATOR: u8 = 0x30;
pub const INSTRUCTION_DROP_PROOF_DISCRIMINATOR: u8 = 0x31;

//==============
// Invocation
//==============
pub const INSTRUCTION_CALL_FUNCTION_DISCRIMINATOR: u8 = 0x40;
pub const INSTRUCTION_CALL_METHOD_DISCRIMINATOR: u8 = 0x41;
pub const INSTRUCTION_CALL_ROYALTY_METHOD_DISCRIMINATOR: u8 = 0x42;
pub const INSTRUCTION_CALL_METADATA_METHOD_DISCRIMINATOR: u8 = 0x43;
pub const INSTRUCTION_CALL_ROLE_ASSIGNMENT_METHOD_DISCRIMINATOR: u8 = 0x44;
pub const INSTRUCTION_CALL_DIRECT_VAULT_METHOD_DISCRIMINATOR: u8 = 0x45;

//==============
// Complex
//==============
pub const INSTRUCTION_DROP_NAMED_PROOFS_DISCRIMINATOR: u8 = 0x52;
pub const INSTRUCTION_DROP_ALL_PROOFS_DISCRIMINATOR: u8 = 0x50;
pub const INSTRUCTION_ALLOCATE_GLOBAL_ADDRESS_DISCRIMINATOR: u8 = 0x51;
