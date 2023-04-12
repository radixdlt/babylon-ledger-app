// Instructions recognized by instruction extractor
// Keep in sync with
// https://raw.githubusercontent.com/radixdlt/radixdlt-scrypto/develop/transaction/src/model/instruction.rs

const TAKE_FROM_WORKTOP: u8 = 00;
const TAKE_FROM_WORKTOP_BY_AMOUNT: u8 = 01;
const TAKE_FROM_WORKTOP_BY_IDS: u8 = 02;
const RETURN_TO_WORKTOP: u8 = 03;
const ASSERT_WORKTOP_CONTAINS: u8 = 04;
const ASSERT_WORKTOP_CONTAINS_BY_AMOUNT: u8 = 05;
const ASSERT_WORKTOP_CONTAINS_BY_IDS: u8 = 06;
const POP_FROM_AUTH_ZONE: u8 = 07;
const PUSH_TO_AUTH_ZONE: u8 = 08;
const CLEAR_AUTH_ZONE: u8 = 09;
const CREATE_PROOF_FROM_AUTH_ZONE: u8 = 10;
const CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT: u8 = 11;
const CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS: u8 = 12;
const CREATE_PROOF_FROM_BUCKET: u8 = 13;
const CLONE_PROOF: u8 = 14;
const DROP_PROOF: u8 = 15;
const DROP_ALL_PROOFS: u8 = 16;
const CLEAR_SIGNATURE_PROOFS: u8 = 17;
const PUBLISH_PACKAGE: u8 = 18;
const PUBLISH_PACKAGE_ADVANCED: u8 = 19;
const BURN_RESOURCE: u8 = 20;
const RECALL_RESOURCE: u8 = 21;
const SET_METADATA: u8 = 22;
const REMOVE_METADATA: u8 = 23;
const SET_PACKAGE_ROYALTY_CONFIG: u8 = 24;
const SET_COMPONENT_ROYALTY_CONFIG: u8 = 25;
const CLAIM_PACKAGE_ROYALTY: u8 = 26;
const CLAIM_COMPONENT_ROYALTY: u8 = 27;
const SET_METHOD_ACCESS_RULE: u8 = 28;
const MINT_FUNGIBLE: u8 = 29;
const MINT_NON_FUNGIBLE: u8 = 30;
const MINT_UUID_NON_FUNGIBLE: u8 = 31;
const CALL_FUNCTION: u8 = 32;
const CALL_METHOD: u8 = 33;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    TakeFromWorktop,               //{ resource_address: ResourceAddress, },
    TakeFromWorktopByAmount,       // { amount: Decimal, resource_address: ResourceAddress, },
    TakeFromWorktopByIds, // { ids: BTreeSet<NonFungibleLocalId>, resource_address: ResourceAddress, },
    ReturnToWorktop,      // { bucket_id: ManifestBucket, },
    AssertWorktopContains, // { resource_address: ResourceAddress, },
    AssertWorktopContainsByAmount, // { amount: Decimal, resource_address: ResourceAddress, },
    AssertWorktopContainsByIds, // { ids: BTreeSet<NonFungibleLocalId>, resource_address: ResourceAddress, },
    PopFromAuthZone,
    PushToAuthZone, // { proof_id: ManifestProof, },
    ClearAuthZone,
    CreateProofFromAuthZone, // { resource_address: ResourceAddress, },
    CreateProofFromAuthZoneByAmount, // { amount: Decimal, resource_address: ResourceAddress, },
    CreateProofFromAuthZoneByIds, // { ids: BTreeSet<NonFungibleLocalId>, resource_address: ResourceAddress, },
    CreateProofFromBucket,        // { bucket_id: ManifestBucket, },
    CloneProof,                   // { proof_id: ManifestProof, },
    DropProof,                    // { proof_id: ManifestProof, },
    DropAllProofs,
    ClearSignatureProofs,
    PublishPackage, // { code: ManifestBlobRef, schema: ManifestBlobRef, royalty_config: BTreeMap<String, RoyaltyConfig>, metadata: BTreeMap<String, String>, },
    PublishPackageAdvanced, // { code: ManifestBlobRef, schema: ManifestBlobRef, royalty_config: BTreeMap<String, RoyaltyConfig>, metadata: BTreeMap<String, String>, access_rules: AccessRulesConfig, },
    BurnResource,           // { bucket_id: ManifestBucket, },
    RecallResource,         // { vault_id: ObjectId, amount: Decimal, },
    SetMetadata,            // { entity_address: ManifestAddress, key: String, value: String, },
    RemoveMetadata,         // { entity_address: ManifestAddress, key: String, },
    SetPackageRoyaltyConfig, // { package_address: PackageAddress, royalty_config: BTreeMap<String, RoyaltyConfig>, },
    SetComponentRoyaltyConfig, // { component_address: ComponentAddress, royalty_config: RoyaltyConfig, },
    ClaimPackageRoyalty,       // { package_address: PackageAddress, },
    ClaimComponentRoyalty,     // { component_address: ComponentAddress, },
    SetMethodAccessRule, // { entity_address: ManifestAddress, key: MethodKey, rule: AccessRule, },
    MintFungible,        // { resource_address: ResourceAddress, amount: Decimal, },
    MintNonFungible,     // { resource_address: ResourceAddress, args: ManifestValue, },
    MintUuidNonFungible, // { resource_address: ResourceAddress, args: ManifestValue, },
    CallFunction, // { package_address: PackageAddress, blueprint_name: String, function_name: String, args: ManifestValue, },
    CallMethod, // { component_address: ComponentAddress, method_name: String, args: ManifestValue, },
}

