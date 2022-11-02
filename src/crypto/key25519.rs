use core::ffi::{c_uchar, c_uint};
use core::ptr::{copy, null_mut, write_bytes};
use core::str::from_utf8;

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::utilities::clone::clone_into_array;
use crate::utilities::{debug, debug_arr, debug_u32};

use crate::crypto::bindings::{
    cx_curve_t, cx_ecfp_generate_pair_no_throw, cx_ecfp_init_private_key_no_throw,
    cx_ecfp_init_public_key_no_throw, cx_ecfp_private_key_t, cx_ecfp_public_key_t, cx_err_t,
    os_perso_derive_node_with_seed_key, size_t, CX_CURVE_Ed25519, CX_SHA512, HDW_ED25519_SLIP10,
};
use crate::crypto::curves::Curve::Ed25519;
use crate::crypto::curves::{generate_key_pair, Curve};
use crate::crypto::key_pair::KeyPair;

pub struct KeyPair25519 {
    public: [u8; 32],
    private: [u8; 32],
}

impl Drop for KeyPair25519 {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

impl From<KeyPair> for KeyPair25519 {
    fn from(key_pair: KeyPair) -> Self {
        Self {
            public: transform_public_key(&key_pair.public),
            private: key_pair.private.d,
        }
    }
}

impl KeyPair25519 {
    pub fn derive(path: &Bip32Path) -> Result<Self, AppError> {
        let pair = generate_key_pair(Ed25519, path)?;
        Ok(pair.into())
    }

    pub fn public(&self) -> &[u8] {
        &self.public
    }

    pub fn private(&self) -> &[u8] {
        &self.private
    }
}

// Public key is encoded according to the following document: https://www.secg.org/sec1-v2.pdf
// See also https://crypto.stackexchange.com/questions/72134/raw-curve25519-public-key-points
// 1. Reverse the order of the bytes (we need only Y coordinate and in opposite byte order)
// 2. Flip bit in the last byte, depending on the flag which is attached to X coordinate.
// Due to resource constraints both operations are done in place, using original public
// key as a buffer. Result is returned in first 32 bytes of the public key buffer.
fn transform_public_key(pub_key: &cx_ecfp_public_key_t) -> [u8; 32] {
    let mut pk: [u8; 32] = [0u8; 32];

    let flip_bit = pub_key.W[32] & 1u8 == 1;

    for i in 0..32 {
        pk[i] = pub_key.W[64 - i];
    }

    if flip_bit {
        pk[31] |= 0x80;
    }

    pk
}
