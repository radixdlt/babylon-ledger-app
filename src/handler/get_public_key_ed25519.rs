use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read_cap26(comm)
        .and_then(|path| KeyPair25519::derive(&path))
        .map(|key| {
            key.public(comm);
        })
}
