use crate::bech32::encoder::Bech32;
use crate::bech32::hrp::{hrp_prefix, hrp_suffix};
use crate::bech32::network::NetworkId;
use crate::math::StaticVec;
use crate::type_info::ADDRESS_STATIC_LEN;

/// Reusable storage for various types of addresses. All addresses share same length.
/// For convenience in use with stream decoders, additional boolean flag is maintained.
#[derive(Copy, Clone, Debug)]
pub struct Address {
    address: [u8; ADDRESS_STATIC_LEN as usize],
    is_set: bool,
}

impl Address {
    const XRD_ADDRESS: Address = Self {
        address: [
            93, 166, 99, 24, 198, 49, 140, 97, 245, 166, 27, 76, 99, 24, 198, 49, 140, 247, 148,
            170, 141, 41, 95, 20, 230, 49, 140, 99, 24, 198,
        ],
        is_set: true,
    };

    pub fn new() -> Self {
        Self {
            address: [0; ADDRESS_STATIC_LEN as usize],
            is_set: false,
        }
    }

    pub fn from_array(src: [u8; ADDRESS_STATIC_LEN as usize]) -> Self {
        Self {
            address: src,
            is_set: true,
        }
    }

    pub fn as_ref(&self) -> &[u8] {
        &self.address
    }

    pub fn is_set(&self) -> bool {
        self.is_set
    }

    pub fn is_xrd(&self) -> bool {
        self.is_same(&Self::XRD_ADDRESS)
    }

    pub fn set_entity_type(&mut self, entity_type: u8) {
        self.address[0] = entity_type;
    }

    pub fn copy_from_slice(&mut self, src: &[u8]) {
        self.address.copy_from_slice(src);
        self.is_set = true;
    }

    pub fn copy_from_other(&mut self, other: &Address) {
        self.is_set = other.is_set;

        if self.is_set {
            self.address.copy_from_slice(&other.address);
        }
    }

    pub fn reset(&mut self) {
        self.is_set = false;
        self.address = [0; ADDRESS_STATIC_LEN as usize];
    }

    pub fn is_same(&self, other: &Address) -> bool {
        if self.is_set && other.is_set {
            self.address == other.address
        } else {
            false
        }
    }

    pub fn prefix(&self) -> Option<&'static [u8]> {
        if self.is_set {
            hrp_prefix(self.address[0])
        } else {
            None
        }
    }

    pub fn format<const N: usize>(&self, data: &mut StaticVec<u8, N>, network_id: NetworkId) {
        match self.prefix() {
            Some(prefix) => {
                data.extend_from_slice(prefix);
                data.extend_from_slice(hrp_suffix(network_id));

                let encoding_result = Bech32::encode(data.as_slice(), self.as_ref());
                data.clear();

                match encoding_result {
                    Ok(encoder) => data.extend_from_slice(encoder.encoded()),
                    Err(..) => data.extend_from_slice(b"<bech32 error>"), // unlikely, just for completeness
                }
            }
            None => data.extend_from_slice(b"unknown address type"),
        }
    }
}
