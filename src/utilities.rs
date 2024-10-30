// The code in this file is used only for debugging. Under normal circumstances, it is not invoked nor referenced.
#![allow(dead_code)]
#[cfg(debug_assertions)]
use core::str::from_utf8;

#[cfg(debug_assertions)]
use ledger_device_sdk::testing::debug_print;
#[cfg(debug_assertions)]
use sbor::utilities::conversion::{to_hex_str, to_str};

pub mod version;

#[cfg(debug_assertions)]
pub fn debug_u32(value: u32) {
    debug_prepared_message(&to_str(value));
}

#[cfg(debug_assertions)]
pub fn debug_u32_hex(value: u32) {
    debug_prepared_message(&to_hex_str(value));
}

#[cfg(debug_assertions)]
pub fn debug_prepared_message(message: &[u8]) {
    debug_print(from_utf8(message).unwrap());
    debug_print("\n");
}

#[cfg(debug_assertions)]
pub fn debug_print_byte(byte: u8) {
    let mut buffer = [0u8; 1];
    buffer[0] = byte;
    debug_print(from_utf8(&buffer).unwrap());
}
/*
// Useful helper function which prints current stack position
// requires making module sbor::print::primitives public

#[cfg(debug_assertions)]
pub mod debug {
    use core::str::from_utf8;
    use nanos_ui::ui::clear_screen;
    use sbor::static_vec::StaticVec;
    use crate::ui::single_message::SingleMessage;

    pub fn display_memory(lead_byte: u8) {
        clear_screen();
        let mut number = StaticVec::<u8, 16>::new(0);
        let ptr = &number as *const StaticVec<u8, 16>;
        number.push(lead_byte);
        sbor::print::primitives::print_u32(&mut number, ptr as u32);
        SingleMessage::new(from_utf8(number.as_slice()).unwrap()).show_and_wait();
    }
}
 */

macro_rules! dbg_print {
    ($($args:tt)*) => {

        let mut msg = [0u8; 300];

        match format_no_std::show(
            &mut msg,
            format_args!($($args)*)
        ) {
            Ok(s) => {
                debug_print(s);
            }
            Err(_) => {
                debug_print("🙅‍♀️ Too long string literal message or args to format sent to macro dbg_print!");
            }
        }

    }
}
pub(crate) use dbg_print;
