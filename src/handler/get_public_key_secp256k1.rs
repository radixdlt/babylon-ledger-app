use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::secp256k1::KeyPairSecp256k1;

//TODO: network id fixed to Olympia mainnet
//TODO: display the address (dedicated command)
pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read(comm)
        .and_then(|path| path.validate_olympia_path())
        .and_then(|path| KeyPairSecp256k1::derive(&path))
        .map(|key| {
            comm.append(key.public());
            ()
        })
}
