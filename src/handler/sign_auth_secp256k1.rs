use crate::io::Comm;
use sbor::print::tx_intent_type::TxIntentType;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::handler::process_sign_outcome::process_sign_outcome;
use crate::sign::sign_mode::SignMode;
use crate::sign::tx_state::TxState;

pub fn handle<T: Copy>(
    comm: &mut Comm,
    class: CommandClass,
    state: &mut TxState<T>,
) -> Result<(), AppError> {
    state
        .process_sign_with_mode(comm, class, SignMode::AuthSecp256k1, TxIntentType::General)
        .and_then(process_sign_outcome)
}
