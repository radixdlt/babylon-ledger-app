// SBOR type information

use core::option::Option;
use core::option::Option::{None, Some};
use core::prelude::rust_2024::derive;

// primitive types
pub const TYPE_UNIT: u8 = 0x00;
pub const TYPE_BOOL: u8 = 0x01;
pub const TYPE_I8: u8 = 0x02;
pub const TYPE_I16: u8 = 0x03;
pub const TYPE_I32: u8 = 0x04;
pub const TYPE_I64: u8 = 0x05;
pub const TYPE_I128: u8 = 0x06;
pub const TYPE_U8: u8 = 0x07;
pub const TYPE_U16: u8 = 0x08;
pub const TYPE_U32: u8 = 0x09;
pub const TYPE_U64: u8 = 0x0a;
pub const TYPE_U128: u8 = 0x0b;
pub const TYPE_STRING: u8 = 0x0c;

// struct and enum
pub const TYPE_STRUCT: u8 = 0x10;
pub const TYPE_ENUM: u8 = 0x11;

// composite types
pub const TYPE_ARRAY: u8 = 0x20;
pub const TYPE_TUPLE: u8 = 0x21;

// custom types (see https://raw.githubusercontent.com/radixdlt/radixdlt-scrypto/develop/radix-engine-interface/src/data/custom_type_id.rs
// for actual list of custom types)
pub const TYPE_PACKAGE_ADDRESS: u8 = 0x80;
pub const TYPE_COMPONENT_ADDRESS: u8 = 0x81;
pub const TYPE_RESOURCE_ADDRESS: u8 = 0x82;
pub const TYPE_SYSTEM_ADDRESS: u8 = 0x83;
pub const TYPE_COMPONENT: u8 = 0x90;
pub const TYPE_KEY_VALUE_STORE: u8 = 0x91;
pub const TYPE_BUCKET: u8 = 0x92;
pub const TYPE_PROOF: u8 = 0x93;
pub const TYPE_VAULT: u8 = 0x94;
pub const TYPE_EXPRESSION: u8 = 0xa0;
pub const TYPE_BLOB: u8 = 0xa1;
pub const TYPE_NON_FUNGIBLE_ADDRESS: u8 = 0xa2;
pub const TYPE_HASH: u8 = 0xb0;
pub const TYPE_ECDSA_SECP256K1_PUBIC_KEY: u8 = 0xb1;
pub const TYPE_ECDSA_SECP256K1_SIGNATURE: u8 = 0xb2;
pub const TYPE_EDDSA_ED25519_PUBIC_KEY: u8 = 0xb3;
pub const TYPE_EDDSA_ED25519_SIGNATURE: u8 = 0xb4;
pub const TYPE_DECIMAL: u8 = 0xb5;
pub const TYPE_PRECISE_DECIMAL: u8 = 0xb6;
pub const TYPE_NON_FUNGIBLE_ID: u8 = 0xb7;
// end of custom types
const ADDRESS_LEN: u8 = 27;
const COMPONENT_LEN: u8 = 36;
const KV_STORE_LEN: u8 = COMPONENT_LEN;
const VAULT_LEN: u8 = COMPONENT_LEN;

const ID_LEN: u8 = 4;
const BUCKET_LEN: u8 = ID_LEN;
const PROOF_LEN: u8 = ID_LEN;

const BLOB_LEN: u8 = 32;
const HASH_LEN: u8 = 32;
const SECP256K1_PUB_KEY_LEN: u8 = 33;
const SECP256K1_SIG_LEN: u8 = 65;
const ED25519_PUB_KEY_LEN: u8 = 32;
const ED25519_SIG_LEN: u8 = 64;

const DECIMAL_LEN: u8 = 32; // 256 bits
const PRECISE_DECIMAL_LEN: u8 = 64; // 512 bits

