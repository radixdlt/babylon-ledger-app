use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::crypto::hash::Hasher;

// Device ID Derivation Path
const DEVICE_ID_DERIVATION_PATH: Bip32Path = Bip32Path::from(b"m/44'/1022'/365'");

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    KeyPair25519::derive(&DEVICE_ID_DERIVATION_PATH)
        .and_then(|key| Hasher::one_step_double_sha256(key.public()))
        .map(|digest| {
            comm.append(digest.as_digest());
            ()
        })
}
