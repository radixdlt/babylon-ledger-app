use staticvec::StaticVec;
use crate::bech32::hrp::*;
use crate::bech32::network::*;

pub struct ParameterPrinterState {
    // pub data: [u8; Self::PARAMETER_AREA_SIZE],
    // pub data_counter: u8,
    pub data: StaticVec<u8, {Self::PARAMETER_AREA_SIZE}>,
    pub discriminator: u8,
    pub inner_discriminator: u8,
    pub flip_flop: bool,
    pub network_id: NetworkId,
    pub resource_id: HrpType,
    pub phase: u8,
    pub expected_len: u32,
    pub nesting_level: u8,
    pub manifest_discriminator: u8,
}

impl ParameterPrinterState {
    pub const PARAMETER_AREA_SIZE: usize = 128;

    pub fn new(network_id: NetworkId) -> Self {
        Self {
            data: StaticVec::new(),
            //data_counter: 0,
            discriminator: 0,
            inner_discriminator: 0,
            flip_flop: false,
            network_id,
            resource_id: HrpType::Autodetect,
            phase: 0,
            expected_len: 0,
            nesting_level: 0,
            manifest_discriminator: 0,
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.discriminator = 0;
        self.inner_discriminator = 0;
        self.flip_flop = false;
        self.resource_id = HrpType::Autodetect;
        self.phase = 0;
        self.expected_len = 0;
        // Nesting level and manifest_discriminator is preserved
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn push_byte_for_string(&mut self, byte: u8) {
        self.push_byte(byte);

        // Add '...' at the end of truncated string.
        if self.data.len() == ParameterPrinterState::PARAMETER_AREA_SIZE - 2 {
            self.data.pop();
            self.push_byte(b'.');
            self.push_byte(b'.');
            self.push_byte(b'.');
        }
    }
}
