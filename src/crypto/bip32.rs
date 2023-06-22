use core::intrinsics::write_bytes;

use nanos_sdk::io::Comm;
use sbor::bech32::network::NetworkId;
use sbor::utilities::conversion::read_u32_be;

use crate::AppError;

const BIP32_REQUIRED_LEN: u8 = 6;
const BIP32_LEAD_WORD_INDEX: usize = 0;
const BIP32_COIN_TYPE_INDEX: usize = 1;
const BIP32_NETWORK_ID_INDEX: usize = 2;
const BIP32_ENTITY_INDEX: usize = 3;
const BIP32_KEY_TYPE_INDEX: usize = 4;
const BIP32_ENTITY_INDEX_INDEX: usize = 5;

const OLYMPIA_REQUIRED_LEN: u8 = 5;
const OLYMPIA_LEAD_WORD_INDEX: usize = 0;
const OLYMPIA_COIN_TYPE_INDEX: usize = 1;
const OLYMPIA_ELEMENT1_INDEX: usize = 3;
const OLYMPIA_ELEMENT2_INDEX: usize = 4;
const OLYMPIA_ELEMENT3_INDEX: usize = 5;

const BIP32_HARDENED: u32 = 0x80000000u32;
const BIP32_LEAD_WORD: u32 = 44u32 | BIP32_HARDENED; // 0
const BIP32_COIN_TYPE: u32 = 1022u32 | BIP32_HARDENED; // 1
const BIP32_MAX_NETWORK_ID: u32 = 255u32; // 2
const BIP32_ENTITY_ACCOUNT: u32 = 525u32 | BIP32_HARDENED; // 3
const BIP32_ENTITY_IDENTITY: u32 = 618u32 | BIP32_HARDENED; // 3

const BIP32_KEY_KIND_TRANSACTION_SIGNING: u32 = 1460u32 | BIP32_HARDENED; // 5
const BIP32_KEY_KIND_AUTHENTICATION_SIGNING: u32 = 1678u32 | BIP32_HARDENED; // 5
const BIP32_KEY_KIND_MESSAGE_ENCRYPTION: u32 = 1391u32 | BIP32_HARDENED; // 5

pub const MAX_BIP32_PATH_LEN: usize = 8;
const BIP32_PATH_MIN_ENCODED_LEN: usize = 5;

#[repr(C, align(4))]
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

    fn validate_for_olympia(&self) -> Result<Bip32Path, AppError> {
        if self.len != OLYMPIA_REQUIRED_LEN {
            return Err(AppError::BadBip32PathLen);
        }

        if self.path[OLYMPIA_LEAD_WORD_INDEX] != BIP32_LEAD_WORD {
            return Err(AppError::BadBip32PathLeadWord);
        }

        if self.path[OLYMPIA_COIN_TYPE_INDEX] != BIP32_COIN_TYPE {
            return Err(AppError::BadBip32PathCoinType);
        }

        Ok(Self {
            len: self.len,
            path: self.path,
        })
    }

    fn validate_for_cap26(&self) -> Result<Bip32Path, AppError> {
        if self.len != BIP32_REQUIRED_LEN {
            return Err(AppError::BadBip32PathLen);
        }

        if self.path[BIP32_LEAD_WORD_INDEX] != BIP32_LEAD_WORD {
            return Err(AppError::BadBip32PathLeadWord);
        }

        if self.path[BIP32_COIN_TYPE_INDEX] != BIP32_COIN_TYPE {
            return Err(AppError::BadBip32PathCoinType);
        }

        let network_id = self.path[BIP32_NETWORK_ID_INDEX];

        if (network_id & BIP32_HARDENED) == 0 {
            return Err(AppError::BadBip32PathMustBeHardened);
        }

        NetworkId::try_from(network_id & !BIP32_HARDENED)
            .map_err(|_| AppError::BadBip32PathNetworkId)?;

        if self.path[BIP32_ENTITY_INDEX] != BIP32_ENTITY_ACCOUNT
            && self.path[BIP32_ENTITY_INDEX] != BIP32_ENTITY_IDENTITY
        {
            return Err(AppError::BadBip32PathEntity);
        }

        if (self.path[BIP32_ENTITY_INDEX_INDEX] & BIP32_HARDENED) == 0 {
            return Err(AppError::BadBip32PathMustBeHardened);
        }

        if self.path[BIP32_KEY_TYPE_INDEX] != BIP32_KEY_KIND_TRANSACTION_SIGNING
            && self.path[BIP32_KEY_TYPE_INDEX] != BIP32_KEY_KIND_AUTHENTICATION_SIGNING
            && self.path[BIP32_KEY_TYPE_INDEX] != BIP32_KEY_KIND_MESSAGE_ENCRYPTION
        {
            return Err(AppError::BadBip32PathKeyType);
        }

        Ok(Self {
            len: self.len,
            path: self.path,
        })
    }

    // Following data layout is assumed (bytes):
    // [0] - len (in number of BIP32 path elements)
    // [1..5] - first path element (big endian)
    // [5..9] - second path element (big endian)
    // ...
    fn read(comm: &mut Comm) -> Result<Self, AppError> {
        let data = comm.get_data()?;

        if data.len() < BIP32_PATH_MIN_ENCODED_LEN {
            return Err(AppError::BadBip32PathDataLen);
        }

        let path_len = data[0];

        if data.len() < ((path_len as usize) * 4 + 1) {
            return Err(AppError::BadBip32PathDataLen);
        }

        let mut path = Bip32Path::new(path_len);
        let mut idx = 1usize;

        for i in 0..path_len as usize {
            path.path[i] = read_u32_be(&data[idx..]);
            idx += 4;
        }

        Ok(path)
    }

    // Read path and validate according to the CAP-26 document.
    pub fn read_cap26(comm: &mut Comm) -> Result<Self, AppError> {
        Bip32Path::read(comm).and_then(|path| path.validate_for_cap26())
    }

    // Read path and validate according to the Olympia mainnet rules.
    pub fn read_olympia(comm: &mut Comm) -> Result<Self, AppError> {
        Bip32Path::read(comm).and_then(|path| path.validate_for_olympia())
    }

    pub const fn new(len: u8) -> Self {
        Self {
            path: [0u32; MAX_BIP32_PATH_LEN],
            len,
        }
    }

    pub const fn for_path(some_path: &[u32]) -> Self {
        let mut path = Bip32Path {
            len: some_path.len() as u8,
            path: [0; MAX_BIP32_PATH_LEN],
        };

        let mut i = 0;

        while i < some_path.len() {
            path.path[i] = some_path[i];
            i += 1;
        }
        path
    }

    pub fn network_id(&self) -> Result<NetworkId, AppError> {
        NetworkId::try_from(self.path[BIP32_NETWORK_ID_INDEX] & !BIP32_HARDENED)
            .map_err(|_| AppError::BadBip32PathNetworkId)
    }
}
