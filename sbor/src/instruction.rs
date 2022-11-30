use phf::phf_map;

// Keep in sync with
// https://raw.githubusercontent.com/radixdlt/radixdlt-scrypto/develop/transaction/src/model/instruction.rs
#[repr(u8)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Instruction {
    AssertWorktopContainsByAmount, // { amount: Decimal, resource_address: ResourceAddress, },
    AssertWorktopContainsByIds, // { ids: BTreeSet<NonFungibleId>, resource_address: ResourceAddress, },
    AssertWorktopContains,      // { resource_address: ResourceAddress },
    CallFunction,               // { function_ident: ScryptoFunctionIdent, args: Vec<u8>, },
    CallMethod,                 // { method_ident: ScryptoMethodIdent, args: Vec<u8>,  },
    CallNativeFunction,         // { function_ident: NativeFunctionIdent, args: Vec<u8>, },
    CallNativeMethod,           // { method_ident: NativeMethodIdent, args: Vec<u8>, },
    ClearAuthZone,              //
    CloneProof,                 // { proof_id: ProofId },
    CreateProofFromAuthZoneByAmount, // { amount: Decimal, resource_address: ResourceAddress, },
    CreateProofFromAuthZoneByIds, // { ids: BTreeSet<NonFungibleId>, resource_address: ResourceAddress,  },
    CreateProofFromAuthZone,      // { resource_address: ResourceAddress },
    CreateProofFromBucket,        // { bucket_id: BucketId },
    DropAllProofs,
    DropProof,               // { proof_id: ProofId },
    PopFromAuthZone,         //
    PublishPackage,          // { code: Blob, abi: Blob },
    PushToAuthZone,          // { proof_id: ProofId },
    ReturnToWorktop,         // { bucket_id: BucketId },
    TakeFromWorktopByAmount, // { amount: Decimal, resource_address: ResourceAddress, },
    TakeFromWorktopByIds, // { ids: BTreeSet<NonFungibleId>, resource_address: ResourceAddress, },
    TakeFromWorktop,      // { resource_address: ResourceAddress },
}

static INSTRUCTIONS: phf::Map<&[u8], Instruction> = phf_map! {
    b"AssertWorktopContainsByAmount" => Instruction::AssertWorktopContainsByAmount,
    b"AssertWorktopContainsByIds" => Instruction::AssertWorktopContainsByIds,
    b"AssertWorktopContains" => Instruction::AssertWorktopContains,
    b"CallFunction" => Instruction::CallFunction,
    b"CallMethod" => Instruction::CallMethod,
    b"CallNativeFunction" => Instruction::CallNativeFunction,
    b"CallNativeMethod" => Instruction::CallNativeMethod,
    b"ClearAuthZone" => Instruction::ClearAuthZone,
    b"CloneProof" => Instruction::CloneProof,
    b"CreateProofFromAuthZoneByAmount" => Instruction::CreateProofFromAuthZoneByAmount,
    b"CreateProofFromAuthZoneByIds" => Instruction::CreateProofFromAuthZoneByIds,
    b"CreateProofFromAuthZone" => Instruction::CreateProofFromAuthZone,
    b"CreateProofFromBucket" => Instruction::CreateProofFromBucket,
    b"DropAllProofs" => Instruction::DropAllProofs,
    b"DropProof" => Instruction::DropProof,
    b"PopFromAuthZone" => Instruction::PopFromAuthZone,
    b"PublishPackage" => Instruction::PublishPackage,
    b"PushToAuthZone" => Instruction::PushToAuthZone,
    b"ReturnToWorktop" => Instruction::ReturnToWorktop,
    b"TakeFromWorktopByAmount" => Instruction::TakeFromWorktopByAmount,
    b"TakeFromWorktopByIds" => Instruction::TakeFromWorktopByIds,
    b"TakeFromWorktop" => Instruction::TakeFromWorktop,
};

pub fn to_instruction(input: &[u8]) -> Option<Instruction> {
    INSTRUCTIONS.get(input).cloned()
}

#[cfg(test)]
mod tests {
    use crate::instruction::{to_instruction, Instruction};

    #[test]
    pub fn known_names_are_decoded_correctly() {
        assert_eq!(
            to_instruction(b"AssertWorktopContainsByAmount"),
            Some(Instruction::AssertWorktopContainsByAmount)
        );
        assert_eq!(
            to_instruction(b"AssertWorktopContainsByIds"),
            Some(Instruction::AssertWorktopContainsByIds)
        );
        assert_eq!(
            to_instruction(b"AssertWorktopContains"),
            Some(Instruction::AssertWorktopContains)
        );
        assert_eq!(
            to_instruction(b"CallFunction"),
            Some(Instruction::CallFunction)
        );
        assert_eq!(to_instruction(b"CallMethod"), Some(Instruction::CallMethod));
        assert_eq!(
            to_instruction(b"CallNativeFunction"),
            Some(Instruction::CallNativeFunction)
        );
        assert_eq!(
            to_instruction(b"CallNativeMethod"),
            Some(Instruction::CallNativeMethod)
        );
        assert_eq!(
            to_instruction(b"ClearAuthZone"),
            Some(Instruction::ClearAuthZone)
        );
        assert_eq!(to_instruction(b"CloneProof"), Some(Instruction::CloneProof));
        assert_eq!(
            to_instruction(b"CreateProofFromAuthZoneByAmount"),
            Some(Instruction::CreateProofFromAuthZoneByAmount)
        );
        assert_eq!(
            to_instruction(b"CreateProofFromAuthZoneByIds"),
            Some(Instruction::CreateProofFromAuthZoneByIds)
        );
        assert_eq!(
            to_instruction(b"CreateProofFromAuthZone"),
            Some(Instruction::CreateProofFromAuthZone)
        );
        assert_eq!(
            to_instruction(b"CreateProofFromBucket"),
            Some(Instruction::CreateProofFromBucket)
        );
        assert_eq!(
            to_instruction(b"DropAllProofs"),
            Some(Instruction::DropAllProofs)
        );
        assert_eq!(to_instruction(b"DropProof"), Some(Instruction::DropProof));
        assert_eq!(
            to_instruction(b"PopFromAuthZone"),
            Some(Instruction::PopFromAuthZone)
        );
        assert_eq!(
            to_instruction(b"PublishPackage"),
            Some(Instruction::PublishPackage)
        );
        assert_eq!(
            to_instruction(b"PushToAuthZone"),
            Some(Instruction::PushToAuthZone)
        );
        assert_eq!(
            to_instruction(b"ReturnToWorktop"),
            Some(Instruction::ReturnToWorktop)
        );
        assert_eq!(
            to_instruction(b"TakeFromWorktopByAmount"),
            Some(Instruction::TakeFromWorktopByAmount)
        );
        assert_eq!(
            to_instruction(b"TakeFromWorktopByIds"),
            Some(Instruction::TakeFromWorktopByIds)
        );
        assert_eq!(
            to_instruction(b"TakeFromWorktop"),
            Some(Instruction::TakeFromWorktop)
        );
    }
}
