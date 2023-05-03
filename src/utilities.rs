pub mod conversion;
pub mod version;

use crate::utilities::conversion::{to_hex, to_hex_str, to_str};
use core::str::from_utf8;
use nanos_sdk::testing::debug_print;

pub fn debug_arr(arr: &[u8]) {
    match to_hex(arr) {
        Ok(text) => debug_prepared_message(&text),
        Err(_) => debug_print("Input too long"),
    }
}

pub fn debug_u32(value: u32) {
    debug_prepared_message(&to_str(value));
}

pub fn debug_u32_hex(value: u32) {
    debug_prepared_message(&to_hex_str(value));
}

pub fn debug_prepared_message(message: &[u8]) {
    debug_print(from_utf8(message).unwrap());
    debug_print("\n");
}

pub fn debug_print_byte(byte: u8) {
    let mut buffer = [0u8; 1];
    buffer[0] = byte;
    debug_print(from_utf8(&buffer).unwrap());
}
