// __Transaction Intent SBOR decoder and instruction extractor__
//
// The design of the SBOR decoder is significantly influenced by the need to limit resource
// consumption as much as possible. The performance of the solution is not an issue, since there
// is no need to decode significant amount of SBOR data. The SBOR decoder is able to decode
// arbitrarily long SBOR input, the only limitation is the level of the nesting of the SBOR
// structures. This limit can be changed by changing `STACK_DEPH` constant in `sbor_decoder.rs`.
//
// Decoding SBOR is rather straightforward task. All encoded data types follow one of the following
// of patterns in the layout of encoded data:
//
// Used by fixed length types, number of data bytes defined by the type:
//         `[id byte], [data bytes]`
// String uses variable length encoding:
//         `[id byte], [len], [data bytes]`
// Len is also encoded using variable number of bytes.

// TODO: start of outdated information
// Struct and Tuple use very similar pattern, except instead of raw bytes, each element is encoded
// as SBOR, basically creating nested SBOR element:
//         `[id byte], [len], [encoded element 0], ... [encoded element N]`
// Array uses similar pattern, but type of elements is encoded only once:
//         `[id byte], [element id byte], [len], [element data 0], ... [element data 1]`
// Elements are encoded as SBOR except first byte of each element encoding (type id) since it is
// identical for all elements.
// Finally ENUM uses separate layout:
//         `[id byte], [name len], [name bytes], [len], [encoded element 0], ... [encoded element N]`
// Name contains name of the encoded ENUM variant, followed by payload specific to that variant.
//
// Custom types are using one of the above encodings - either fixed length encoding (like fixed
// length types) or variable length encoding (like Struct/Tuple or String).
// TODO: end of outdated information ^^^^

// This implementation of the SBOR decoder uses dynamically reconfigurable state machine for
// decoding. Initial state of all variants of the data layout is the same - retrieving type id byte.
// So, decoder loads type ID and then selects which state machine will be used to decode remaining
// data depenging on actual type. When decoding nested SBOR element is necessary, then current state
// is preserved and fresh new one is pushed to the stack and the deoder restarted from the initial
// state. This approach works quite well and only somewhat tricky part is the handling the end of
// the decoding, since it may require ending of decoding not only at current nesting level, but
// at previous level as well (if the decoded element is the last element in the struct/tuple/array)
// and so on and so forth up to the initial nesting level (0).
//
// Main advantage of this implementation is that it does not require storing decoded data (except
// types) to decode arbitrarily complex SBOR. Hence it has very low resource footpring.
// Nevertheless, some parts of data still need to be stored, for example, Transaction Intent
// instructions and relevant parameters. To solve this problem, SBOR decoder generates events
// which mark start/end of each element as well as end of decoding of `len` fields and data bytes.
//
// The __Instruction Extractor__ uses these events to decode higher level structure - Transaction Intent
// and extract individual instructions and their parameters. The Instruction Extractor uses yet
// another state machine to skip irrelevant parts of the Transaction Intent, extracts and prepares
// for further processing parts of instructions - start and end of the instuction, instruction
// parameters, etc. These parts are also submitted as events to the provided listener, effectively
// decoupling Instruction Extractor from the code which will use extracted instructions and
// their parameters.

#![feature(prelude_2024)]
#![feature(const_mut_refs)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]
pub mod bech32;
pub mod decoder_error;
pub mod digest;
pub mod instruction;
pub mod instruction_extractor;
pub mod math;
pub mod print;
pub mod sbor_decoder;
pub mod static_vec;
pub mod type_info;
pub mod utilities;

pub mod debug;

#[cfg(test)]
pub mod si_test_data;
#[cfg(test)]
pub mod tx_intent_test_data;
#[cfg(test)]
pub mod tx_intent_test_data_gen;
