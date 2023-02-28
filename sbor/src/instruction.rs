// Instructions recognized by instruction extractor

// Keep in sync with
// https://raw.githubusercontent.com/radixdlt/radixdlt-scrypto/develop/transaction/src/model/instruction.rs
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    PublishPackage, // { code: ManifestBlobRef, abi: ManifestBlobRef, royalty_config: BTreeMap<String, RoyaltyConfig>, metadata: BTreeMap<String, String>, access_rules: AccessRules, },
    PublishPackageWithOwner, // { code: ManifestBlobRef, abi: ManifestBlobRef, owner_badge: NonFungibleGlobalId, },
    BurnResource,            // { bucket_id: ManifestBucket, },
    RecallResource,          // { vault_id: VaultId, amount: Decimal, },
    SetMetadata,             // { entity_address: GlobalAddress, key: String, value: String, },
    SetPackageRoyaltyConfig, // { package_address: PackageAddress, royalty_config: BTreeMap<String, RoyaltyConfig>, },
    SetComponentRoyaltyConfig, // { component_address: ComponentAddress, royalty_config: RoyaltyConfig, },
    ClaimPackageRoyalty,       // { package_address: PackageAddress, },
    ClaimComponentRoyalty,     // { component_address: ComponentAddress, },
    SetMethodAccessRule, // { entity_address: GlobalAddress, index: u32, key: AccessRuleKey, rule: AccessRule, },
    MintFungible,        // { resource_address: ResourceAddress, amount: Decimal, },
    MintNonFungible, // { resource_address: ResourceAddress, entries: BTreeMap<NonFungibleLocalId, (Vec<u8>, Vec<u8>)>, },
    MintUuidNonFungible, // { resource_address: ResourceAddress, entries: Vec<(Vec<u8>, Vec<u8>)>, },
    CreateFungibleResource, // { divisibility: u8, metadata: BTreeMap<String, String>, access_rules: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>, initial_supply: Option<Decimal>, },
    CreateFungibleResourceWithOwner, // { divisibility: u8, metadata: BTreeMap<String, String>, owner_badge: NonFungibleGlobalId, initial_supply: Option<Decimal>, },
    CreateNonFungibleResource, // { id_type: NonFungibleIdType, metadata: BTreeMap<String, String>, access_rules: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>, initial_supply: Option<BTreeMap<NonFungibleLocalId, (Vec<u8>, Vec<u8>)>>, },
    CreateNonFungibleResourceWithOwner, // { id_type: NonFungibleIdType, metadata: BTreeMap<String, String>, owner_badge: NonFungibleGlobalId, initial_supply: Option<BTreeMap<NonFungibleLocalId, (Vec<u8>, Vec<u8>)>>, },
    CreateAccessController, // { controlled_asset: ManifestBucket, primary_role: AccessRule, recovery_role: AccessRule, confirmation_role: AccessRule, timed_recovery_delay_in_minutes: Option<u32>, },
    CreateIdentity,         // { access_rule: AccessRule, },
    CallFunction, // { package_address: PackageAddress, blueprint_name: String, function_name: String, args: Vec<u8>, },
    CallMethod,   // { component_address: ComponentAddress, method_name: String, args: Vec<u8>, },
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ParameterType {
    Ignored,
    AccessRule,
    AccessRuleKey,
    AccessRules,
    BTreeMapByNonFungibleLocalId,
    BTreeMapByResourceMethodAuthKey,
    BTreeMapByStringToRoyaltyConfig,
    BTreeMapByStringToString,
    BTreeSetOfNonFungibleLocalId,
    ComponentAddress,
    Decimal,
    GlobalAddress,
    ManifestBlobRef,
    ManifestBucket,
    ManifestProof,
    NonFungibleGlobalId,
    NonFungibleIdType,
    OptionOfBTreeMapByNonFungibleLocalId,
    OptionOfDecimal,
    OptionOfU32,
    PackageAddress,
    ResourceAddress,
    RoyaltyConfig,
    String,
    VaultId,
    VecOfVecTuple,
    VecOfU8,
    U32,
    U8,
}

#[derive(Copy, Clone, Debug)]
pub struct InstructionInfo {
    pub instruction: Instruction,
    pub parameter_count: u8,
    pub name: &'static [u8],
    pub params: &'static [ParameterType],
}

