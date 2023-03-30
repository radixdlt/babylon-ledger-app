use crate::bech32::network::*;
use crate::print::tty::TTY;
use crate::sbor_decoder::STACK_DEPTH;
use staticvec::StaticVec;

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
    pub tty: &'a mut dyn TTY,
}

impl<'a> ParameterPrinterState<'a> {
    pub fn new(network_id: NetworkId, tty: &'a mut dyn TTY) -> Self {
        Self {
            data: StaticVec::new(),
            stack: StaticVec::new(),
            nesting_level: 0,
            network_id,
            tty,
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
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
}
