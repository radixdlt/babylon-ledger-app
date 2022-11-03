use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::key_pair::InternalKeyPair;
use core::ffi::{c_uchar, c_uint};
use core::intrinsics::write_bytes;
use core::ptr::null_mut;
use nanos_sdk::bindings::{
    cx_curve_t, cx_err_t, CX_CURVE_Ed25519, CX_CARRY, CX_CURVE_SECP256K1, CX_EC_INFINITE_POINT,
    CX_EC_INVALID_CURVE, CX_EC_INVALID_POINT, CX_INTERNAL_ERROR, CX_INVALID_PARAMETER,
    CX_INVALID_PARAMETER_SIZE, CX_INVALID_PARAMETER_VALUE, CX_LOCKED, CX_MEMORY_FULL,
    CX_NOT_INVERTIBLE, CX_NOT_LOCKED, CX_NOT_UNLOCKED, CX_NO_RESIDUE, CX_OK, CX_OVERFLOW,
    CX_SHA512, CX_UNLOCKED, HDW_ED25519_SLIP10, HDW_NORMAL,
};
pub use nanos_sdk::bindings::{cx_ecfp_private_key_t, cx_ecfp_public_key_t, size_t};

pub const INTERNAL_SEED_SIZE: usize = 32;

pub struct Seed(pub [u8; INTERNAL_SEED_SIZE]);

impl Drop for Seed {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Curve {
    Secp256k1 = CX_CURVE_SECP256K1,
    Ed25519 = CX_CURVE_Ed25519,
}

pub fn generate_key_pair(curve: Curve, path: &Bip32Path) -> Result<InternalKeyPair, AppError> {
    let mut key_pair = InternalKeyPair {
        private: derive(curve, &path)?,
        public: init_public_key(curve)?,
    };

    unsafe {
        let rc = cx_ecfp_generate_pair_no_throw(
            curve as cx_curve_t,
            &mut key_pair.public as *mut cx_ecfp_public_key_t,
            &key_pair.private as *const cx_ecfp_private_key_t,
            true,
        );
        map_error_code(rc)?;
    }

    Ok(key_pair)
}

extern "C" {
    fn cx_ecfp_generate_pair_no_throw(
        curve: cx_curve_t,
        pubkey: *mut cx_ecfp_public_key_t,
        private_key: *const cx_ecfp_private_key_t,
        keep_private: bool,
    ) -> cx_err_t;

    fn cx_ecfp_init_public_key_no_throw(
        curve: cx_curve_t,
        raw_key: *const u8,
        key_len: size_t,
        key: *mut cx_ecfp_public_key_t,
    ) -> cx_err_t;

    fn cx_ecfp_init_private_key_no_throw(
        curve: cx_curve_t,
        raw_key: *const u8,
        key_len: size_t,
        private_key: *mut cx_ecfp_private_key_t,
    ) -> cx_err_t;

    fn os_perso_derive_node_with_seed_key(
        mode: core::ffi::c_uint,
        curve: cx_curve_t,
        path: *const core::ffi::c_uint,
        path_length: core::ffi::c_uint,
        private_key: *mut core::ffi::c_uchar,
        chain: *mut core::ffi::c_uchar,
        seed_key: *mut core::ffi::c_uchar,
        seed_key_length: core::ffi::c_uint,
    );
}

fn init_public_key(curve: Curve) -> Result<cx_ecfp_public_key_t, AppError> {
    let mut pub_key = cx_ecfp_public_key_t {
        curve: 0,
        W_len: 0,
        W: [0; 65],
    };

    unsafe {
        let rc = cx_ecfp_init_public_key_no_throw(curve as cx_curve_t, null_mut(), 0, &mut pub_key);
        map_error_code(rc)?;
    };
    Ok(pub_key)
}

fn init_private_key(curve: Curve, seed: &Seed) -> Result<cx_ecfp_private_key_t, AppError> {
    let mut priv_key = cx_ecfp_private_key_t {
        curve: 0,
        d: [0; 32],
        d_len: 0,
    };

    unsafe {
        let rc = cx_ecfp_init_private_key_no_throw(
            curve as cx_curve_t,
            &seed.0 as *const u8,
            seed.0.len() as size_t,
            &mut priv_key,
        );
        map_error_code(rc)?;
    };

    Ok(priv_key)
}

fn derive(curve: Curve, path: &Bip32Path) -> Result<cx_ecfp_private_key_t, AppError> {
    let mut seed = Seed([0; INTERNAL_SEED_SIZE]);

    unsafe {
        os_perso_derive_node_with_seed_key(
            if curve == Curve::Ed25519 {
                HDW_ED25519_SLIP10
            } else {
                HDW_NORMAL
            },
            curve as cx_curve_t,
            path.path.as_ptr(),
            path.len as c_uint,
            seed.0.as_mut_ptr() as *mut c_uchar,
            null_mut(),
            null_mut(),
            0,
        );
    }

    init_private_key(curve, &seed)
}

fn map_error_code(code: cx_err_t) -> Result<(), AppError> {
    match code {
        CX_OK => Ok(()),
        CX_CARRY => Err(AppError::CxErrorCarry),
        CX_LOCKED => Err(AppError::CxErrorLocked),
        CX_UNLOCKED => Err(AppError::CxErrorUnlocked),
        CX_NOT_LOCKED => Err(AppError::CxErrorNotLocked),
        CX_NOT_UNLOCKED => Err(AppError::CxErrorNotUnlocked),
        CX_INTERNAL_ERROR => Err(AppError::CxErrorInternalError),
        CX_INVALID_PARAMETER_SIZE => Err(AppError::CxErrorInvalidParameterSize),
        CX_INVALID_PARAMETER_VALUE => Err(AppError::CxErrorInvalidParameterValue),
        CX_INVALID_PARAMETER => Err(AppError::CxErrorInvalidParameter),
        CX_NOT_INVERTIBLE => Err(AppError::CxErrorNotInvertible),
        CX_OVERFLOW => Err(AppError::CxErrorOverflow),
        CX_MEMORY_FULL => Err(AppError::CxErrorMemoryFull),
        CX_NO_RESIDUE => Err(AppError::CxErrorNoResidue),
        CX_EC_INFINITE_POINT => Err(AppError::CxErrorEcInfinitePoint),
        CX_EC_INVALID_POINT => Err(AppError::CxErrorEcInvalidPoint),
        CX_EC_INVALID_CURVE => Err(AppError::CxErrorEcInvalidCurve),
        _ => Err(AppError::Unknown),
    }
}
