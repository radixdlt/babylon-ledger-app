use core::ffi::{c_uchar, c_uint};
use core::ptr::copy;
use nanos_sdk::bindings::{
    cx_ecfp_256_private_key_t, cx_ecfp_256_public_key_t, cx_ecfp_generate_pair2_no_throw,
    cx_ecfp_init_private_key_no_throw, cx_ecfp_init_public_key_no_throw,
    os_perso_derive_node_with_seed_key, CX_SHA512, HDW_ED25519_SLIP10,
};
use nanos_sdk::ecc::CurvesId::Curve25519;
//use nanos_sdk::ecc::CurvesId::Secp256k1;

use crate::app_errors::AppErrors;
use crate::pubkey::KeyType::{
    Curve25519Private, Curve25519Public, Secp256k1Private, Secp256k1PublicCompressed,
    Secp256k1PublicUncompressed,
};

const MAX_BIP32_PATH_LEN: usize = 10;

// Secp256k1:
//   Public keys:
//     Compressed: 33
//     Uncompressed: 64
//   Private keys: 32
// Curve25519:
//   Public keys: 32
//   Private keys: 32
const MAX_KEY_LEN: usize = 64;

pub struct Bip32Path {
    len: u8,
    path: [u32; MAX_BIP32_PATH_LEN],
}

// pub fn to_bip32path<const N: usize>(input: [u32; N]) -> Bip32Path {
//     let res = Bip32Path { len: N.into(), path: input.clone()};
// }

pub enum KeyType {
    Secp256k1PublicUncompressed,
    Secp256k1PublicCompressed,
    Secp256k1Private,
    Curve25519Public,
    Curve25519Private,
}

pub const fn key_len(key_type: KeyType) -> usize {
    match key_type {
        Secp256k1PublicUncompressed => 64,
        Secp256k1PublicCompressed => 33,
        Secp256k1Private => 32,
        Curve25519Public => 32,
        Curve25519Private => 32,
    }
}

pub struct Key {
    key_type: KeyType,
    key: [u8; MAX_KEY_LEN],
}

struct Seed {
    seed: [u8; 32],
}

impl Default for Seed {
    fn default() -> Self {
        let mut s = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

pub fn derive_curve25519(path: &Bip32Path) -> Result<Key, AppErrors> {
    unsafe {
        let mut pub_key = generate_pair_curve25519(path)?;

        let mut derived = Key {
            key_type: Curve25519Public,
            key: [0; MAX_KEY_LEN],
        };

        copy(
            &pub_key.W as *const u8,
            &mut derived.key as *mut u8,
            key_len(Curve25519Public),
        );

        Ok(derived)
    }
}

fn generate_pair_curve25519(path: &Bip32Path) -> Result<cx_ecfp_256_public_key_t, AppErrors> {
    unsafe {
        let mut priv_key = derive_private_key_curve25519(&path)?;
        let mut pub_key = init_public_key_curve25519()?;

        let rc: AppErrors = cx_ecfp_generate_pair2_no_throw(
            Curve25519 as u8,
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

        let rc: AppErrors = cx_ecfp_init_public_key_no_throw(
            Curve25519 as u8,
            core::ptr::null_mut(),
            0,
            &mut pub_key,
        )
        .into();

        if rc == AppErrors::Ok {
            Ok(pub_key)
        } else {
            Err(rc)
        }
    }
}

fn derive_private_key_curve25519(
    path: &&Bip32Path,
) -> Result<cx_ecfp_256_private_key_t, AppErrors> {
    unsafe {
        let mut seed = Seed { seed: [0; 32] };

        os_perso_derive_node_with_seed_key(
            HDW_ED25519_SLIP10 as c_uint,
            Curve25519 as u8,
            &path.path as *const c_uint,
            path.len as c_uint,
            &mut seed.seed as *mut c_uchar,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
        );

        let mut priv_key = cx_ecfp_256_private_key_t {
            curve: 0,
            d: [0; 32],
            d_len: 0,
        };

        let rc: AppErrors = cx_ecfp_init_private_key_no_throw(
            Curve25519 as u8,
            &seed.seed as *const u8,
            32u32,
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
