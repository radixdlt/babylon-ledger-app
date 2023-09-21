use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::crypto::hash::Blake2bHasher;
use crate::handler::params_zero::ParamsZero;

// Device ID Derivation Path
const DEVICE_ID_DERIVATION_PATH: Bip32Path = Bip32Path::from(b"m/44'/1022'/365'");

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    comm.check_params_zero()
        .and_then(|_| KeyPair25519::derive(&DEVICE_ID_DERIVATION_PATH))
        .and_then(|key| Blake2bHasher::one_step(&key.public_bytes()))
        .map(|digest| comm.append(digest.as_bytes()))
}
