use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::handler::process_sign_outcome::process_sign_outcome;
use crate::tx_sign_state::{SignOutcome, SignTxType, TxSignState};
use crate::utilities::conversion::{to_hex_str, to_str};
use crate::utilities::{debug, debug_prepared_message};

pub fn handle(
    comm: &mut Comm,
    class: CommandClass,
    state: &mut TxSignState,
) -> Result<(), AppError> {
    state
        .process_request(comm, class, SignTxType::Ed25519)
        .and_then(|outcome| process_sign_outcome(comm, outcome))
}
