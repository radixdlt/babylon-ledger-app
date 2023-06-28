// SBOR type information

use core::option::Option;
use core::prelude::rust_2024::derive;

pub const TYPE_NONE: u8 = 0x00;
// primitive types
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

// composite types
pub const TYPE_ARRAY: u8 = 0x20;
pub const TYPE_TUPLE: u8 = 0x21;
pub const TYPE_ENUM: u8 = 0x22;
pub const TYPE_MAP: u8 = 0x23;

// Manifest custom types
pub const TYPE_ADDRESS: u8 = 0x80;
pub const TYPE_BUCKET: u8 = 0x81;
pub const TYPE_PROOF: u8 = 0x82;
pub const TYPE_EXPRESSION: u8 = 0x83;
pub const TYPE_BLOB: u8 = 0x84;
pub const TYPE_DECIMAL: u8 = 0x85;
pub const TYPE_PRECISE_DECIMAL: u8 = 0x86;
pub const TYPE_NON_FUNGIBLE_LOCAL_ID: u8 = 0x87;

pub const SIMPLE_TYPES: [u8; 20] = [
    TYPE_BOOL,
    TYPE_I8,
    TYPE_I16,
    TYPE_I32,
    TYPE_I64,
    TYPE_I128,
    TYPE_U8,
    TYPE_U16,
    TYPE_U32,
    TYPE_U64,
    TYPE_U128,
    TYPE_STRING,
    TYPE_ADDRESS,
    TYPE_BUCKET,
    TYPE_PROOF,
    TYPE_EXPRESSION,
    TYPE_BLOB,
    TYPE_DECIMAL,
    TYPE_PRECISE_DECIMAL,
    TYPE_NON_FUNGIBLE_LOCAL_ID,
];

// end of custom types
pub const ADDRESS_STATIC_LEN: u8 = 30; // 1 byte discriminator + 29 bytes address
pub const COMPONENT_LEN: u8 = 36;

pub const INTEGER_LEN: u8 = 8;
pub const UUID_LEN: u8 = 16;

pub const ID_LEN: u8 = 4;
pub const BUCKET_LEN: u8 = ID_LEN;
pub const PROOF_LEN: u8 = ID_LEN;
pub const BLOB_LEN: u8 = 32;
pub const DECIMAL_LEN: u8 = 32; // 256 bits
pub const PRECISE_DECIMAL_LEN: u8 = 64; // 512 bits

pub const TYPE_DATA_BUFFER_SIZE: usize = 256;

// Non-fungible local ID discriminators
pub const NFL_STRING: u8 = 0;
pub const NFL_INTEGER: u8 = 1;
pub const NFL_BYTES: u8 = 2;
pub const NFL_RUID: u8 = 3;
pub const NFL_MAX_STRING_LENG: usize = 64;

// Address discriminators
pub const ADDRESS_STATIC: u8 = 0;
pub const ADDRESS_NAMED: u8 = 1;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DecoderPhase {
    ReadingTypeId,
    ReadingElementTypeId,
    ReadingKeyTypeId,
    ReadingValueTypeId,
    ReadingLen,
    ReadingData,
    ReadingDiscriminator,
    ReadingNFLDiscriminator,
}

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug)]
pub struct TypeInfo {
    pub next_phases: &'static [DecoderPhase],
    pub fixed_len: u8,
    pub type_id: u8,
}

// Placeholder
const NONE_DECODING: [DecoderPhase; 1] = [DecoderPhase::ReadingTypeId];
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
const ENUM_DECODING: [DecoderPhase; 4] = [
    DecoderPhase::ReadingTypeId,
    DecoderPhase::ReadingDiscriminator,
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

const MAP_DECODING: [DecoderPhase; 5] = [
    DecoderPhase::ReadingTypeId,
    DecoderPhase::ReadingKeyTypeId,
    DecoderPhase::ReadingValueTypeId,
    DecoderPhase::ReadingLen,
    DecoderPhase::ReadingData,
];

const NON_FUNGIBLE_LOCAL_ID_ENCODING: [DecoderPhase; 4] = [
    DecoderPhase::ReadingTypeId,
    DecoderPhase::ReadingNFLDiscriminator,
    DecoderPhase::ReadingLen,
    DecoderPhase::ReadingData,
];

const ADDRESS_ENCODING: [DecoderPhase; 2] =
    [DecoderPhase::ReadingTypeId, DecoderPhase::ReadingData];

pub fn to_type_info(byte: u8) -> Option<TypeInfo> {
    match byte {
        TYPE_NONE => Some(TypeInfo {
            type_id: TYPE_NONE,
            next_phases: &NONE_DECODING,
            fixed_len: 0,
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
        TYPE_MAP => Some(TypeInfo {
            type_id: TYPE_MAP,
            next_phases: &MAP_DECODING,
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

        TYPE_ADDRESS => Some(TypeInfo {
            type_id: TYPE_ADDRESS,
            next_phases: &ADDRESS_ENCODING,
            fixed_len: ADDRESS_STATIC_LEN,
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
        TYPE_EXPRESSION => Some(TypeInfo {
            type_id: TYPE_EXPRESSION,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_BLOB => Some(TypeInfo {
            type_id: TYPE_BLOB,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: BLOB_LEN,
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
        TYPE_NON_FUNGIBLE_LOCAL_ID => Some(TypeInfo {
            type_id: TYPE_NON_FUNGIBLE_LOCAL_ID,
            next_phases: &NON_FUNGIBLE_LOCAL_ID_ENCODING, // Mix of fixed/variable len encoding
            fixed_len: 0,
        }),
        _ => None,
    }
}

pub fn to_type_name(type_id: u8) -> &'static [u8] {
    match type_id {
        TYPE_NONE => b"None",
        TYPE_BOOL => b"Bool",
        TYPE_I8 => b"I8",
        TYPE_I16 => b"I16",
        TYPE_I32 => b"I32",
        TYPE_I64 => b"I64",
        TYPE_I128 => b"I128",
        TYPE_U8 => b"U8",
        TYPE_U16 => b"U16",
        TYPE_U32 => b"U32",
        TYPE_U64 => b"U64",
        TYPE_U128 => b"U128",
        TYPE_STRING => b"String",
        TYPE_ARRAY => b"Array",
        TYPE_TUPLE => b"Tuple",
        TYPE_ENUM => b"Enum",
        TYPE_MAP => b"Map",
        TYPE_ADDRESS => b"Address",
        TYPE_BUCKET => b"Bucket",
        TYPE_PROOF => b"Proof",
        TYPE_EXPRESSION => b"Expression",
        TYPE_BLOB => b"Blob",
        TYPE_DECIMAL => b"Decimal",
        TYPE_PRECISE_DECIMAL => b"PreciseDecimal",
        TYPE_NON_FUNGIBLE_LOCAL_ID => b"NonFungibleLocalId",
        _ => b"(unknown)",
    }
}
