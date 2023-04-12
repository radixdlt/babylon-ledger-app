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
pub const ADDRESS_LEN: u8 = 27; // 1 byte discriminator + 26 bytes address
pub const COMPONENT_LEN: u8 = 36;

pub const INTEGER_LEN: u8 = 8;
pub const UUID_LEN: u8 = 16;

pub const ID_LEN: u8 = 4;
const BUCKET_LEN: u8 = ID_LEN;
const PROOF_LEN: u8 = ID_LEN;
const BLOB_LEN: u8 = 32;
const DECIMAL_LEN: u8 = 32; // 256 bits
const PRECISE_DECIMAL_LEN: u8 = 64; // 512 bits

pub const TYPE_DATA_BUFFER_SIZE: usize = 256;

// Non-fungible local ID discriminators
pub const NFL_STRING: u8 = 0;
pub const NFL_INTEGER: u8 = 1;
pub const NFL_BYTES: u8 = 2;
pub const NFL_UUID: u8 = 3;

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

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TypeKind {
    None,
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    String,
    Array,
    Tuple,
    Enum,
    Map,
    Address,
    Bucket,
    Proof,
    Expression,
    Blob,
    Decimal,
    PreciseDecimal,
    NonFungibleLocalId,
}

#[derive(Copy, Clone, Debug)]
pub struct TypeInfo {
    pub next_phases: &'static [DecoderPhase],
    pub fixed_len: u8,
    pub type_id: u8,
    pub type_kind: TypeKind,
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

pub const NONE_TYPE_INFO: TypeInfo = TypeInfo {
    type_id: TYPE_NONE,
    type_kind: TypeKind::None,
    next_phases: &NONE_DECODING,
    fixed_len: 0,
};

pub fn to_type_info(byte: u8) -> Option<TypeInfo> {
    match byte {
        TYPE_BOOL => Some(TypeInfo {
            type_id: TYPE_BOOL,
            type_kind: TypeKind::Bool,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_I8 => Some(TypeInfo {
            type_id: TYPE_I8,
            type_kind: TypeKind::I8,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_I16 => Some(TypeInfo {
            type_id: TYPE_I16,
            type_kind: TypeKind::I16,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 2,
        }),
        TYPE_I32 => Some(TypeInfo {
            type_id: TYPE_I32,
            type_kind: TypeKind::I32,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 4,
        }),
        TYPE_I64 => Some(TypeInfo {
            type_id: TYPE_I64,
            type_kind: TypeKind::I64,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 8,
        }),
        TYPE_I128 => Some(TypeInfo {
            type_id: TYPE_I128,
            type_kind: TypeKind::I128,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 16,
        }),
        TYPE_U8 => Some(TypeInfo {
            type_id: TYPE_U8,
            type_kind: TypeKind::U8,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_U16 => Some(TypeInfo {
            type_id: TYPE_U16,
            type_kind: TypeKind::U16,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 2,
        }),
        TYPE_U32 => Some(TypeInfo {
            type_id: TYPE_U32,
            type_kind: TypeKind::U32,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 4,
        }),
        TYPE_U64 => Some(TypeInfo {
            type_id: TYPE_U64,
            type_kind: TypeKind::U64,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 8,
        }),
        TYPE_U128 => Some(TypeInfo {
            type_id: TYPE_U128,
            type_kind: TypeKind::U128,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 16,
        }),
        TYPE_STRING => Some(TypeInfo {
            type_id: TYPE_STRING,
            type_kind: TypeKind::String,
            next_phases: &VARIABLE_LEN_DECODING,
            fixed_len: 0,
        }),
        TYPE_MAP => Some(TypeInfo {
            type_id: TYPE_MAP,
            type_kind: TypeKind::Map,
            next_phases: &MAP_DECODING,
            fixed_len: 0,
        }),

        TYPE_ENUM => Some(TypeInfo {
            type_id: TYPE_ENUM,
            type_kind: TypeKind::Enum,
            next_phases: &ENUM_DECODING,
            fixed_len: 0,
        }),
        TYPE_ARRAY => Some(TypeInfo {
            type_id: TYPE_ARRAY,
            type_kind: TypeKind::Array,
            next_phases: &LIST_DECODING,
            fixed_len: 0,
        }),
        TYPE_TUPLE => Some(TypeInfo {
            type_id: TYPE_TUPLE,
            type_kind: TypeKind::Tuple,
            next_phases: &VARIABLE_LEN_DECODING,
            fixed_len: 0,
        }),

        TYPE_ADDRESS => Some(TypeInfo {
            type_id: TYPE_ADDRESS,
            type_kind: TypeKind::Address,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: ADDRESS_LEN,
        }),

        TYPE_BUCKET => Some(TypeInfo {
            type_id: TYPE_BUCKET,
            type_kind: TypeKind::Bucket,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: BUCKET_LEN,
        }),
        TYPE_PROOF => Some(TypeInfo {
            type_id: TYPE_PROOF,
            type_kind: TypeKind::Proof,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: PROOF_LEN,
        }),
        TYPE_EXPRESSION => Some(TypeInfo {
            type_id: TYPE_EXPRESSION,
            type_kind: TypeKind::Expression,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: 1,
        }),
        TYPE_BLOB => Some(TypeInfo {
            type_id: TYPE_BLOB,
            type_kind: TypeKind::Blob,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: BLOB_LEN,
        }),
        TYPE_DECIMAL => Some(TypeInfo {
            type_id: TYPE_DECIMAL,
            type_kind: TypeKind::Decimal,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: DECIMAL_LEN,
        }),
        TYPE_PRECISE_DECIMAL => Some(TypeInfo {
            type_id: TYPE_PRECISE_DECIMAL,
            type_kind: TypeKind::PreciseDecimal,
            next_phases: &FIXED_LEN_DECODING,
            fixed_len: PRECISE_DECIMAL_LEN,
        }),
        TYPE_NON_FUNGIBLE_LOCAL_ID => Some(TypeInfo {
            type_id: TYPE_NON_FUNGIBLE_LOCAL_ID,
            type_kind: TypeKind::NonFungibleLocalId,
            next_phases: &NON_FUNGIBLE_LOCAL_ID_ENCODING, // Mix of fixed/variable len encoding
            fixed_len: 0,
        }),
        _ => None,
    }
}

pub fn to_type_name(type_id: u8) -> &'static [u8] {
    to_type_info(type_id)
        .map(|info| to_kind_name(info.type_kind))
        .unwrap_or(b"(unknown)")
}

pub fn to_kind_name(kind: TypeKind) -> &'static [u8] {
    match kind {
        TypeKind::None => b"None",
        TypeKind::Bool => b"Bool",
        TypeKind::I8 => b"I8",
        TypeKind::I16 => b"I16",
        TypeKind::I32 => b"I32",
        TypeKind::I64 => b"I64",
        TypeKind::I128 => b"I128",
        TypeKind::U8 => b"U8",
        TypeKind::U16 => b"U16",
        TypeKind::U32 => b"U32",
        TypeKind::U64 => b"U64",
        TypeKind::U128 => b"U128",
        TypeKind::String => b"String",
        TypeKind::Array => b"Array",
        TypeKind::Tuple => b"Tuple",
        TypeKind::Enum => b"Enum",
        TypeKind::Map => b"Map",
        TypeKind::Address => b"Address",
        TypeKind::Bucket => b"Bucket",
        TypeKind::Proof => b"Proof",
        TypeKind::Expression => b"Expression",
        TypeKind::Blob => b"Blob",
        TypeKind::Decimal => b"Decimal",
        TypeKind::PreciseDecimal => b"PreciseDecimal",
        TypeKind::NonFungibleLocalId => b"NonFungibleLocalId",
    }
}
