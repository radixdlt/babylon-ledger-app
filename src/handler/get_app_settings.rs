use crate::app_error::AppError;
use crate::sign::tx_state::TxState;
use crate::io::Comm;

pub fn handle<T: Copy>(comm: &mut Comm, state: &mut TxState<T>) -> Result<(), AppError> {
    state.send_settings(comm)
}
