use crate::app_error::AppError;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::secp256k1::KeyPairSecp256k1;
use nanos_sdk::io::Comm;
use crate::command_class::CommandClass;
use crate::tx_sign_state::{TxSignState, SignTxType};

pub fn handle(comm: &mut Comm, class: CommandClass, state: &mut TxSignState) -> Result<(), AppError> {
    state.validate(class, SignTxType::Secp256k1)?;
    Ok(())
}
