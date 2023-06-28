use core::ops::Range;

use crate::bech32::network::*;
use crate::print::primitives::print_u32;
use crate::print::tty::TTY;
use crate::print::tx_summary_detector::Address;
use crate::sbor_decoder::STACK_DEPTH;
use crate::static_vec::StaticVec;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct ValueState {
    pub main_type_id: u8,    // Outer type ID at current nesting level
    pub key_type_id: u8,     // Map key type ID; Resource ID for HRP; Discriminator for enums
    pub element_type_id: u8, // Map value type ID; Array/Tuple/Enum - element type ID
    pub flip_flop: bool,     // Used for printing map keys and values
}

impl ValueState {
    pub fn new(main_type_id: u8) -> Self {
        Self {
            main_type_id,
            key_type_id: 0,
            element_type_id: 0,
            flip_flop: false,
        }
    }
}

impl Default for ValueState {
    fn default() -> Self {
        Self {
            main_type_id: 0,
            key_type_id: 0,
            element_type_id: 0,
            flip_flop: false,
        }
    }
}

pub const PARAMETER_AREA_SIZE: usize = 256;

#[cfg(target_os = "nanos")]
pub const DISPLAY_SIZE: usize = 256; // Use smaller buffer for Nano S
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
pub const DISPLAY_SIZE: usize = 1024; // Nano S+/X have more memory
#[cfg(not(any(target_os = "nanos", target_os = "nanox", target_os = "nanosplus")))]
pub const DISPLAY_SIZE: usize = 2048;

pub const TITLE_SIZE: usize = 32;

pub struct ParameterPrinterState<T: Copy> {
    pub display: StaticVec<u8, { DISPLAY_SIZE }>,
    pub data: StaticVec<u8, { PARAMETER_AREA_SIZE }>,
    pub title: StaticVec<u8, { TITLE_SIZE }>,
    pub stack: StaticVec<ValueState, { STACK_DEPTH as usize }>,
    pub nesting_level: u8,
    pub network_id: NetworkId,
    pub show_instructions: bool,
    tty: TTY<T>,
}

const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

#[inline(always)]
fn lower_as_hex(byte: u8) -> u8 {
    HEX_DIGITS[(byte & 0x0F) as usize]
}

#[inline(always)]
fn upper_as_hex(byte: u8) -> u8 {
    HEX_DIGITS[((byte >> 4) & 0x0F) as usize]
}

impl<T: Copy> ParameterPrinterState<T> {
    pub fn new(network_id: NetworkId, tty: TTY<T>) -> Self {
        Self {
            data: StaticVec::new(0),
            stack: StaticVec::new(ValueState::default()),
            display: StaticVec::new(0),
            title: StaticVec::new(0),
            nesting_level: 0,
            network_id,
            show_instructions: true,
            tty: tty,
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }

    pub fn set_show_instructions(&mut self, show: bool) {
        self.show_instructions = show;
    }

    pub fn set_tty(&mut self, tty: TTY<T>) {
        self.tty = tty;
    }

    pub fn get_tty(&self) -> &TTY<T> {
        &self.tty
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.stack.clear();
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn active_state(&mut self) -> &mut ValueState {
        self.stack.last_mut().expect("Stack can't be empty")
    }

    pub fn print_data_as_text(&mut self) {
        self.display.extend_from_slice(self.data.as_slice());
    }

    pub fn print_data_as_hex(&mut self) {
        for &byte in self.data.as_slice() {
            self.display.push(upper_as_hex(byte));
            self.display.push(lower_as_hex(byte));
        }
    }

    pub fn print_data_as_hex_slice(&mut self, range: Range<usize>) {
        for &byte in &self.data.as_slice()[range] {
            self.display.push(upper_as_hex(byte));
            self.display.push(lower_as_hex(byte));
        }
    }

    pub fn print_space(&mut self) {
        self.print_byte(b' ');
    }

    pub fn print_hex_byte(&mut self, byte: u8) {
        self.print_byte(upper_as_hex(byte));
        self.print_byte(lower_as_hex(byte));
    }

    pub fn start(&mut self) {
        self.display.clear();
        self.title.clear();
    }

    pub fn end(&mut self) {
        if self.show_instructions {
            (self.tty.show_message)(
                &mut self.tty.data,
                self.title.as_slice(),
                self.display.as_slice(),
            );
        }
    }

    pub fn print_byte(&mut self, byte: u8) {
        self.display.push(byte);
    }

    pub fn print_text(&mut self, text: &[u8]) {
        self.display.extend_from_slice(text);
    }

    pub fn print_static_address(&mut self) {
        let mut address = Address::new();
        address.copy_from_slice(&self.data.as_slice()[1..]);
        self.data.clear();

        address.format(&mut self.data, self.network_id);

        self.display.extend_from_slice(b"Address(");
        self.display.extend_from_slice(self.data.as_slice());
        self.display.push(b')');
    }

    pub fn print_named_address(&mut self) {
        let mut array: [u8; 4] = [0u8; 4];
        array.copy_from_slice(&self.data.as_slice()[1..]);
        let address = u32::from_be_bytes(array);

        self.data.clear();
        print_u32(&mut self.data, address);

        self.display.extend_from_slice(b"Address(");
        self.display.extend_from_slice(self.data.as_slice());
        self.display.extend_from_slice(b"u32)");
    }

    pub fn format_address(&mut self, address: &Address) -> &[u8] {
        self.data.clear();
        address.format(&mut self.data, self.network_id);
        self.data.as_slice()
    }
}