#[derive(Copy, Clone, Debug)]
pub struct InstructionInfo {
    pub instruction: Instruction,
    pub name: &'static [u8],
}

pub fn to_instruction(input: u8) -> Option<InstructionInfo> {
    match input {
        TAKE_FROM_WORKTOP => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktop,
            name: b"TakeFromWorktop",
        }),
        TAKE_FROM_WORKTOP_BY_AMOUNT => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktopByAmount,
            name: b"TakeFromWorktopByAmount",
        }),
        TAKE_FROM_WORKTOP_BY_IDS => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktopByIds,
            name: b"TakeFromWorktopByIds",
        }),
        RETURN_TO_WORKTOP => Some(InstructionInfo {
            instruction: Instruction::ReturnToWorktop,
            name: b"ReturnToWorktop",
        }),
        ASSERT_WORKTOP_CONTAINS => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContains,
            name: b"AssertWorktopContains",
        }),
        ASSERT_WORKTOP_CONTAINS_BY_AMOUNT => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContainsByAmount,
            name: b"AssertWorktopContainsByAmount",
        }),
        ASSERT_WORKTOP_CONTAINS_BY_IDS => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContainsByIds,
            name: b"AssertWorktopContainsByIds",
        }),
        POP_FROM_AUTH_ZONE => Some(InstructionInfo {
            instruction: Instruction::PopFromAuthZone,
            name: b"PopFromAuthZone",
        }),
        PUSH_TO_AUTH_ZONE => Some(InstructionInfo {
            instruction: Instruction::PushToAuthZone,
            name: b"PushToAuthZone",
        }),
        CLEAR_AUTH_ZONE => Some(InstructionInfo {
            instruction: Instruction::ClearAuthZone,
            name: b"ClearAuthZone",
        }),
        CREATE_PROOF_FROM_AUTH_ZONE => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZone,
            name: b"CreateProofFromAuthZone",
        }),
        CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZoneByAmount,
            name: b"CreateProofFromAuthZoneByAmount",
        }),
        CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZoneByIds,
            name: b"CreateProofFromAuthZoneByIds",
        }),
        CREATE_PROOF_FROM_BUCKET => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromBucket,
            name: b"CreateProofFromBucket",
        }),
        CLONE_PROOF => Some(InstructionInfo {
            instruction: Instruction::CloneProof,
            name: b"CloneProof",
        }),
        DROP_PROOF => Some(InstructionInfo {
            instruction: Instruction::DropProof,
            name: b"DropProof",
        }),
        DROP_ALL_PROOFS => Some(InstructionInfo {
            instruction: Instruction::DropAllProofs,
            name: b"DropAllProofs",
        }),
        CLEAR_SIGNATURE_PROOFS => Some(InstructionInfo {
            instruction: Instruction::ClearSignatureProofs,
            name: b"ClearSignatureProofs",
        }),
        PUBLISH_PACKAGE => Some(InstructionInfo {
            instruction: Instruction::PublishPackage,
            name: b"PublishPackage",
        }),
        PUBLISH_PACKAGE_ADVANCED => Some(InstructionInfo {
            instruction: Instruction::PublishPackage,
            name: b"PublishPackageAdvanced",
        }),
        BURN_RESOURCE => Some(InstructionInfo {
            instruction: Instruction::BurnResource,
            name: b"BurnResource",
        }),
        RECALL_RESOURCE => Some(InstructionInfo {
            instruction: Instruction::RecallResource,
            name: b"RecallResource",
        }),
        SET_METADATA => Some(InstructionInfo {
            instruction: Instruction::SetMetadata,
            name: b"SetMetadata",
        }),
        REMOVE_METADATA => Some(InstructionInfo {
            instruction: Instruction::RemoveMetadata,
            name: b"RemoveMetadata",
        }),
        SET_PACKAGE_ROYALTY_CONFIG => Some(InstructionInfo {
            instruction: Instruction::SetPackageRoyaltyConfig,
            name: b"SetPackageRoyaltyConfig",
        }),
        SET_COMPONENT_ROYALTY_CONFIG => Some(InstructionInfo {
            instruction: Instruction::SetComponentRoyaltyConfig,
            name: b"SetComponentRoyaltyConfig",
        }),
        CLAIM_PACKAGE_ROYALTY => Some(InstructionInfo {
            instruction: Instruction::ClaimPackageRoyalty,
            name: b"ClaimPackageRoyalty",
        }),
        CLAIM_COMPONENT_ROYALTY => Some(InstructionInfo {
            instruction: Instruction::ClaimComponentRoyalty,
            name: b"ClaimComponentRoyalty",
        }),
        SET_METHOD_ACCESS_RULE => Some(InstructionInfo {
            instruction: Instruction::SetMethodAccessRule,
            name: b"SetMethodAccessRule",
        }),
        MINT_FUNGIBLE => Some(InstructionInfo {
            instruction: Instruction::MintFungible,
            name: b"MintFungible",
        }),
        MINT_NON_FUNGIBLE => Some(InstructionInfo {
            instruction: Instruction::MintNonFungible,
            name: b"MintNonFungible",
        }),
        MINT_UUID_NON_FUNGIBLE => Some(InstructionInfo {
            instruction: Instruction::MintUuidNonFungible,
            name: b"MintUuidNonFungible",
        }),
        CALL_FUNCTION => Some(InstructionInfo {
            instruction: Instruction::CallFunction,
            name: b"CallFunction",
        }),
        CALL_METHOD => Some(InstructionInfo {
            instruction: Instruction::CallMethod,
            name: b"CallMethod",
        }),
        _ => None,
    }
}