pub const TYPE_DATA_BUFFER_SIZE: usize = 256;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderPhase {
    ReadingTypeId,
    ReadingElementTypeId,
    ReadingLen,
    ReadingData,
    ReadingNameLen,
    ReadingNameData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeInfo {
    pub next_phases: &'static [DecoderPhase],
    pub fixed_len: u8,
    pub type_id: u8,
}

// id -> o (UNIT)
const UNIT_DECODING: [DecoderPhase; 1] = [DecoderPhase::ReadingTypeId];
// id -> data -> o (fixed size types)
const FIXED_LEN_DECODING: [DecoderPhase; 2] =
    [DecoderPhase::ReadingTypeId, DecoderPhase::ReadingData];
// id -> len -> data -> o (String, Struct, Tuple)
const VARIABLE_LEN_DECODING: [DecoderPhase; 3] = [
    DecoderPhase::ReadingTypeId,
    DecoderPhase::ReadingLen,
    DecoderPhase::ReadingData,
];
// id -> name (len -> data) -> len -> data -> o (Enum)
const ENUM_DECODING: [DecoderPhase; 5] = [
    DecoderPhase::ReadingTypeId,
    DecoderPhase::ReadingNameLen,
    DecoderPhase::ReadingNameData,
    DecoderPhase::ReadingLen,
    DecoderPhase::ReadingData,
];
// id -> element_id -> len -> data -> (Array, List, Set)
const LIST_DECODING: [DecoderPhase; 4] = [
    DecoderPhase::ReadingTypeId,
    DecoderPhase::ReadingElementTypeId,
    DecoderPhase::ReadingLen,
    DecoderPhase::ReadingData,
];

pub const NONE_TYPE_INFO: TypeInfo = TypeInfo {
    type_id: TYPE_UNIT,
    next_phases: &UNIT_DECODING,
    fixed_len: 0,
};

pub fn to_type_info(byte: u8) -> Option<TypeInfo> {
    match byte {
        TYPE_UNIT => Some(TypeInfo {
            type_id: TYPE_UNIT,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_BOOL => Some(TypeInfo {
            type_id: TYPE_BOOL,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_I8 => Some(TypeInfo {
            type_id: TYPE_I8,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_I16 => Some(TypeInfo {
            type_id: TYPE_I16,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 2,
        }),
        TYPE_I32 => Some(TypeInfo {
            type_id: TYPE_I32,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 4,
        }),
        TYPE_I64 => Some(TypeInfo {
            type_id: TYPE_I64,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 8,
        }),
        TYPE_I128 => Some(TypeInfo {
            type_id: TYPE_I128,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 16,
        }),
        TYPE_U8 => Some(TypeInfo {
            type_id: TYPE_U8,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_U16 => Some(TypeInfo {
            type_id: TYPE_U16,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 2,
        }),
        TYPE_U32 => Some(TypeInfo {
            type_id: TYPE_U32,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 4,
        }),
        TYPE_U64 => Some(TypeInfo {
            type_id: TYPE_U64,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 8,
        }),
        TYPE_U128 => Some(TypeInfo {
            type_id: TYPE_U128,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 16,
        }),
        TYPE_STRING => Some(TypeInfo {
            type_id: TYPE_STRING,
            next_phases: &VARIABLE_LEN_DECODING,
            fixed_len: 0,
        }),
        TYPE_STRUCT => Some(TypeInfo {
            type_id: TYPE_STRUCT,
            next_phases: &VARIABLE_LEN_DECODING,
            fixed_len: 0,
        }),

        TYPE_ENUM => Some(TypeInfo {
            type_id: TYPE_ENUM,
            next_phases: &ENUM_DECODING,
            fixed_len: 0,
        }),
        TYPE_ARRAY => Some(TypeInfo {
            type_id: TYPE_ARRAY,
            next_phases: &LIST_DECODING,
            fixed_len: 0,
        }),
        TYPE_TUPLE => Some(TypeInfo {
            type_id: TYPE_TUPLE,
            next_phases: &VARIABLE_LEN_DECODING,
            fixed_len: 0,
        }),

        // see https://raw.githubusercontent.com/radixdlt/radixdlt-scrypto/develop/radix-engine-interface/src/data/custom_value.rs
        // for necessary details (fixed/variable size, fixed length)
        TYPE_PACKAGE_ADDRESS => Some(TypeInfo {
            type_id: TYPE_PACKAGE_ADDRESS,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: ADDRESS_LEN,
        }),
        TYPE_COMPONENT_ADDRESS => Some(TypeInfo {
            type_id: TYPE_COMPONENT_ADDRESS,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: ADDRESS_LEN,
        }),
        TYPE_RESOURCE_ADDRESS => Some(TypeInfo {
            type_id: TYPE_RESOURCE_ADDRESS,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: ADDRESS_LEN,
        }),
        TYPE_SYSTEM_ADDRESS => Some(TypeInfo {
            type_id: TYPE_SYSTEM_ADDRESS,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: ADDRESS_LEN,
        }),
        TYPE_COMPONENT => Some(TypeInfo {
            type_id: TYPE_COMPONENT,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: COMPONENT_LEN,
        }),
        TYPE_KEY_VALUE_STORE => Some(TypeInfo {
            type_id: TYPE_KEY_VALUE_STORE,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: KV_STORE_LEN,
        }),
        TYPE_BUCKET => Some(TypeInfo {
            type_id: TYPE_BUCKET,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: BUCKET_LEN,
        }),
        TYPE_PROOF => Some(TypeInfo {
            type_id: TYPE_PROOF,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: PROOF_LEN,
        }),
        TYPE_VAULT => Some(TypeInfo {
            type_id: TYPE_VAULT,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: VAULT_LEN,
        }),
        TYPE_BLOB => Some(TypeInfo {
            type_id: TYPE_BLOB,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: BLOB_LEN,
        }),
        TYPE_HASH => Some(TypeInfo {
            type_id: TYPE_HASH,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: HASH_LEN,
        }),
        TYPE_ECDSA_SECP256K1_PUBIC_KEY => Some(TypeInfo {
            type_id: TYPE_ECDSA_SECP256K1_PUBIC_KEY,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: SECP256K1_PUB_KEY_LEN,
        }),
        TYPE_ECDSA_SECP256K1_SIGNATURE => Some(TypeInfo {
            type_id: TYPE_ECDSA_SECP256K1_SIGNATURE,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: SECP256K1_SIG_LEN,
        }),
        TYPE_EDDSA_ED25519_PUBIC_KEY => Some(TypeInfo {
            type_id: TYPE_EDDSA_ED25519_PUBIC_KEY,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: ED25519_PUB_KEY_LEN,
        }),
        TYPE_EDDSA_ED25519_SIGNATURE => Some(TypeInfo {
            type_id: TYPE_EDDSA_ED25519_SIGNATURE,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: ED25519_SIG_LEN,
        }),
        TYPE_DECIMAL => Some(TypeInfo {
            type_id: TYPE_DECIMAL,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: DECIMAL_LEN,
        }),
        TYPE_PRECISE_DECIMAL => Some(TypeInfo {
            type_id: TYPE_PRECISE_DECIMAL,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: PRECISE_DECIMAL_LEN,
        }),

        TYPE_EXPRESSION => Some(TypeInfo {
            type_id: TYPE_EXPRESSION,
            next_phases: &VARIABLE_LEN_DECODING,
            fixed_len: 0,
        }),
        TYPE_NON_FUNGIBLE_ADDRESS => Some(TypeInfo {
            type_id: TYPE_NON_FUNGIBLE_ADDRESS,
            next_phases: &VARIABLE_LEN_DECODING,
            fixed_len: 0,
        }),
        TYPE_NON_FUNGIBLE_ID => Some(TypeInfo {
            type_id: TYPE_NON_FUNGIBLE_ID,
            next_phases: &VARIABLE_LEN_DECODING,
            fixed_len: 0,
        }),

        _ => None,
    }
}
