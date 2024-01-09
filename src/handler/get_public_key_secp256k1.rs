#[cfg(not(target_os = "stax"))]
use crate::io::Comm;
#[cfg(target_os = "stax")]
use ledger_device_sdk::io::Comm;

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::secp256k1::KeyPairSecp256k1;

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read_olympia(comm)
        .and_then(|path| KeyPairSecp256k1::derive(&path))
        .map(|key| key.public(comm))
}
