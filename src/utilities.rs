mod as_mut;
pub mod clone;
pub mod conversion;
pub mod version;

use crate::utilities::conversion::{to_hex, to_hex_str, to_str};
use core::str::from_utf8;
use nanos_ui::ui;

pub fn debug_arr(arr: &[u8]) {
    match to_hex(arr) {
        Ok(text) => debug_prepared_message(&text),
        Err(_) => debug("Input too long"),
    }
}

pub fn debug_u32(value: u32) {
    debug_prepared_message(&to_str(value));
}

pub fn debug_u32_hex(value: u32) {
    debug_prepared_message(&to_hex_str(value));
}

pub fn debug_prepared_message(message: &[u8]) {
    debug(from_utf8(message).unwrap());
}

pub fn debug(message: &str) {
    ui::MessageScroller::new(message).event_loop();
}
