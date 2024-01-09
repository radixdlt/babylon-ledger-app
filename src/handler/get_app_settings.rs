use crate::app_error::AppError;

#[cfg(not(target_os = "stax"))]
use crate::io::Comm;
#[cfg(target_os = "stax")]
use ledger_device_sdk::io::Comm;

use crate::sign::tx_state::TxState;

pub fn handle<T: Copy>(comm: &mut Comm, state: &mut TxState<T>) -> Result<(), AppError> {
    state.send_settings(comm)
}
