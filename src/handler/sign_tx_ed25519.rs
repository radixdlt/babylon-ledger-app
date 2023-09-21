use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::curves::Curve;
use crate::handler::process_sign_outcome::process_sign_outcome;
use crate::sign::tx_state::TxState;

pub fn handle<T: Copy>(
    comm: &mut Comm,
    class: CommandClass,
    state: &mut TxState<T>,
) -> Result<(), AppError> {
    state
        .sign_tx(comm, class, Curve::Ed25519)
        .and_then(|outcome| process_sign_outcome(outcome))
}
