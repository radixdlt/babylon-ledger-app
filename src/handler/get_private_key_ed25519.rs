use nanos_sdk::io::Comm;

use crate::app_error::AppError;
#[cfg(debug_assertions)]
use crate::crypto::bip32::Bip32Path;
#[cfg(debug_assertions)]
use crate::crypto::ed25519::KeyPair25519;

#[cfg(debug_assertions)]
pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read(comm)
        .and_then(|path| path.validate())
        .and_then(|path| KeyPair25519::derive(&path))
        .map(|key| {
            comm.append(key.private());
        })
}

#[cfg(not(debug_assertions))]
pub fn handle(_comm: &mut Comm) -> Result<(), AppError> {
    Err(AppError::NotImplemented)
}
