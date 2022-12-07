use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::secp256k1::KeyPairSecp256k1;
use nanos_sdk::io::Comm;

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Ok(())
}
