use core::ptr::write_bytes;

use crate::io::Comm;
use ledger_secure_sdk_sys::{
    cx_ecfp_private_key_t, cx_ecfp_public_key_t, cx_err_t, cx_md_t, CX_ECCINFO_PARITY_ODD, CX_LAST,
    CX_NONE, CX_RND_TRNG,
};

use crate::app_error::{to_result, AppError};
use crate::crypto::bip32::Bip32Path;
use crate::crypto::curves::Curve;
use crate::crypto::key_pair::InternalKeyPair;
use crate::crypto::types::size_t;
use crate::sign::sign_outcome::SignOutcome;

const PUB_KEY_TYPE_UNCOMPRESSED: u8 = 0x04;
const PUB_KEY_TYPE_COMPRESSED_Y_EVEN: u8 = 0x02;
const PUB_KEY_TYPE_COMPRESSED_Y_ODD: u8 = 0x03;
const PUB_KEY_UNCOMPRESSED_LEN: usize = 65;
const PUB_KEY_COMPRESSED_LEN: usize = 33;
const PRIV_KEY_LEN: usize = 32;
const PUB_KEY_X_COORDINATE_SIZE: usize = 32;
const PUB_KEY_UNCOMPRESSED_LAST_BYTE: usize = 64;
const DER_MAX_LEN: usize = 72;
const MAX_DER_OFFSET: usize = DER_MAX_LEN - 32;
pub const SECP256K1_SIGNATURE_LEN: usize = 65;
pub const SECP256K1_PUBLIC_KEY_LEN: usize = PUB_KEY_COMPRESSED_LEN;

pub struct KeyPairSecp256k1 {
    origin: InternalKeyPair,
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
        Self { origin: key_pair }
    }
}

fn validate_secp256k1_public_key(pub_key: &cx_ecfp_public_key_t) -> Result<(), AppError> {
    if pub_key.W_len != PUB_KEY_UNCOMPRESSED_LEN {
        return Err(AppError::BadSecp256k1PublicKeyLen);
    }

    if pub_key.W[0] != PUB_KEY_TYPE_UNCOMPRESSED {
        return Err(AppError::BadSecp256k1PublicKeyType);
    }

    Ok(())
}

extern "C" {
    pub fn cx_ecdsa_sign_no_throw(
        pvkey: *const cx_ecfp_private_key_t,
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
        let pair = Curve::Secp256k1.key_pair(path)?;
        validate_secp256k1_public_key(&pair.public)?;

        Ok(pair.into())
    }

    pub fn sign(&self, comm: &mut Comm, message: &[u8]) -> Result<SignOutcome, AppError> {
        unsafe {
            let mut info: u32 = 0;
            let mut len: size_t = DER_MAX_LEN as size_t;

            let rc = cx_ecdsa_sign_no_throw(
                &self.origin.private,
                CX_RND_TRNG | CX_LAST,
                CX_NONE,
                message.as_ptr(),
                message.len() as size_t,
                comm.work_buffer.as_mut_ptr(),
                &mut len as *mut size_t,
                &mut info as *mut size_t,
            );

            to_result(rc)?;

            // DER has format: `30 || L || 02 || Lr || r || 02 || Ls || s`

            let index_r_len = 3usize;
            let r_len = comm.work_buffer[index_r_len] as usize;
            let mut r_start = index_r_len + 1;
            let index_s_len = r_start + r_len + 1;
            let s_start = index_s_len + 1;

            if r_len == 33 {
                // we skip first byte of R.
                r_start += 1;
            }

            let parity = if (info & CX_ECCINFO_PARITY_ODD) != 0 {
                0x01u8
            } else {
                0x00
            };

            if r_start > MAX_DER_OFFSET || s_start > MAX_DER_OFFSET {
                return Err(AppError::BadSDKResponse);
            }

            comm.append(&[parity]);
            comm.append_work_buffer_from_to(r_start, r_start + 32);
            comm.append_work_buffer_from_to(s_start, s_start + 32);
        }

        self.public(comm);
        comm.append(message);

        Ok(SignOutcome::SignatureSecp256k1)
    }

    pub fn public(&self, comm: &mut Comm) {
        // check if Y is even or odd. Assuming big-endian, just check the last byte.
        let key_parity = if self.origin.public.W[PUB_KEY_UNCOMPRESSED_LAST_BYTE] % 2 == 0 {
            PUB_KEY_TYPE_COMPRESSED_Y_EVEN
        } else {
            PUB_KEY_TYPE_COMPRESSED_Y_ODD
        };

        comm.append(&[key_parity]);

        comm.append(&self.origin.public.W[1..1 + PUB_KEY_X_COORDINATE_SIZE]);
    }

    pub fn public_bytes(&self) -> [u8; SECP256K1_PUBLIC_KEY_LEN] {
        let mut pk = [0u8; SECP256K1_PUBLIC_KEY_LEN];

        let key_parity = if self.origin.public.W[PUB_KEY_UNCOMPRESSED_LAST_BYTE] % 2 == 0 {
            PUB_KEY_TYPE_COMPRESSED_Y_EVEN
        } else {
            PUB_KEY_TYPE_COMPRESSED_Y_ODD
        };

        pk[0] = key_parity;
        pk[1..].copy_from_slice(&self.origin.public.W[1..1 + PUB_KEY_X_COORDINATE_SIZE]);

        pk
    }

    pub fn private(&self) -> &[u8] {
        &self.origin.private.d
    }
}
