use crate::bech32::address::Address;
use crate::type_info::ADDRESS_STATIC_LEN;
use core::ptr::write_bytes;

use crate::utilities::conversion::{lower_as_hex, upper_as_hex};

pub const BLAKE2B_DIGEST_SIZE: usize = 32; // 256 bits

const COPY_SIZE: usize = ADDRESS_STATIC_LEN as usize;
const COPY_FROM: usize = BLAKE2B_DIGEST_SIZE - COPY_SIZE;
const COPY_TO: usize = BLAKE2B_DIGEST_SIZE;

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct Digest(pub [u8; BLAKE2B_DIGEST_SIZE]);

impl Digest {
    pub const DIGEST_LENGTH: usize = BLAKE2B_DIGEST_SIZE;

    pub fn new() -> Self {
        Self([0; BLAKE2B_DIGEST_SIZE])
    }

    pub fn as_mut(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_hex(&self) -> [u8; Self::DIGEST_LENGTH * 2] {
        let mut output = [0u8; Self::DIGEST_LENGTH * 2];
        for (i, &byte) in self.0.iter().enumerate() {
            output[i * 2] = upper_as_hex(byte);
            output[i * 2 + 1] = lower_as_hex(byte);
        }
        output
    }

    pub fn as_address(&self, entity_type: u8) -> Address {
        let mut address = Address::new();
        address.copy_from_slice(&self.0[COPY_FROM..COPY_TO]);
        address.set_entity_type(entity_type);

        address
    }
}

impl Drop for Digest {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}
