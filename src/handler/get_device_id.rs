use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::key25519::Key25519;
use crate::crypto::sha256::Sha256;
use nanos_sdk::io::Comm;

// Device ID Derivation Path
const DEVICE_ID_DERIVATION_PATH: Bip32Path = Bip32Path::from(b"m/44'/1022'/365'");

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Key25519::derive(&DEVICE_ID_DERIVATION_PATH)
        .map(|key| Sha256::double(key.public()))
        .map(|hash| {
            comm.append(hash.hash());
            ()
        })
}
