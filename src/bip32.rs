use crate::AppErrors;
use core::intrinsics::{copy, write_bytes};
use core::ptr::copy_nonoverlapping;
use nanos_sdk::io::Comm;

const BIP32_REQUIRED_LEN: u8 = 6;
const BIP32_LEAD_WORD_INDEX: usize = 0;
const BIP32_COIN_TYPE_INDEX: usize = 1;
const BIP32_NETWORK_ID_INDEX: usize = 2;
const BIP32_ENTITY_INDEX: usize = 3;
const BIP32_ENTITY_INDEX_INDEX: usize = 4;
const BIP32_KEY_TYPE_INDEX: usize = 5;

const BIP32_HARDENED: u32 = 0x80000000u32;
const BIP32_LEAD_WORD: u32 = 44u32 | BIP32_HARDENED; // 0
const BIP32_COIN_TYPE: u32 = 1022u32 | BIP32_HARDENED; // 1
const BIP32_MAX_NETWORK_ID: u32 = 255u32; // 2
const BIP32_ENTITY_ACCOUNT: u32 = 525u32 | BIP32_HARDENED; // 3
const BIP32_ENTITY_IDENTITY: u32 = 618u32 | BIP32_HARDENED; // 3
const BIP32_KEY_TYPE_SIGN_TRANSACTION: u32 = 1238u32 | BIP32_HARDENED; // 5
const BIP32_KEY_TYPE_SIGN_AUTH: u32 = 706u32 | BIP32_HARDENED; // 5

pub const MAX_BIP32_PATH_LEN: usize = 10;
#[repr(C)]
#[derive(Default, Clone)]
pub struct Bip32Path {
    pub path: [u32; MAX_BIP32_PATH_LEN],
    pub len: u8,
}

impl Drop for Bip32Path {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

impl Bip32Path {
    pub const fn from(bytes: &[u8]) -> Bip32Path {
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

    pub fn validate(&self) -> Result<(), AppErrors> {
        if self.len != BIP32_REQUIRED_LEN {
            return Err(AppErrors::BadBip32PathLen);
        }

        if self.path[BIP32_LEAD_WORD_INDEX] != BIP32_LEAD_WORD {
            return Err(AppErrors::BadBip32PathLeadWord);
        }

        if self.path[BIP32_COIN_TYPE_INDEX] != BIP32_COIN_TYPE {
            return Err(AppErrors::BadBip32PathCoinType);
        }

        let mut network_id = self.path[BIP32_NETWORK_ID_INDEX];

        if (network_id & BIP32_HARDENED) == 0 {
            return Err(AppErrors::BadBip32PathMustBeHardened);
        }

        network_id &= !BIP32_HARDENED;

        if network_id > BIP32_MAX_NETWORK_ID {
            return Err(AppErrors::BadBip32PathNetworkId);
        }

        if self.path[BIP32_ENTITY_INDEX] != BIP32_ENTITY_ACCOUNT
            && self.path[BIP32_ENTITY_INDEX] != BIP32_ENTITY_IDENTITY
        {
            return Err(AppErrors::BadBip32PathEntity);
        }

        if (self.path[BIP32_ENTITY_INDEX_INDEX] & BIP32_HARDENED) == 0 {
            return Err(AppErrors::BadBip32PathMustBeHardened);
        }

        if self.path[BIP32_KEY_TYPE_INDEX] != BIP32_KEY_TYPE_SIGN_AUTH
            && self.path[BIP32_KEY_TYPE_INDEX] != BIP32_KEY_TYPE_SIGN_TRANSACTION
        {
            return Err(AppErrors::BadBip32PathKeyType);
        }

        Ok(())
    }

    pub fn read(comm: &mut Comm) -> Result<Self, AppErrors> {
        if comm.rx <= 4 {
            return Err(AppErrors::BadBip32PathLen);
        }

        let path_len = comm.apdu_buffer[4];
        let count = (path_len * 4) as usize;

        if comm.rx != count + 5 {
            return Err(AppErrors::BadBip32PathDataLen);
        }

        unsafe {
            let src = comm.apdu_buffer[5..(count + 5)].as_ptr();
            let mut path = Bip32Path::new(path_len);

            copy_nonoverlapping(src, path.path.as_mut_ptr() as *mut u8, count);
            Ok(path)
        }
    }

    pub fn new(len: u8) -> Self {
        Self {
            path: [0u32; MAX_BIP32_PATH_LEN],
            len,
        }
    }
}
