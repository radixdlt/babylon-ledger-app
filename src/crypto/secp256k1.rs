use core::ffi::{c_uchar, c_uint};
use core::ptr::{copy, null_mut, write_bytes};
use core::str::from_utf8;

use crate::crypto::bindings::{
    cx_curve_t, cx_ecfp_generate_pair_no_throw, cx_ecfp_init_private_key_no_throw,
    cx_ecfp_init_public_key_no_throw, cx_ecfp_private_key_t, cx_ecfp_public_key_t, cx_err_t,
    os_perso_derive_node_with_seed_key, size_t, CX_CURVE_Ed25519, CX_SHA512, HDW_ED25519_SLIP10,
};

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::curves::generate_key_pair;
use crate::crypto::curves::Curve::Secp256k1;
use crate::crypto::key_pair::KeyPair;
use crate::utilities::clone::clone_into_array;
use crate::utilities::{debug, debug_arr, debug_u32};

const PK_TYPE_UNCOMPRESSED: u8 = 0x04;
const PK_TYPE_COMPRESSED_Y_EVEN: u8 = 0x02;
const PK_TYPE_COMPRESSED_Y_ODD: u8 = 0x03;
const PK_UNCOMPRESSED_LEN: size_t = 65;
const PK_COMPRESSED_LEN: size_t = 33;
const PK_X_COORDINATE_SIZE: usize = 32;
const PK_LAST_BYTE: usize = 64;

pub struct KeySecp256k1 {
    public: [u8; 33],
    private: [u8; 32],
}

impl Drop for KeySecp256k1 {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

impl From<KeyPair> for KeySecp256k1 {
    fn from(key_pair: KeyPair) -> Self {
        Self {
            public: transform_public_key(&key_pair.public),
            private: key_pair.private.d,
        }
    }
}

fn validate_secp256k1_public_key(pub_key: &cx_ecfp_public_key_t) -> Result<(), AppError> {
    if pub_key.W_len != PK_UNCOMPRESSED_LEN {
        return Err(AppError::BadSecp256k1PublicKeyLen);
    }

    if pub_key.W[0] != PK_TYPE_UNCOMPRESSED {
        return Err(AppError::BadSecp256k1PublicKeyType);
    }

    Ok(())
}

impl KeySecp256k1 {
    pub fn derive(path: &Bip32Path) -> Result<Self, AppError> {
        let pair = generate_key_pair(Secp256k1, path)?;
        validate_secp256k1_public_key(&pair.public)?;

        Ok(pair.into())
    }

    pub fn public(&self) -> &[u8] {
        &self.public
    }

    pub fn private(&self) -> &[u8] {
        &self.private
    }
}

fn transform_public_key(pub_key: &cx_ecfp_public_key_t) -> [u8; 33] {
    let mut pk: [u8; 33] = [0u8; 33];

    // check if Y is even or odd. Assuming big-endian, just check the last byte.
    pk[0] = if pub_key.W[PK_LAST_BYTE] % 2 == 0 {
        PK_TYPE_COMPRESSED_Y_EVEN
    } else {
        PK_TYPE_COMPRESSED_Y_ODD
    };

    unsafe {
        copy(
            &pub_key.W[1] as *const u8,
            &mut pk[1] as *mut u8,
            PK_X_COORDINATE_SIZE,
        );
    }

    pk
}
