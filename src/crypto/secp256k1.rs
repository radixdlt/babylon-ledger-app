use core::ffi::{c_uchar, c_uint};
use core::ptr::{copy, null_mut, write_bytes};
use core::str::from_utf8;

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::curves::{cx_ecfp_public_key_t, generate_key_pair, size_t, Curve};
use crate::crypto::key_pair::InternalKeyPair;
use crate::utilities::{debug, debug_arr, debug_u32};

const PUB_KEY_TYPE_UNCOMPRESSED: u8 = 0x04;
const PUB_KEY_TYPE_COMPRESSED_Y_EVEN: u8 = 0x02;
const PUB_KEY_TYPE_COMPRESSED_Y_ODD: u8 = 0x03;
const PUB_KEY_UNCOMPRESSED_LEN: usize = 65;
const PUB_KEY_COMPRESSED_LEN: usize = 33;
const PRIV_KEY_LEN: usize = 32;
const PUB_KEY_X_COORDINATE_SIZE: usize = 32;
const PUB_KEY_UNCOMPRESSED_LAST_BYTE: usize = 64;

struct PublicKeySecp256k1(pub [u8; PUB_KEY_COMPRESSED_LEN]);
struct PrivateKeySecp256k1(pub [u8; PRIV_KEY_LEN]);

pub struct KeyPairSecp256k1 {
    public: PublicKeySecp256k1,
    private: PrivateKeySecp256k1,
}

impl Drop for KeyPairSecp256k1 {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

impl From<InternalKeyPair> for KeyPairSecp256k1 {
    fn from(key_pair: InternalKeyPair) -> Self {
        Self {
            public: key_pair.public.into(),
            private: PrivateKeySecp256k1(key_pair.private.d),
        }
    }
}

// This operation transforms uncompressed key into compressed one
impl From<cx_ecfp_public_key_t> for PublicKeySecp256k1 {
    fn from(pub_key: cx_ecfp_public_key_t) -> Self {
        let mut pk = PublicKeySecp256k1([0u8; PUB_KEY_COMPRESSED_LEN]);

        // check if Y is even or odd. Assuming big-endian, just check the last byte.
        pk.0[0] = if pub_key.W[PUB_KEY_UNCOMPRESSED_LAST_BYTE] % 2 == 0 {
            PUB_KEY_TYPE_COMPRESSED_Y_EVEN
        } else {
            PUB_KEY_TYPE_COMPRESSED_Y_ODD
        };

        pk.0[1..].copy_from_slice(&pub_key.W[1..1 + PUB_KEY_X_COORDINATE_SIZE]);
        pk
    }
}

fn validate_secp256k1_public_key(pub_key: &cx_ecfp_public_key_t) -> Result<(), AppError> {
    if pub_key.W_len != PUB_KEY_UNCOMPRESSED_LEN as size_t {
        return Err(AppError::BadSecp256k1PublicKeyLen);
    }

    if pub_key.W[0] != PUB_KEY_TYPE_UNCOMPRESSED {
        return Err(AppError::BadSecp256k1PublicKeyType);
    }

    Ok(())
}

impl KeyPairSecp256k1 {
    pub fn derive(path: &Bip32Path) -> Result<Self, AppError> {
        let pair = generate_key_pair(Curve::Secp256k1, path)?;
        validate_secp256k1_public_key(&pair.public)?;

        Ok(pair.into())
    }

    pub fn public(&self) -> &[u8] {
        &self.public.0
    }

    pub fn private(&self) -> &[u8] {
        &self.private.0
    }
}
