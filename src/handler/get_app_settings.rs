use crate::app_error::AppError;
use crate::sign::tx_state::TxState;
use nanos_sdk::io::Comm;

pub fn handle<T: Copy>(comm: &mut Comm, state: &mut TxState<T>) -> Result<(), AppError> {
    state.send_settings(comm)
}
