use crate::bech32::network::*;
use crate::print::tty::TTY;
use crate::sbor_decoder::STACK_DEPTH;
use crate::static_vec::StaticVec;
use core::ops::Range;

#[repr(packed)]
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

#[cfg(target_os = "nanos")]
pub const PARAMETER_AREA_SIZE: usize = 128;
#[cfg(not(target_os = "nanos"))]
pub const PARAMETER_AREA_SIZE: usize = 128;

#[cfg(target_os = "nanos")]
pub const DISPLAY_SIZE: usize = 256;    // Use smaller buffer for Nano S
#[cfg(target_os = "nanosplus")]
pub const DISPLAY_SIZE: usize = 1024;   // Nano S+ and Nano X have larger screens and more memory
#[cfg(target_os = "nanox")]
pub const DISPLAY_SIZE: usize = 1024;
#[cfg(not(any(target_os = "nanos", target_os = "nanox", target_os = "nanosplus")))]
pub const DISPLAY_SIZE: usize = 2048;   // For testing on desktop

pub struct ParameterPrinterState {
    pub display: StaticVec<u8, { DISPLAY_SIZE }>,
    pub data: StaticVec<u8, { PARAMETER_AREA_SIZE }>,
    pub stack: StaticVec<ValueState, { STACK_DEPTH as usize }>,
    pub nesting_level: u8,
    pub network_id: NetworkId,
    tty: TTY,
}

impl ParameterPrinterState {
    pub fn new(network_id: NetworkId, tty: TTY) -> Self {
        Self {
            data: StaticVec::new(),
            stack: StaticVec::new(),
            display: StaticVec::new(),
            nesting_level: 0,
            network_id,
            tty: tty,
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }

    pub fn set_tty(&mut self, tty: TTY) {
        self.tty = tty;
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

    pub fn display_hex_string(&mut self, data: &[u8]) {
        self.start();
        self.data.clear();
        self.data.extend_from_slice(data);
        self.print_data_as_hex();
        self.end();
    }

    const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

    pub fn print_data_as_text(&mut self) {
        self.display.extend_from_slice(self.data.as_slice());
    }

    pub fn print_data_as_hex(&mut self) {
        for &byte in self.data.as_slice() {
            self.display
                .push(Self::HEX_DIGITS[((byte >> 4) & 0x0F) as usize]);
            self.display.push(Self::HEX_DIGITS[(byte & 0x0F) as usize]);
        }
    }

    pub fn print_data_as_hex_slice(&mut self, range: Range<usize>) {
        for &byte in &self.data.as_slice()[range] {
            self.display
                .push(Self::HEX_DIGITS[((byte >> 4) & 0x0F) as usize]);
            self.display.push(Self::HEX_DIGITS[(byte & 0x0F) as usize]);
        }
    }

    pub fn print_space(&mut self) {
        self.print_byte(b' ');
    }

    pub fn print_hex_byte(&mut self, byte: u8) {
        self.print_byte(Self::HEX_DIGITS[((byte >> 4) & 0x0F) as usize]);
        self.print_byte(Self::HEX_DIGITS[(byte & 0x0F) as usize]);
    }

    pub fn start(&mut self) {
        self.display.clear();
    }

    pub fn end(&mut self) {
        (self.tty.show_message)(self.display.as_slice());
    }

    pub fn print_byte(&mut self, byte: u8) {
        self.display.push(byte);
    }

    pub fn print_text(&mut self, text: &[u8]) {
        for &byte in text {
            self.print_byte(byte);
        }
    }
}
