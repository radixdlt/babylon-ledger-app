use ledger_device_sdk::testing::debug_print;

use crate::io::Comm;

use crate::app_error::AppError;
use crate::handler::params_zero::ParamsZero;
use crate::utilities::dbg_print;
use crate::utilities::version::VERSION_DATA;

pub fn handle(comm: &mut Comm) -> Result<(), AppError> {
    dbg_print!(
        "\n\n🔮🔮 This works like {},\nversion: {:?}\n",
        "println!",
        VERSION_DATA
    );
    comm.check_params_zero().map(|_| comm.append(&VERSION_DATA))
}
