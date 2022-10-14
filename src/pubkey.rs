use core::ffi::{c_uchar, c_uint};
use core::ptr::copy;
use nanos_sdk::bindings::{
    cx_ecfp_256_private_key_t, cx_ecfp_256_public_key_t, cx_ecfp_generate_pair2_no_throw,
    cx_ecfp_init_private_key_no_throw, cx_ecfp_init_public_key_no_throw, cx_hash_sha256,
    os_perso_derive_node_with_seed_key, size_t, CX_SHA512, HDW_ED25519_SLIP10,
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
    pub key_type: KeyType,
    pub key: [u8; MAX_KEY_LEN],
}

pub fn derive_curve25519(path: &Bip32Path) -> Result<Key, AppErrors> {
    unsafe {
        let pub_key = generate_pair_curve25519(path)?;

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

#[derive(Default, Copy, Clone)]
pub struct Buffer32 {
    pub buffer: [u8; 32],
}

pub fn double_sha256(input: &[u8]) -> Buffer32 {
    let mut step1 = Buffer32 {
        ..Default::default()
    };
    let mut step2 = Buffer32 {
        ..Default::default()
    };

    unsafe {
        cx_hash_sha256(
            input.as_ptr(),
            input.len() as size_t,
            &mut step1.buffer as *mut u8,
            step1.buffer.len() as size_t,
        );
        cx_hash_sha256(
            &step1.buffer as *const u8,
            step1.buffer.len() as size_t,
            &mut step2.buffer as *mut u8,
            step2.buffer.len() as size_t,
        );
    }

    step2
}

fn derive_private_key_curve25519(
    path: &&Bip32Path,
) -> Result<cx_ecfp_256_private_key_t, AppErrors> {
    unsafe {
        let mut seed = Buffer32 { buffer: [0; 32] };

        os_perso_derive_node_with_seed_key(
            HDW_ED25519_SLIP10 as c_uint,
            Curve25519 as u8,
            &path.path as *const c_uint,
            path.len as c_uint,
            &mut seed.buffer as *mut c_uchar,
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
            &seed.buffer as *const u8,
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

pub const fn to_bip32_path(bytes: &[u8]) -> Bip32Path {
    // Describes current parser state
    //#[derive(Copy, Clone)]
    enum Bip32ParserState {
        FirstDigit,
        Digit,
        Hardened,
    }

    let mut path = Bip32Path {
        len: 0,
        path: [0; MAX_BIP32_PATH_LEN],
    };

    // Verify path starts with "m/"
    if (bytes[0] != b'm') || (bytes[1] != b'/') {
        panic!("path must start with \"m/\"")
    }

    // Iterate over all characters (skipping m/ header)
    let mut i = 2; // parsed character index
    let mut j = 0; // constructed path number index
    let mut acc = 0; // constructed path number
    let mut state = Bip32ParserState::FirstDigit;

    while i < bytes.len() {
        let c = bytes[i];
        match state {
            // We are expecting a digit, after a /
            // This prevent having empty numbers, like //
            Bip32ParserState::FirstDigit => match c {
                b'0'..=b'9' => {
                    acc = (c - b'0') as u32;
                    path.path[j] = acc;
                    state = Bip32ParserState::Digit
                }
                _ => panic!("expected digit after '/'"),
            },
            // We are parsing digits for the current path token. We may also
            // find ' for hardening, or /.
            Bip32ParserState::Digit => {
                match c {
                    b'0'..=b'9' => {
                        acc = acc * 10 + (c - b'0') as u32;
                        path.path[j] = acc;
                    }
                    // Hardening
                    b'\'' => {
                        path.path[j] = acc + 0x80000000;
                        j += 1;
                        state = Bip32ParserState::Hardened
                    }
                    // Separator for next number
                    b'/' => {
                        path.path[j] = acc;
                        j += 1;
                        state = Bip32ParserState::FirstDigit
                    }
                    _ => panic!("unexpected character in path"),
                }

                if j >= MAX_BIP32_PATH_LEN {
                    panic!("too long derivation path")
                }
                path.len = j as u8;
            }
            // Previous number has hardening. Next character must be a /
            // separator.
            Bip32ParserState::Hardened => match c {
                b'/' => state = Bip32ParserState::FirstDigit,
                _ => panic!("expected '/' character after hardening"),
            },
        }
        i += 1;
    }

    // Prevent last character from being /
    if let Bip32ParserState::FirstDigit = state {
        panic!("missing number in path")
    }

    path
}
