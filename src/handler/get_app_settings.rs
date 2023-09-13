use nanos_sdk::io::Comm;
use crate::app_error::AppError;
use crate::sign::tx_state::TxState;

pub fn handle<T: Copy>(
    comm: &mut Comm,
    state: &mut TxState<T>,
) -> Result<(), AppError> {
    state.send_settings(comm)
}
