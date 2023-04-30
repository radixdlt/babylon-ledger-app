use core::ptr::write_bytes;

use crate::app_error::{to_result, AppError};
use crate::crypto::bip32::Bip32Path;
use crate::crypto::curves::{
    cx_ecfp_private_key_t, cx_ecfp_public_key_t, cx_err_t, cx_md_t, generate_key_pair, size_t,
    Curve, CX_SHA512,
};
use crate::crypto::key_pair::InternalKeyPair;

pub const ED25519_PUBLIC_KEY_LEN: usize = 32;
pub const ED25519_PRIVATE_KEY_LEN: usize = 32;
pub const ED25519_SIGNATURE_LEN: usize = 64;

struct PublicKey25519(pub [u8; ED25519_PUBLIC_KEY_LEN]);
struct PrivateKey25519(pub [u8; ED25519_PRIVATE_KEY_LEN]);

pub struct KeyPair25519 {
    public: PublicKey25519,
    private: PrivateKey25519,
    origin: InternalKeyPair,
}

impl Drop for KeyPair25519 {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

impl From<InternalKeyPair> for KeyPair25519 {
    fn from(key_pair: InternalKeyPair) -> Self {
        Self {
            public: key_pair.public.into(),
            private: PrivateKey25519(key_pair.private.d),
            origin: key_pair.clone(),
        }
    }
}

// Public key is encoded according to the following document: https://www.secg.org/sec1-v2.pdf
// See also https://crypto.stackexchange.com/questions/72134/raw-curve25519-public-key-points
//
// To build compressed version of the public key we need to do following:
// 1. Reverse the order of the bytes (we need only Y coordinate and in opposite byte order)
// 2. Flip bit in the last byte, depending on the flag which is attached to X coordinate.
impl From<cx_ecfp_public_key_t> for PublicKey25519 {
    fn from(pub_key: cx_ecfp_public_key_t) -> Self {
        let mut pk = PublicKey25519([0u8; 32]);

        let flip_bit = pub_key.W[32] & 1u8 == 1;

        for i in 0..32 {
            pk.0[i] = pub_key.W[64 - i];
        }

        if flip_bit {
            pk.0[31] |= 0x80;
        }

        pk
    }
}

extern "C" {
    pub fn cx_eddsa_sign_no_throw(
        pvkey: *const cx_ecfp_private_key_t,
        hashID: cx_md_t,
        hash: *const u8,
        hash_len: size_t,
        sig: *mut u8,
        sig_len: size_t,
    ) -> cx_err_t;
}

impl KeyPair25519 {
    pub fn derive(path: &Bip32Path) -> Result<Self, AppError> {
        let pair = generate_key_pair(Curve::Ed25519, path)?;
        Ok(pair.into())
    }

    pub fn sign(&self, message: &[u8]) -> Result<[u8; ED25519_SIGNATURE_LEN], AppError> {
        let mut signature: [u8; ED25519_SIGNATURE_LEN] = [0; ED25519_SIGNATURE_LEN];

        let rc = unsafe {
            cx_eddsa_sign_no_throw(
                &self.origin.private,
                CX_SHA512,
                message.as_ptr(),
                message.len() as size_t,
                signature.as_mut_ptr(),
                signature.len() as size_t,
            )
        };

        to_result(rc).map(|_| signature)
    }

    pub fn public(&self) -> &[u8] {
        &self.public.0
    }

    pub fn private(&self) -> &[u8] {
        &self.private.0
    }

    pub fn public_key(&self) -> [u8; ED25519_PUBLIC_KEY_LEN] {
        self.public.0
    }
}
