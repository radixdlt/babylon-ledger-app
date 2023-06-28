// Instructions recognized by instruction extractor
// Keep in sync with
// https://raw.githubusercontent.com/radixdlt/radixdlt-scrypto/develop/transaction/src/model/instruction.rs

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    TakeAllFromWorktop,                    // { resource_address: ResourceAddress },
    TakeFromWorktop, // { resource_address: ResourceAddress, amount: Decimal, },
    TakeNonFungiblesFromWorktop, // { resource_address: ResourceAddress, ids: Vec<NonFungibleLocalId>, },
    ReturnToWorktop,             // { bucket_id: ManifestBucket },
    AssertWorktopContains,       // { resource_address: ResourceAddress, amount: Decimal, },
    AssertWorktopContainsNonFungibles, // { resource_address: ResourceAddress, ids: Vec<NonFungibleLocalId>, },
    PopFromAuthZone,                   //,
    PushToAuthZone,                    // { proof_id: ManifestProof },
    ClearAuthZone,                     //,
    CreateProofFromAuthZone,           // { resource_address: ResourceAddress },
    CreateProofFromAuthZoneOfAmount,   // { resource_address: ResourceAddress, amount: Decimal, },
    CreateProofFromAuthZoneOfNonFungibles, // { resource_address: ResourceAddress, ids: Vec<NonFungibleLocalId>, },
    CreateProofFromAuthZoneOfAll,          // { resource_address: ResourceAddress },
    ClearSignatureProofs,                  //,
    CreateProofFromBucket,                 // { bucket_id: ManifestBucket },
    CreateProofFromBucketOfAmount,         // { bucket_id: ManifestBucket, amount: Decimal, },
    CreateProofFromBucketOfNonFungibles, // { bucket_id: ManifestBucket, ids: Vec<NonFungibleLocalId>, },
    CreateProofFromBucketOfAll,          // { bucket_id: ManifestBucket },
    BurnResource,                        // { bucket_id: ManifestBucket },
    CloneProof,                          // { proof_id: ManifestProof },
    DropProof,                           // { proof_id: ManifestProof },
    CallFunction, // { package_address: DynamicPackageAddress, blueprint_name: String, function_name: String, args: ManifestValue, },
    CallMethod,   // { address: DynamicGlobalAddress, method_name: String, args: ManifestValue, },
    CallRoyaltyMethod, // { address: DynamicGlobalAddress, method_name: String, args: ManifestValue, },
    CallMetadataMethod, // { address: DynamicGlobalAddress, method_name: String, args: ManifestValue, },
    CallAccessRulesMethod, // { address: DynamicGlobalAddress, method_name: String, args: ManifestValue, },
    DropAllProofs,         //,
    RecallResource,
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
        INSTRUCTION_CLEAR_AUTH_ZONE_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::ClearAuthZone,
            name: b"ClearAuthZone",
        }),
        INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZone,
            name: b"CreateProofFromAuthZone",
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
        INSTRUCTION_CLEAR_SIGNATURE_PROOFS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::ClearSignatureProofs,
            name: b"ClearSignatureProofs",
        }),
        INSTRUCTION_CREATE_PROOF_FROM_BUCKET_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromBucket,
            name: b"CreateProofFromBucket",
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
        INSTRUCTION_CALL_ACCESS_RULES_METHOD_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::CallAccessRulesMethod,
            name: b"CallAccessRulesMethod",
        }),
        INSTRUCTION_RECALL_RESOURCE_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::RecallResource,
            name: b"RecallResource",
        }),
        INSTRUCTION_DROP_ALL_PROOFS_DISCRIMINATOR => Some(InstructionInfo {
            instruction: Instruction::DropAllProofs,
            name: b"DropAllProofs",
        }),

        _ => None,
    }
}

//==============
// Worktop
//==============
pub const INSTRUCTION_TAKE_FROM_WORKTOP_DISCRIMINATOR: u8 = 0x00;
pub const INSTRUCTION_TAKE_NON_FUNGIBLES_FROM_WORKTOP_DISCRIMINATOR: u8 = 0x01;
pub const INSTRUCTION_TAKE_ALL_FROM_WORKTOP_DISCRIMINATOR: u8 = 0x02;
pub const INSTRUCTION_RETURN_TO_WORKTOP_DISCRIMINATOR: u8 = 0x03;
pub const INSTRUCTION_ASSERT_WORKTOP_CONTAINS_DISCRIMINATOR: u8 = 0x04;
pub const INSTRUCTION_ASSERT_WORKTOP_CONTAINS_NON_FUNGIBLES_DISCRIMINATOR: u8 = 0x05;

//==============
// Auth zone
//==============
pub const INSTRUCTION_POP_FROM_AUTH_ZONE_DISCRIMINATOR: u8 = 0x10;
pub const INSTRUCTION_PUSH_TO_AUTH_ZONE_DISCRIMINATOR: u8 = 0x11;
pub const INSTRUCTION_CLEAR_AUTH_ZONE_DISCRIMINATOR: u8 = 0x12;
pub const INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_DISCRIMINATOR: u8 = 0x13;
pub const INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_AMOUNT_DISCRIMINATOR: u8 = 0x14;
pub const INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_NON_FUNGIBLES_DISCRIMINATOR: u8 = 0x15;
pub const INSTRUCTION_CREATE_PROOF_FROM_AUTH_ZONE_OF_ALL_DISCRIMINATOR: u8 = 0x16;
pub const INSTRUCTION_CLEAR_SIGNATURE_PROOFS_DISCRIMINATOR: u8 = 0x17;

//==============
// Named bucket
//==============
pub const INSTRUCTION_CREATE_PROOF_FROM_BUCKET_DISCRIMINATOR: u8 = 0x20;
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
pub const INSTRUCTION_CALL_ACCESS_RULES_METHOD_DISCRIMINATOR: u8 = 0x44;
pub const INSTRUCTION_RECALL_RESOURCE_DISCRIMINATOR: u8 = 0x45;

//==============
// Complex
//==============
pub const INSTRUCTION_DROP_ALL_PROOFS_DISCRIMINATOR: u8 = 0x50;
