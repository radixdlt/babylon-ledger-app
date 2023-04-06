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

pub const PARAMETER_AREA_SIZE: usize = 65;

pub struct ParameterPrinterState<'a> {
    pub data: StaticVec<u8, { PARAMETER_AREA_SIZE }>,
    pub stack: StaticVec<ValueState, { STACK_DEPTH as usize }>,
    pub nesting_level: u8,
    pub network_id: NetworkId,
    tty: Option<&'a mut dyn TTY>,
}

impl TTY for ParameterPrinterState<'_> {
    fn start(&mut self) {
        debug_print("print_text: tty start\n");
        match self.tty {
            Some(_) => debug_print("print_text: tty present"),
            None => debug_print("print_text: tty not present"),
        };

//        self.tty.as_mut().expect("TTY not set").start();
    }

    fn end(&mut self) {
        debug_print("print_text: tty end\n");
        //self.tty.as_mut().expect("TTY not set").end();
    }

    fn print_byte(&mut self, byte: u8) {
        debug_print("print_text: tty print_byte\n");
        debug_print_byte(byte.clone());
        //self.tty.as_mut().expect("TTY not set").print_byte(byte);
    }

    fn print_text(&mut self, text: &[u8]) {
        debug_print("print_text: tty print_text\n");
        debug_prepared_message(text);
        //self.tty.as_mut().expect("TTY not set").print_text(text);
    }
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

    pub fn print_data_as_text(&mut self) {
        self.tty.as_mut().expect("TTY not set").print_text(self.data.as_slice());
    }

    pub fn print_data_as_hex(&mut self) {
        for &byte in self.data.as_slice() {
            self.tty.as_mut().expect("TTY not set").print_hex_byte(byte);
        }
    }

    pub fn print_data_as_hex_slice(&mut self, range: Range<usize>) {
        for &byte in &self.data.as_slice()[range] {
            self.tty.as_mut().expect("TTY not set").print_hex_byte(byte);
        }
    }
}
