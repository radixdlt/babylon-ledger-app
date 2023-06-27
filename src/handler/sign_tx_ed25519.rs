use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::handler::process_sign_outcome::process_sign_outcome;
use crate::sign::sign_type::SignType;
use crate::sign::tx_state::TxState;
use crate::utilities::debug::display_memory;

pub fn handle<T: Copy>(
    comm: &mut Comm,
    class: CommandClass,
    state: &mut TxState<T>,
) -> Result<(), AppError> {
    display_memory(b'H'); //536873772
    state
        .process_sign(comm, class, SignType::Ed25519)
        .and_then(|outcome| process_sign_outcome(outcome))
}
