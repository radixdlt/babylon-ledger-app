use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::crypto::address_verifier::verify_address;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::curves::Curve;

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read_olympia(comm)
        .and_then(|path| Curve::Secp256k1.to_address(&path))
        .map(|(address, network_id)| verify_address(address, network_id, comm))
}
