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
const ASSERT_ACCESS_RULE: u8 = 32;
const CALL_FUNCTION: u8 = 33;
const CALL_METHOD: u8 = 34;

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
    AssertAccessRule,    // { access_rule: AccessRule, },
    CallFunction, // { package_address: PackageAddress, blueprint_name: String, function_name: String, args: ManifestValue, },
    CallMethod, // { component_address: ComponentAddress, method_name: String, args: ManifestValue, },
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ParameterType {
    Ignored,
    AccessRule,
    AccessRulesConfig,
    BTreeMapByStringToRoyaltyConfig, // Royalty config
    BTreeMapByStringToString,        // Metadata
    BTreeSetOfNonFungibleLocalId,
    ComponentAddress,
    Decimal,
    ManifestAddress,
    ManifestBlobRef,
    ManifestBucket,
    ManifestProof,
    ManifestValue,
    MethodKey,
    PackageAddress,
    ResourceAddress,
    RoyaltyConfig,
    String,
    ObjectId,
    U8,
}

#[derive(Copy, Clone, Debug)]
pub struct InstructionInfo {
    pub instruction: Instruction,
    pub name: &'static [u8],
    pub params: &'static [ParameterType],
}

pub fn to_instruction(input: u8) -> Option<InstructionInfo> {
    match input {
        TAKE_FROM_WORKTOP => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktop,
            name: b"TakeFromWorktop",
            params: &[ParameterType::ResourceAddress],
        }),
        TAKE_FROM_WORKTOP_BY_AMOUNT => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktopByAmount,
            name: b"TakeFromWorktopByAmount",
            params: &[ParameterType::Decimal, ParameterType::ResourceAddress],
        }),
        TAKE_FROM_WORKTOP_BY_IDS => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktopByIds,
            name: b"TakeFromWorktopByIds",
            params: &[
                ParameterType::BTreeSetOfNonFungibleLocalId,
                ParameterType::ResourceAddress,
            ],
        }),
        RETURN_TO_WORKTOP => Some(InstructionInfo {
            instruction: Instruction::ReturnToWorktop,
            name: b"ReturnToWorktop",
            params: &[ParameterType::ManifestBucket],
        }),
        ASSERT_WORKTOP_CONTAINS => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContains,
            name: b"AssertWorktopContains",
            params: &[ParameterType::ResourceAddress],
        }),
        ASSERT_WORKTOP_CONTAINS_BY_AMOUNT => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContainsByAmount,
            name: b"AssertWorktopContainsByAmount",
            params: &[ParameterType::Decimal, ParameterType::ResourceAddress],
        }),
        ASSERT_WORKTOP_CONTAINS_BY_IDS => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContainsByIds,
            name: b"AssertWorktopContainsByIds",
            params: &[
                ParameterType::BTreeSetOfNonFungibleLocalId,
                ParameterType::ResourceAddress,
            ],
        }),
        POP_FROM_AUTH_ZONE => Some(InstructionInfo {
            instruction: Instruction::PopFromAuthZone,
            name: b"PopFromAuthZone",
            params: &[],
        }),
        PUSH_TO_AUTH_ZONE => Some(InstructionInfo {
            instruction: Instruction::PushToAuthZone,
            name: b"PushToAuthZone",
            params: &[ParameterType::ManifestProof],
        }),
        CLEAR_AUTH_ZONE => Some(InstructionInfo {
            instruction: Instruction::ClearAuthZone,
            name: b"ClearAuthZone",
            params: &[],
        }),
        CREATE_PROOF_FROM_AUTH_ZONE => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZone,
            name: b"CreateProofFromAuthZone",
            params: &[ParameterType::ResourceAddress],
        }),
        CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZoneByAmount,
            name: b"CreateProofFromAuthZoneByAmount",
            params: &[ParameterType::Decimal, ParameterType::ResourceAddress],
        }),
        CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZoneByIds,
            name: b"CreateProofFromAuthZoneByIds",
            params: &[
                ParameterType::BTreeSetOfNonFungibleLocalId,
                ParameterType::ResourceAddress,
            ],
        }),
        CREATE_PROOF_FROM_BUCKET => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromBucket,
            name: b"CreateProofFromBucket",
            params: &[ParameterType::ManifestBucket],
        }),
        CLONE_PROOF => Some(InstructionInfo {
            instruction: Instruction::CloneProof,
            name: b"CloneProof",
            params: &[ParameterType::ManifestProof],
        }),
        DROP_PROOF => Some(InstructionInfo {
            instruction: Instruction::DropProof,
            name: b"DropProof",
            params: &[ParameterType::ManifestProof],
        }),
        DROP_ALL_PROOFS => Some(InstructionInfo {
            instruction: Instruction::DropAllProofs,
            name: b"DropAllProofs",
            params: &[],
        }),
        CLEAR_SIGNATURE_PROOFS => Some(InstructionInfo {
            instruction: Instruction::ClearSignatureProofs,
            name: b"ClearSignatureProofs",
            params: &[],
        }),
        PUBLISH_PACKAGE => Some(InstructionInfo {
            instruction: Instruction::PublishPackage,
            name: b"PublishPackage",
            params: &[
                ParameterType::ManifestBlobRef,
                ParameterType::ManifestBlobRef,
                ParameterType::BTreeMapByStringToRoyaltyConfig,
                ParameterType::BTreeMapByStringToString,
            ],
        }),
        PUBLISH_PACKAGE_ADVANCED => Some(InstructionInfo {
            instruction: Instruction::PublishPackage,
            name: b"PublishPackage",
            params: &[
                ParameterType::ManifestBlobRef,
                ParameterType::ManifestBlobRef,
                ParameterType::BTreeMapByStringToRoyaltyConfig,
                ParameterType::BTreeMapByStringToString,
                ParameterType::AccessRulesConfig,
            ],
        }),
        BURN_RESOURCE => Some(InstructionInfo {
            instruction: Instruction::BurnResource,
            name: b"BurnResource",
            params: &[ParameterType::ManifestBucket],
        }),
        RECALL_RESOURCE => Some(InstructionInfo {
            instruction: Instruction::RecallResource,
            name: b"RecallResource",
            params: &[ParameterType::ObjectId, ParameterType::Decimal],
        }),
        SET_METADATA => Some(InstructionInfo {
            instruction: Instruction::SetMetadata,
            name: b"SetMetadata",
            params: &[
                ParameterType::ManifestAddress,
                ParameterType::String,
                ParameterType::String,
            ],
        }),
        REMOVE_METADATA => Some(InstructionInfo {
            instruction: Instruction::RemoveMetadata,
            name: b"RemoveMetadata",
            params: &[ParameterType::ManifestAddress, ParameterType::String],
        }),
        SET_PACKAGE_ROYALTY_CONFIG => Some(InstructionInfo {
            instruction: Instruction::SetPackageRoyaltyConfig,
            name: b"SetPackageRoyaltyConfig",
            params: &[
                ParameterType::PackageAddress,
                ParameterType::BTreeMapByStringToRoyaltyConfig,
            ],
        }),
        SET_COMPONENT_ROYALTY_CONFIG => Some(InstructionInfo {
            instruction: Instruction::SetComponentRoyaltyConfig,
            name: b"SetComponentRoyaltyConfig",
            params: &[
                ParameterType::ComponentAddress,
                ParameterType::RoyaltyConfig,
            ],
        }),
        CLAIM_PACKAGE_ROYALTY => Some(InstructionInfo {
            instruction: Instruction::ClaimPackageRoyalty,
            name: b"ClaimPackageRoyalty",
            params: &[ParameterType::PackageAddress],
        }),
        CLAIM_COMPONENT_ROYALTY => Some(InstructionInfo {
            instruction: Instruction::ClaimComponentRoyalty,
            name: b"ClaimComponentRoyalty",
            params: &[ParameterType::ComponentAddress],
        }),
        SET_METHOD_ACCESS_RULE => Some(InstructionInfo {
            instruction: Instruction::SetMethodAccessRule,
            name: b"SetMethodAccessRule",
            params: &[
                ParameterType::ManifestAddress,
                ParameterType::MethodKey,
                ParameterType::AccessRule,
            ],
        }),
        MINT_FUNGIBLE => Some(InstructionInfo {
            instruction: Instruction::MintFungible,
            name: b"MintFungible",
            params: &[ParameterType::ResourceAddress, ParameterType::Decimal],
        }),
        MINT_NON_FUNGIBLE => Some(InstructionInfo {
            instruction: Instruction::MintNonFungible,
            name: b"MintNonFungible",
            params: &[ParameterType::ResourceAddress, ParameterType::ManifestValue],
        }),
        MINT_UUID_NON_FUNGIBLE => Some(InstructionInfo {
            instruction: Instruction::MintUuidNonFungible,
            name: b"MintUuidNonFungible",
            params: &[ParameterType::ResourceAddress, ParameterType::ManifestValue],
        }),
        ASSERT_ACCESS_RULE => Some(InstructionInfo {
            instruction: Instruction::AssertAccessRule,
            name: b"CreateIdentity",
            params: &[ParameterType::AccessRule],
        }),
        CALL_FUNCTION => Some(InstructionInfo {
            instruction: Instruction::CallFunction,
            name: b"CallFunction",
            params: &[
                ParameterType::PackageAddress,
                ParameterType::String,
                ParameterType::String,
                ParameterType::ManifestValue,
            ],
        }),
        CALL_METHOD => Some(InstructionInfo {
            instruction: Instruction::CallMethod,
            name: b"CallMethod",
            params: &[
                ParameterType::ComponentAddress,
                ParameterType::String,
                ParameterType::ManifestValue,
            ],
        }),
        _ => None,
    }
}
