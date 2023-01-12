use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::tx_sign_state::{TxSignState, SignTxType};
use crate::utilities::conversion::{to_hex_str, to_str};
use crate::utilities::{debug, debug_prepared_message};

pub fn handle(comm: &mut Comm, class: CommandClass, state: &mut TxSignState) -> Result<(), AppError> {
    state.validate(class, SignTxType::Ed25519)?;
    Ok(())
}
