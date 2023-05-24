use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::handler::process_sign_outcome::process_sign_outcome;
use crate::sign::sign_type::SignType;
use crate::sign::tx_state::TxState;

pub fn handle<T>(
    comm: &mut Comm,
    class: CommandClass,
    state: &mut TxState<T>,
) -> Result<(), AppError> {
    state
        .process_sign(comm, class, SignType::Ed25519Summary)
        .and_then(|outcome| process_sign_outcome(comm, outcome))
}
