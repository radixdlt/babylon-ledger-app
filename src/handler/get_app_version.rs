use crate::io::Comm;

use crate::app_error::AppError;
use crate::handler::params_zero::ParamsZero;
use crate::utilities::version::VERSION_DATA;

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    comm.check_params_zero().map(|_| comm.append(&VERSION_DATA))
}
