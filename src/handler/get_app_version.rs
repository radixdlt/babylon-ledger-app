#[cfg(not(target_os = "stax"))]
use crate::io::Comm;
#[cfg(target_os = "stax")]
use ledger_device_sdk::io::Comm;

use crate::app_error::AppError;
use crate::handler::params_zero::ParamsZero;
use crate::utilities::version::VERSION_DATA;

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    comm.check_params_zero().map(|_| comm.append(&VERSION_DATA))
}
