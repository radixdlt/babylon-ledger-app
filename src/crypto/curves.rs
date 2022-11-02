use crate::app_error::AppError;
use crate::crypto::bindings::{
    cx_curve_t, cx_ecfp_generate_pair_no_throw, cx_ecfp_init_private_key_no_throw,
    cx_ecfp_init_public_key_no_throw, cx_ecfp_private_key_t, cx_ecfp_public_key_t,
    os_perso_derive_node_with_seed_key, size_t, CX_CURVE_Ed25519, CX_CURVE_SECP256K1,
    HDW_ED25519_SLIP10, HDW_NORMAL,
};
use crate::crypto::bip32::Bip32Path;
use crate::crypto::curves::Curve::Ed25519;
use crate::crypto::key_pair::KeyPair;
use core::ffi::{c_uchar, c_uint};
use core::ptr::null_mut;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Curve {
    Secp256k1 = CX_CURVE_SECP256K1,
    Ed25519 = CX_CURVE_Ed25519,
}

fn init_public_key(curve: Curve) -> Result<cx_ecfp_public_key_t, AppError> {
    unsafe {
        let mut pub_key = cx_ecfp_public_key_t {
            curve: 0,
            W_len: 0,
            W: [0; 65],
        };

        let rc = cx_ecfp_init_public_key_no_throw(curve as cx_curve_t, null_mut(), 0, &mut pub_key)
            .into();

        if rc == AppError::Ok {
            Ok(pub_key)
        } else {
            Err(rc)
        }
    }
}

fn init_private_key(curve: Curve, seed: [u8; 32]) -> Result<cx_ecfp_private_key_t, AppError> {
    unsafe {
        let mut priv_key = cx_ecfp_private_key_t {
            curve: 0,
            d: [0; 32],
            d_len: 0,
        };

        let rc = cx_ecfp_init_private_key_no_throw(
            curve as cx_curve_t,
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

fn derive(curve: Curve, path: &Bip32Path) -> Result<cx_ecfp_private_key_t, AppError> {
    let mut seed: [u8; 32] = [0; 32];

    unsafe {
        os_perso_derive_node_with_seed_key(
            if curve == Ed25519 {
                HDW_ED25519_SLIP10
            } else {
                HDW_NORMAL
            },
            curve as cx_curve_t,
            path.path.as_ptr(),
            path.len as c_uint,
            seed.as_mut_ptr() as *mut c_uchar,
            null_mut(),
            null_mut(),
            0,
        );
    }

    init_private_key(curve, seed)
}

pub fn generate_key_pair(curve: Curve, path: &Bip32Path) -> Result<KeyPair, AppError> {
    let mut key_pair = KeyPair {
        private: derive(curve, &path)?,
        public: init_public_key(curve)?,
    };

    unsafe {
        let rc = cx_ecfp_generate_pair_no_throw(
            curve as cx_curve_t,
            &mut key_pair.public,
            &key_pair.private,
            true,
        )
        .into();

        if rc != AppError::Ok {
            return Err(rc);
        }
    }

    Ok(key_pair)
}
