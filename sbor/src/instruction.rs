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

#[derive(Copy, Clone, Debug)]
pub struct InstructionInfo(pub Instruction, pub u8);

pub fn to_instruction(input: u8) -> Option<InstructionInfo> {
    match input {
        0 => Some(InstructionInfo(Instruction::TakeFromWorktop, 1)),
        1 => Some(InstructionInfo(Instruction::TakeFromWorktopByAmount, 2)),
        2 => Some(InstructionInfo(Instruction::TakeFromWorktopByIds, 2)),
        3 => Some(InstructionInfo(Instruction::ReturnToWorktop, 1)),
        4 => Some(InstructionInfo(Instruction::AssertWorktopContains, 1)),
        5 => Some(InstructionInfo(
            Instruction::AssertWorktopContainsByAmount,
            2,
        )),
        6 => Some(InstructionInfo(Instruction::AssertWorktopContainsByIds, 2)),
        7 => Some(InstructionInfo(Instruction::PopFromAuthZone, 0)),
        8 => Some(InstructionInfo(Instruction::PushToAuthZone, 1)),
        9 => Some(InstructionInfo(Instruction::ClearAuthZone, 0)),
        10 => Some(InstructionInfo(Instruction::CreateProofFromAuthZone, 1)),
        11 => Some(InstructionInfo(
            Instruction::CreateProofFromAuthZoneByAmount,
            2,
        )),
        12 => Some(InstructionInfo(
            Instruction::CreateProofFromAuthZoneByIds,
            2,
        )),
        13 => Some(InstructionInfo(Instruction::CreateProofFromBucket, 1)),
        14 => Some(InstructionInfo(Instruction::CloneProof, 1)),
        15 => Some(InstructionInfo(Instruction::DropProof, 1)),
        16 => Some(InstructionInfo(Instruction::DropAllProofs, 0)),
        17 => Some(InstructionInfo(Instruction::PublishPackage, 5)),
        18 => Some(InstructionInfo(Instruction::PublishPackageWithOwner, 3)),
        19 => Some(InstructionInfo(Instruction::BurnResource, 1)),
        20 => Some(InstructionInfo(Instruction::RecallResource, 2)),
        21 => Some(InstructionInfo(Instruction::SetMetadata, 3)),
        22 => Some(InstructionInfo(Instruction::SetPackageRoyaltyConfig, 2)),
        23 => Some(InstructionInfo(Instruction::SetComponentRoyaltyConfig, 2)),
        24 => Some(InstructionInfo(Instruction::ClaimPackageRoyalty, 1)),
        25 => Some(InstructionInfo(Instruction::ClaimComponentRoyalty, 1)),
        26 => Some(InstructionInfo(Instruction::SetMethodAccessRule, 4)),
        27 => Some(InstructionInfo(Instruction::MintFungible, 2)),
        28 => Some(InstructionInfo(Instruction::MintNonFungible, 2)),
        29 => Some(InstructionInfo(Instruction::MintUuidNonFungible, 2)),
        30 => Some(InstructionInfo(Instruction::CreateFungibleResource, 4)),
        31 => Some(InstructionInfo(
            Instruction::CreateFungibleResourceWithOwner,
            4,
        )),
        32 => Some(InstructionInfo(Instruction::CreateNonFungibleResource, 4)),
        33 => Some(InstructionInfo(
            Instruction::CreateNonFungibleResourceWithOwner,
            4,
        )),
        34 => Some(InstructionInfo(Instruction::CreateAccessController, 5)),
        35 => Some(InstructionInfo(Instruction::CreateIdentity, 1)),
        36 => Some(InstructionInfo(Instruction::CallFunction, 4)),
        37 => Some(InstructionInfo(Instruction::CallMethod, 3)),
        _ => None,
    }
}
