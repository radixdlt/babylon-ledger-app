use core::ffi::{c_uchar, c_uint};
use core::ptr::{copy, null_mut, write_bytes};
use core::str::from_utf8;

use nanos_sdk::bindings::{
    cx_curve_t, cx_ecfp_256_private_key_t, cx_ecfp_256_public_key_t,
    cx_ecfp_init_private_key_no_throw, cx_ecfp_init_public_key_no_throw, cx_ecfp_private_key_t,
    cx_ecfp_public_key_t, cx_err_t, cx_hash_sha256, os_perso_derive_node_with_seed_key, size_t,
    CX_CURVE_Ed25519, CX_SHA512, HDW_ED25519_SLIP10,
};

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::utilities::clone::clone_into_array;
use crate::utilities::{debug, debug_arr};

pub struct Key25519 {
    public: [u8; 32],
    private: [u8; 32],
}

impl Drop for Key25519 {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

extern "C" {
    pub fn cx_ecfp_generate_pair_no_throw(
        curve: cx_curve_t,
        pubkey: *mut cx_ecfp_public_key_t,
        privkey: *const cx_ecfp_private_key_t,
        keepprivate: bool,
    ) -> cx_err_t;
}

impl Key25519 {
    pub fn derive(path: &Bip32Path) -> Result<Self, AppError> {
        let priv_key = derive_private_key_curve25519(&path)?;
        let mut pub_key = init_public_key_curve25519()?;

        unsafe {
            let rc: AppError =
                cx_ecfp_generate_pair_no_throw(CX_CURVE_Ed25519, &mut pub_key, &priv_key, true)
                    .into();

            if rc != AppError::Ok {
                return Err(rc);
            }
        }

        // BOLOS returns public key in internal format:
        // shifted to the end and in reverse byte order
        for i in 0..32 {
            pub_key.W[i] = pub_key.W[64 - i];
        }

        Ok(Self {
            public: clone_into_array(&pub_key.W[0..32]),
            private: priv_key.d,
        })
    }

    pub fn public(&self) -> &[u8] {
        &self.public
    }

    pub fn private(&self) -> &[u8] {
        &self.private
    }
}

fn init_public_key_curve25519() -> Result<cx_ecfp_public_key_t, AppError> {
    unsafe {
        let mut pub_key = cx_ecfp_public_key_t {
            curve: 0,
            W_len: 0,
            W: [0; 65],
        };

        let rc: AppError =
            cx_ecfp_init_public_key_no_throw(CX_CURVE_Ed25519, null_mut(), 0, &mut pub_key).into();

        if rc == AppError::Ok {
            Ok(pub_key)
        } else {
            Err(rc)
        }
    }
}

fn derive_private_key_curve25519(path: &Bip32Path) -> Result<cx_ecfp_private_key_t, AppError> {
    unsafe {
        let mut seed: [u8; 32] = [0; 32];

        os_perso_derive_node_with_seed_key(
            HDW_ED25519_SLIP10,
            CX_CURVE_Ed25519,
            path.path.as_ptr(),
            path.len as c_uint,
            seed.as_mut_ptr() as *mut c_uchar,
            null_mut(),
            null_mut(),
            0,
        );

        let mut priv_key = cx_ecfp_private_key_t {
            curve: 0,
            d: [0; 32],
            d_len: 0,
        };

        let rc: AppError = cx_ecfp_init_private_key_no_throw(
            CX_CURVE_Ed25519,
            &seed as *const u8,
            seed.len() as size_t,
            &mut priv_key,
        )
        .into();

        if rc == AppError::Ok {
            Ok(priv_key)
        } else {
            Err(rc)
        }
    }
}
