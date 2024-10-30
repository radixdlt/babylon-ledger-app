use crate::io::Comm;
use ledger_device_sdk::testing::debug_print;

use sbor::print::tx_intent_type::TxIntentType;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::handler::process_sign_outcome::process_sign_outcome;
use crate::settings::Settings;
use crate::sign::sign_mode::SignMode;
use crate::sign::tx_state::TxState;

pub fn handle<T: Copy>(
    comm: &mut Comm,
    class: CommandClass,
    state: &mut TxState<T>,
) -> Result<(), AppError> {
    let sign_mode = if Settings::get().verbose_mode {
        SignMode::TxEd25519Verbose
    } else {
        SignMode::TxEd25519Summary
    };
    state
        .process_sign_with_mode(comm, class, sign_mode, TxIntentType::Transfer)
        .and_then(process_sign_outcome)
}
