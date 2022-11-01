use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::key25519::Key25519;
use nanos_sdk::io::Comm;

#[cfg(debug_assertions)]
pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read(comm)
        .and_then(|path| path.validate())
        .and_then(|path| Key25519::derive(&path))
        .map(|key| {
            comm.append(key.private());
            ()
        })
}

#[cfg(not(debug_assertions))]
pub fn handle(_comm: &mut Comm) -> Result<(), AppError> {
    Err(AppError::NotImplemented)
}
