use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::handler::process_sign_outcome::process_sign_outcome;
use crate::tx_sign_state::{SignTxType, TxSignState};
use nanos_sdk::io::Comm;

pub fn handle(
    comm: &mut Comm,
    class: CommandClass,
    state: &mut TxSignState,
) -> Result<(), AppError> {
    state
        .process_request(comm, class, SignTxType::Secp256k1Summary)
        .and_then(|outcome| process_sign_outcome(comm, outcome))
}
