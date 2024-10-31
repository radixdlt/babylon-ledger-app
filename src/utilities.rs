pub mod version;

// Useful helper function which prints current stack position.
// When necessary, uncomment code below and make module sbor::print::primitives public to make code compile

/*
#[cfg(debug_assertions)]
pub mod debug {
    use core::str::from_utf8;
    use ledger_device_sdk::testing::debug_print;
    use ledger_device_sdk::ui::gadgets::clear_screen;
    use sbor::static_vec::StaticVec;
    use sbor::utilities::conversion::{to_hex_str, to_str};

    use crate::ui::single_message::SingleMessage;

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
