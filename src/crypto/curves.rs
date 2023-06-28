use core::ffi::{c_uchar, c_uint};
use core::intrinsics::write_bytes;
use core::ptr::null_mut;

use nanos_sdk::bindings::{
    cx_curve_t, CX_CURVE_Ed25519, CX_CURVE_SECP256K1, HDW_ED25519_SLIP10, HDW_NORMAL,
};
pub use nanos_sdk::bindings::{
    cx_ecfp_private_key_t, cx_ecfp_public_key_t, cx_err_t, cx_md_t, size_t, CX_SHA512,
};

use crate::app_error::{to_result, AppError};
use crate::crypto::bip32::Bip32Path;
use crate::crypto::key_pair::InternalKeyPair;

pub const INTERNAL_SEED_SIZE: usize = 32;

pub struct Seed(pub [u8; INTERNAL_SEED_SIZE]);

impl Drop for Seed {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
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
        mode: c_uint,
        curve: cx_curve_t,
        path: *const c_uint,
        path_length: c_uint,
        private_key: *mut c_uchar,
        chain: *mut c_uchar,
        seed_key: *mut c_uchar,
        seed_key_length: c_uint,
    );
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Curve {
    Secp256k1 = CX_CURVE_SECP256K1,
    Ed25519 = CX_CURVE_Ed25519,
}

impl Curve {
    pub fn key_pair(&self, path: &Bip32Path) -> Result<InternalKeyPair, AppError> {
        let mut key_pair = InternalKeyPair {
            private: self.derive(path)?,
            public: self.init_public_key()?,
        };

        let rc = unsafe {
            cx_ecfp_generate_pair_no_throw(
                *self as cx_curve_t,
                &mut key_pair.public as *mut cx_ecfp_public_key_t,
                &key_pair.private as *const cx_ecfp_private_key_t,
                true,
            )
        };

        to_result(rc).map(|_| key_pair)
    }

    fn init_public_key(&self) -> Result<cx_ecfp_public_key_t, AppError> {
        let mut pub_key = cx_ecfp_public_key_t {
            curve: 0,
            W_len: 0,
            W: [0; 65],
        };

        let rc = unsafe {
            cx_ecfp_init_public_key_no_throw(*self as cx_curve_t, null_mut(), 0, &mut pub_key)
        };

        to_result(rc).map(|_| pub_key)
    }

    fn init_private_key(&self, seed: &Seed) -> Result<cx_ecfp_private_key_t, AppError> {
        let mut priv_key = cx_ecfp_private_key_t {
            curve: 0,
            d: [0; 32],
            d_len: 0,
        };

        let rc = unsafe {
            cx_ecfp_init_private_key_no_throw(
                *self as cx_curve_t,
                &seed.0 as *const u8,
                seed.0.len() as size_t,
                &mut priv_key,
            )
        };

        to_result(rc).map(|_| priv_key)
    }

    fn derive(&self, path: &Bip32Path) -> Result<cx_ecfp_private_key_t, AppError> {
        let mut seed = Seed([0; INTERNAL_SEED_SIZE]);

        let mode = match self {
            Curve::Secp256k1 => HDW_NORMAL,
            Curve::Ed25519 => HDW_ED25519_SLIP10,
        };

        unsafe {
            os_perso_derive_node_with_seed_key(
                mode,
                *self as cx_curve_t,
                path.path.as_ptr(),
                path.len as c_uint,
                seed.0.as_mut_ptr() as *mut c_uchar,
                null_mut(),
                null_mut(),
                0,
            );
        }

        self.init_private_key(&seed)
    }
}
