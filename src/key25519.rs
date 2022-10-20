use core::ffi::{c_uchar, c_uint};
use core::ptr::{copy, null_mut, write_bytes};
use core::str::from_utf8;
use nanos_sdk::bindings::{
    cx_curve_t, cx_ecfp_256_private_key_t, cx_ecfp_256_public_key_t,
    cx_ecfp_generate_pair2_no_throw, cx_ecfp_init_private_key_no_throw,
    cx_ecfp_init_public_key_no_throw, cx_hash_sha256, os_perso_derive_node_with_seed_key, size_t,
    CX_CURVE_Ed25519, CX_SHA512, HDW_ED25519_SLIP10,
};

use crate::app_errors::AppErrors;
use crate::sha256::Sha256;
use crate::utils::{to_hex, to_str};
use crate::{debug, Bip32Path};

pub struct Key25519 {
    public: cx_ecfp_256_public_key_t,
    private: cx_ecfp_256_private_key_t,
}

impl Drop for Key25519 {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

pub fn derive_pub_key_curve25519(path: &Bip32Path) -> Result<cx_ecfp_256_public_key_t, AppErrors> {
    debug("Before derive private key");
    let mut priv_key = derive_private_key_curve25519(&path)?;
    debug("Before init public key");
    let mut pub_key = init_public_key_curve25519()?;

    unsafe {
        let rc: AppErrors = cx_ecfp_generate_pair2_no_throw(
            CX_CURVE_Ed25519,
            &mut pub_key,
            &mut priv_key,
            true,
            CX_SHA512,
        )
        .into();

        if rc == AppErrors::Ok {
            Ok(pub_key)
        } else {
            Err(rc)
        }
    }
}

fn init_public_key_curve25519() -> Result<cx_ecfp_256_public_key_t, AppErrors> {
    unsafe {
        let mut pub_key = cx_ecfp_256_public_key_t {
            curve: 0,
            W_len: 0,
            W: [0; 65],
        };

        let rc: AppErrors =
            cx_ecfp_init_public_key_no_throw(CX_CURVE_Ed25519, null_mut(), 0, &mut pub_key).into();

        if rc == AppErrors::Ok {
            Ok(pub_key)
        } else {
            Err(rc)
        }
    }
}

extern "C" fn os_perso_derive_node_with_seed_key_(
    mode: c_uint,
    curve: cx_curve_t,
    path: *const c_uint,
    path_length: c_uint,
    private_key: *mut c_uchar,
    chain: *mut c_uchar,
    seed_key: *mut c_uchar,
    seed_key_length: c_uint,
) {
    unsafe {
        os_perso_derive_node_with_seed_key(
            mode,
            curve,
            path,
            path_length,
            private_key,
            chain,
            seed_key,
            seed_key_length,
        );
    }
}

pub fn derive_private_key_curve25519__() -> Result<[u8; 32], AppErrors> {
    //unsafe {
    let mut seed = [0u8; 32];
    // m/44'/1022'/365'
    let path: [u32; 3] = [
        44u32 | 0x80000000u32,
        1022u32 | 0x80000000u32,
        365u32 | 0x80000000u32,
    ];
    let mut ed25519_seed: [u8; 12] = [
        b'R', b'a', b'd', b'i', b'x', b'B', b'a', b'b', b'y', b'l', b'o', b'n',
    ];

    os_perso_derive_node_with_seed_key_(
        HDW_ED25519_SLIP10,
        CX_CURVE_Ed25519,
        path.as_ptr(),
        path.len() as c_uint,
        seed.as_mut_ptr() as *mut c_uchar,
        null_mut(),
        ed25519_seed.as_mut_ptr(),
        ed25519_seed.len() as c_uint,
    );

    return Ok(seed);
    //}
}

pub fn derive_private_key_curve25519(
    path: &Bip32Path,
) -> Result<cx_ecfp_256_private_key_t, AppErrors> {
    unsafe {
        let mut seed: [u8; 32] = [0; 32];
        let mut ed25519_seed: [u8; 12] = [
            b'e', b'd', b'2', b'5', b'5', b'1', b'9', b' ', b's', b'e', b'e', b'd',
        ];

        debug("Before derive node with seed key");
        let str = to_str(path.len as u32);
        debug(from_utf8(&str).unwrap());

        os_perso_derive_node_with_seed_key(
            HDW_ED25519_SLIP10,
            CX_CURVE_Ed25519,
            path.path.as_ptr(),
            path.len as c_uint,
            seed.as_mut_ptr() as *mut c_uchar,
            null_mut(),
            ed25519_seed.as_mut_ptr() as *mut c_uchar,
            ed25519_seed.len() as c_uint,
        );

        let mut priv_key = cx_ecfp_256_private_key_t {
            curve: 0,
            d: [0; 32],
            d_len: 0,
        };

        debug("Before derive init private key");

        let rc: AppErrors = cx_ecfp_init_private_key_no_throw(
            CX_CURVE_Ed25519,
            &seed as *const u8,
            seed.len() as size_t,
            &mut priv_key,
        )
        .into();

        if rc == AppErrors::Ok {
            Ok(priv_key)
        } else {
            Err(rc)
        }
    }
}
