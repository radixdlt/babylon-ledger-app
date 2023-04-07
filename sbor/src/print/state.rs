use core::ops::Range;
use crate::bech32::network::*;
use crate::print::tty::TTY;
use crate::sbor_decoder::STACK_DEPTH;
use staticvec::StaticVec;
use crate::debug::{debug_prepared_message, debug_print, debug_print_byte};

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

pub const PARAMETER_AREA_SIZE: usize = 128;

pub struct ParameterPrinterState<'a> {
    pub data: StaticVec<u8, { PARAMETER_AREA_SIZE }>,
    pub stack: StaticVec<ValueState, { STACK_DEPTH as usize }>,
    pub nesting_level: u8,
    pub network_id: NetworkId,
    tty: Option<&'a mut dyn TTY>,
}

impl<'a> ParameterPrinterState<'a> {
    pub const fn new(network_id: NetworkId) -> Self {
        Self {
            data: StaticVec::new(),
            stack: StaticVec::new(),
            nesting_level: 0,
            network_id,
            tty: None,
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }

    pub fn set_tty(&mut self, tty: &'a mut dyn TTY) {
        self.tty = Some(tty);
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

    const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

    pub fn print_data_as_text(&mut self) {
        debug_print("print_data_as_text: ");
        debug_prepared_message(self.data.as_slice());
        debug_print("\n");
        //self.tty.as_mut().expect("TTY not set").print_text(self.data.as_slice());
    }

    pub fn print_data_as_hex(&mut self) {
        debug_print("print_data_as_hex\n");
        // for &byte in self.data.as_slice() {
        //     self.tty.as_mut().expect("TTY not set").print_hex_byte(byte);
        // }
    }

    pub fn print_data_as_hex_slice(&mut self, range: Range<usize>) {
        debug_print("print_data_as_hex_slice\n");
        // for &byte in &self.data.as_slice()[range] {
        //     self.tty.as_mut().expect("TTY not set").print_hex_byte(byte);
        // }
    }

    pub fn print_space(&mut self) {
        self.print_byte(b' ');
    }

    pub fn print_hex_byte(&mut self, byte: u8) {
        self.print_byte(Self::HEX_DIGITS[((byte >> 4) & 0x0F) as usize]);
        self.print_byte(Self::HEX_DIGITS[(byte & 0x0F) as usize]);
    }

    pub fn start(&mut self) {
        debug_print("tty start\n");
//        self.tty.as_mut().expect("TTY not set").start();
    }

    pub fn end(&mut self) {
        debug_print("tty end\n");
        //self.tty.as_mut().expect("TTY not set").end();
    }

    pub fn print_byte(&mut self, byte: u8) {
        debug_print("tty print_byte: ");
        debug_print_byte(byte.clone());
        //self.tty.as_mut().expect("TTY not set").print_byte(byte);
    }

    pub fn print_text(&mut self, text: &[u8]) {
        debug_print("tty print_text: ");
        debug_prepared_message(text);
        //self.tty.as_mut().expect("TTY not set").print_text(text);
    }
}
