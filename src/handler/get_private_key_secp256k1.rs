use nanos_sdk::io::Comm;

use crate::app_error::AppError;
#[cfg(debug_assertions)]
use crate::crypto::bip32::Bip32Path;
#[cfg(debug_assertions)]
use crate::crypto::secp256k1::KeyPairSecp256k1;

#[cfg(debug_assertions)]
pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read_olympia(comm)
        .and_then(|path| KeyPairSecp256k1::derive(&path))
        .map(|key| {
            comm.append(key.private());
        })
}

#[cfg(not(debug_assertions))]
pub fn handle(_comm: &mut Comm) -> Result<(), AppError> {
    Err(AppError::NotImplemented)
}
