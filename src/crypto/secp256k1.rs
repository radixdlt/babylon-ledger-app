use core::ptr::write_bytes;
use nanos_sdk::bindings::{CX_ECCINFO_PARITY_ODD, cx_err_t, CX_LAST, cx_md_t, CX_RND_RFC6979, CX_SHA256};

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::curves::{cx_ecfp_public_key_t, generate_key_pair, size_t, Curve};
use crate::crypto::key_pair::InternalKeyPair;

const PUB_KEY_TYPE_UNCOMPRESSED: u8 = 0x04;
const PUB_KEY_TYPE_COMPRESSED_Y_EVEN: u8 = 0x02;
const PUB_KEY_TYPE_COMPRESSED_Y_ODD: u8 = 0x03;
const PUB_KEY_UNCOMPRESSED_LEN: usize = 65;
const PUB_KEY_COMPRESSED_LEN: usize = 33;
const PRIV_KEY_LEN: usize = 32;
const PUB_KEY_X_COORDINATE_SIZE: usize = 32;
const PUB_KEY_UNCOMPRESSED_LAST_BYTE: usize = 64;
pub const SECP256K1_SIGNATURE_LEN: usize = 64;

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

extern "C" {
    pub fn cx_ecdsa_sign_no_throw(
        pvkey: *const u8,
        mode: u32,
        hashID: cx_md_t,
        hash: *const u8,
        hash_len: size_t,
        sig: *mut u8,
        sig_len: *mut size_t,
        info: *mut u32,
    ) -> cx_err_t;
}

impl KeyPairSecp256k1 {
    pub fn derive(path: &Bip32Path) -> Result<Self, AppError> {
        let pair = generate_key_pair(Curve::Secp256k1, path)?;
        validate_secp256k1_public_key(&pair.public)?;

        Ok(pair.into())
    }

    pub fn sign(&self, message: &[u8]) -> Result<[u8; SECP256K1_SIGNATURE_LEN], AppError> {
        let mut signature: [u8; SECP256K1_SIGNATURE_LEN] = [0; SECP256K1_SIGNATURE_LEN];

        unsafe {
            let mut info: u32 = 0;
            let mut len: size_t = signature.len() as size_t;

            cx_ecdsa_sign_no_throw(
                self.private.0.as_ptr() as *const u8,
                CX_RND_RFC6979 | CX_LAST,
                CX_SHA256,
                message.as_ptr(),
                message.len() as size_t,
                signature.as_mut_ptr(),
                &mut len as *mut size_t,
                &mut info as *mut size_t,
            );

            //TODO: check if this matches the network algorithm
            if (info & CX_ECCINFO_PARITY_ODD) != 0 {
                signature[0] |= 0x01;
            }
        }

        Ok(signature)
    }

    pub fn public(&self) -> &[u8] {
        &self.public.0
    }

    pub fn private(&self) -> &[u8] {
        &self.private.0
    }
}
