use core::ptr::write_bytes;
use nanos_sdk::io::Comm;

use crate::app_error::{to_result, AppError};
use crate::crypto::bip32::Bip32Path;
use crate::crypto::curves::{
    cx_ecfp_private_key_t, cx_err_t, cx_md_t, size_t, Curve, CX_SHA512,
};
use crate::crypto::key_pair::InternalKeyPair;
use crate::sign::sign_outcome::SignOutcome;
use crate::utilities::debug::display_memory;

pub const ED25519_PUBLIC_KEY_LEN: usize = 32;
pub const ED25519_PRIVATE_KEY_LEN: usize = 32;
pub const ED25519_SIGNATURE_LEN: usize = 64;

//struct PublicKey25519(pub [u8; ED25519_PUBLIC_KEY_LEN]);
//struct PrivateKey25519(pub [u8; ED25519_PRIVATE_KEY_LEN]);

pub struct KeyPair25519 {
    //  public: PublicKey25519,
    //    private: PrivateKey25519,
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
            //            public: key_pair.public.into(),
            //private: PrivateKey25519(key_pair.private.d),
            origin: key_pair,
        }
    }
}

// Public key is encoded according to the following document: https://www.secg.org/sec1-v2.pdf
// See also https://crypto.stackexchange.com/questions/72134/raw-curve25519-public-key-points
//
// To build compressed version of the public key we need to do following:
// 1. Reverse the order of the bytes (we need only Y coordinate and in opposite byte order)
// 2. Flip bit in the last byte, depending on the flag which is attached to X coordinate.
// impl From<cx_ecfp_public_key_t> for PublicKey25519 {
//     fn from(pub_key: cx_ecfp_public_key_t) -> Self {
//         let mut pk = PublicKey25519([0u8; 32]);
//
//         let flip_bit = pub_key.W[32] & 1u8 == 1;
//
//         for i in 0..32 {
//             pk.0[i] = pub_key.W[64 - i];
//         }
//
//         if flip_bit {
//             pk.0[31] |= 0x80;
//         }
//
//         pk
//     }
// }

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
        display_memory(b'E'); //536872196
        Ok(Curve::Ed25519.key_pair(path)?.into())
    }

    pub fn sign(&self, comm: &mut Comm, message: &[u8]) -> Result<SignOutcome, AppError> {
        let mut signature: [u8; ED25519_SIGNATURE_LEN] = [0; ED25519_SIGNATURE_LEN];

        display_memory(b'D');

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

        comm.append(&signature);
        self.public(comm);
        comm.append(message);

        to_result(rc).map(|_| SignOutcome::SignatureEd25519)
    }

    pub fn public(&self, comm: &mut Comm) {
        let flip_bit = if self.origin.public.W[32] & 1u8 == 1 {
            0x80
        } else {
            0x00
        };

        for i in 0..32 {
            if i == 31 {
                comm.append(&[self.origin.public.W[64 - i] ^ flip_bit]);
            } else {
                comm.append(&[self.origin.public.W[64 - i]]);
            }
        }
    }

    pub fn public_bytes(&self) -> [u8; ED25519_PUBLIC_KEY_LEN] {
        let mut pk = [0u8; ED25519_PUBLIC_KEY_LEN];

        for i in 0..32 {
            pk[i] = self.origin.public.W[64 - i];
        }

        if self.origin.public.W[32] & 1u8 == 1 {
            pk[31] |= 0x80;
        }

        pk
    }

    pub fn private(&self) -> &[u8] {
        &self.origin.private.d
    }
}