pub fn to_instruction(input: u8) -> Option<InstructionInfo> {
    match input {
        0 => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktop,
            parameter_count: 1,
            name: b"TakeFromWorktop",
            params: &[ParameterType::ResourceAddress],
        }),
        1 => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktopByAmount,
            parameter_count: 2,
            name: b"TakeFromWorktopByAmount",
            params: &[ParameterType::Decimal, ParameterType::ResourceAddress],
        }),
        2 => Some(InstructionInfo {
            instruction: Instruction::TakeFromWorktopByIds,
            parameter_count: 2,
            name: b"TakeFromWorktopByIds",
            params: &[
                ParameterType::BTreeSetOfNonFungibleLocalId,
                ParameterType::ResourceAddress,
            ],
        }),
        3 => Some(InstructionInfo {
            instruction: Instruction::ReturnToWorktop,
            parameter_count: 1,
            name: b"ReturnToWorktop",
            params: &[ParameterType::ManifestBucket],
        }),
        4 => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContains,
            parameter_count: 1,
            name: b"AssertWorktopContains",
            params: &[ParameterType::ResourceAddress],
        }),
        5 => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContainsByAmount,
            parameter_count: 2,
            name: b"AssertWorktopContainsByAmount",
            params: &[ParameterType::Decimal, ParameterType::ResourceAddress],
        }),
        6 => Some(InstructionInfo {
            instruction: Instruction::AssertWorktopContainsByIds,
            parameter_count: 2,
            name: b"AssertWorktopContainsByIds",
            params: &[
                ParameterType::BTreeSetOfNonFungibleLocalId,
                ParameterType::ResourceAddress,
            ],
        }),
        7 => Some(InstructionInfo {
            instruction: Instruction::PopFromAuthZone,
            parameter_count: 0,
            name: b"PopFromAuthZone",
            params: &[],
        }),
        8 => Some(InstructionInfo {
            instruction: Instruction::PushToAuthZone,
            parameter_count: 1,
            name: b"PushToAuthZone",
            params: &[ParameterType::ManifestProof],
        }),
        9 => Some(InstructionInfo {
            instruction: Instruction::ClearAuthZone,
            parameter_count: 0,
            name: b"ClearAuthZone",
            params: &[],
        }),
        10 => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZone,
            parameter_count: 1,
            name: b"CreateProofFromAuthZone",
            params: &[ParameterType::ResourceAddress],
        }),
        11 => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZoneByAmount,
            parameter_count: 2,
            name: b"CreateProofFromAuthZoneByAmount",
            params: &[ParameterType::Decimal, ParameterType::ResourceAddress],
        }),
        12 => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromAuthZoneByIds,
            parameter_count: 2,
            name: b"CreateProofFromAuthZoneByIds",
            params: &[
                ParameterType::BTreeSetOfNonFungibleLocalId,
                ParameterType::ResourceAddress,
            ],
        }),
        13 => Some(InstructionInfo {
            instruction: Instruction::CreateProofFromBucket,
            parameter_count: 1,
            name: b"CreateProofFromBucket",
            params: &[ParameterType::ManifestBucket],
        }),
        14 => Some(InstructionInfo {
            instruction: Instruction::CloneProof,
            parameter_count: 1,
            name: b"CloneProof",
            params: &[ParameterType::ManifestProof],
        }),
        15 => Some(InstructionInfo {
            instruction: Instruction::DropProof,
            parameter_count: 1,
            name: b"DropProof",
            params: &[ParameterType::ManifestProof],
        }),
        16 => Some(InstructionInfo {
            instruction: Instruction::DropAllProofs,
            parameter_count: 0,
            name: b"DropAllProofs",
            params: &[],
        }),
        17 => Some(InstructionInfo {
            instruction: Instruction::PublishPackage,
            parameter_count: 5,
            name: b"PublishPackage",
            params: &[
                ParameterType::ManifestBlobRef,
                ParameterType::ManifestBlobRef,
                ParameterType::BTreeMapByStringToRoyaltyConfig,
                ParameterType::BTreeMapByStringToString,
                ParameterType::AccessRules,
            ],
        }),
        18 => Some(InstructionInfo {
            instruction: Instruction::PublishPackageWithOwner,
            parameter_count: 3,
            name: b"PublishPackageWithOwner",
            params: &[
                ParameterType::ManifestBlobRef,
                ParameterType::ManifestBlobRef,
                ParameterType::NonFungibleGlobalId,
            ],
        }),
        19 => Some(InstructionInfo {
            instruction: Instruction::BurnResource,
            parameter_count: 1,
            name: b"BurnResource",
            params: &[ParameterType::ManifestBucket],
        }),
        20 => Some(InstructionInfo {
            instruction: Instruction::RecallResource,
            parameter_count: 2,
            name: b"RecallResource",
            params: &[ParameterType::VaultId, ParameterType::Decimal],
        }),
        21 => Some(InstructionInfo {
            instruction: Instruction::SetMetadata,
            parameter_count: 3,
            name: b"SetMetadata",
            params: &[
                ParameterType::GlobalAddress,
                ParameterType::String,
                ParameterType::String,
            ],
        }),
        22 => Some(InstructionInfo {
            instruction: Instruction::SetPackageRoyaltyConfig,
            parameter_count: 2,
            name: b"SetPackageRoyaltyConfig",
            params: &[
                ParameterType::PackageAddress,
                ParameterType::BTreeMapByStringToRoyaltyConfig,
            ],
        }),
        23 => Some(InstructionInfo {
            instruction: Instruction::SetComponentRoyaltyConfig,
            parameter_count: 2,
            name: b"SetComponentRoyaltyConfig",
            params: &[
                ParameterType::ComponentAddress,
                ParameterType::RoyaltyConfig,
            ],
        }),
        24 => Some(InstructionInfo {
            instruction: Instruction::ClaimPackageRoyalty,
            parameter_count: 1,
            name: b"ClaimPackageRoyalty",
            params: &[ParameterType::PackageAddress],
        }),
        25 => Some(InstructionInfo {
            instruction: Instruction::ClaimComponentRoyalty,
            parameter_count: 1,
            name: b"ClaimComponentRoyalty",
            params: &[ParameterType::ComponentAddress],
        }),
        26 => Some(InstructionInfo {
            instruction: Instruction::SetMethodAccessRule,
            parameter_count: 4,
            name: b"SetMethodAccessRule",
            params: &[
                ParameterType::GlobalAddress,
                ParameterType::U32,
                ParameterType::AccessRuleKey,
                ParameterType::AccessRule,
            ],
        }),
        27 => Some(InstructionInfo {
            instruction: Instruction::MintFungible,
            parameter_count: 2,
            name: b"MintFungible",
            params: &[ParameterType::ResourceAddress, ParameterType::Decimal],
        }),
        28 => Some(InstructionInfo {
            instruction: Instruction::MintNonFungible,
            parameter_count: 2,
            name: b"MintNonFungible",
            params: &[
                ParameterType::ResourceAddress,
                ParameterType::BTreeMapByNonFungibleLocalId,
            ],
        }),
        29 => Some(InstructionInfo {
            instruction: Instruction::MintUuidNonFungible,
            parameter_count: 2,
            name: b"MintUuidNonFungible",
            params: &[ParameterType::ResourceAddress, ParameterType::VecOfVecTuple],
        }),
        30 => Some(InstructionInfo {
            instruction: Instruction::CreateFungibleResource,
            parameter_count: 4,
            name: b"CreateFungibleResource",
            params: &[
                ParameterType::U8,
                ParameterType::BTreeMapByStringToString,
                ParameterType::BTreeMapByResourceMethodAuthKey,
                ParameterType::OptionOfDecimal,
            ],
        }),
        31 => Some(InstructionInfo {
            instruction: Instruction::CreateFungibleResourceWithOwner,
            parameter_count: 4,
            name: b"CreateFungibleResourceWithOwner",
            params: &[
                ParameterType::U8,
                ParameterType::BTreeMapByStringToString,
                ParameterType::NonFungibleGlobalId,
                ParameterType::OptionOfDecimal,
            ],
        }),
        32 => Some(InstructionInfo {
            instruction: Instruction::CreateNonFungibleResource,
            parameter_count: 4,
            name: b"CreateNonFungibleResource",
            params: &[
                ParameterType::NonFungibleIdType,
                ParameterType::BTreeMapByStringToString,
                ParameterType::BTreeMapByResourceMethodAuthKey,
                ParameterType::OptionOfBTreeMapByNonFungibleLocalId,
            ],
        }),
        33 => Some(InstructionInfo {
            instruction: Instruction::CreateNonFungibleResourceWithOwner,
            parameter_count: 4,
            name: b"CreateNonFungibleResourceWithOwner",
            params: &[
                ParameterType::NonFungibleIdType,
                ParameterType::BTreeMapByStringToString,
                ParameterType::NonFungibleGlobalId,
                ParameterType::OptionOfBTreeMapByNonFungibleLocalId,
            ],
        }),
        34 => Some(InstructionInfo {
            instruction: Instruction::CreateAccessController,
            parameter_count: 5,
            name: b"CreateAccessController",
            params: &[
                ParameterType::ManifestBucket,
                ParameterType::AccessRule,
                ParameterType::AccessRule,
                ParameterType::AccessRule,
                ParameterType::OptionOfU32,
            ],
        }),
        35 => Some(InstructionInfo {
            instruction: Instruction::CreateIdentity,
            parameter_count: 1,
            name: b"CreateIdentity",
            params: &[ParameterType::AccessRule],
        }),
        36 => Some(InstructionInfo {
            instruction: Instruction::CallFunction,
            parameter_count: 4,
            name: b"CallFunction",
            params: &[
                ParameterType::PackageAddress,
                ParameterType::String,
                ParameterType::String,
                ParameterType::VecOfU8,
            ],
        }),
        37 => Some(InstructionInfo {
            instruction: Instruction::CallMethod,
            parameter_count: 3,
            name: b"CallMethod",
            params: &[
                ParameterType::ComponentAddress,
                ParameterType::String,
                ParameterType::VecOfU8,
            ],
        }),
        _ => None,
    }
}

//////////////////////////////////////////////////////////////////////////////
