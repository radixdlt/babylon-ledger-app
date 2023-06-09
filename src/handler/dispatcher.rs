use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::command::Command;
use crate::command_class::CommandClass;
use crate::handler::*;
use crate::sign::tx_state::TxState;

pub fn dispatcher<T: Copy>(
    comm: &mut Comm,
    ins: Command,
    state: &mut TxState<T>,
) -> Result<(), AppError> {
    let class = CommandClass::from_comm(comm)?;

    match ins {
        Command::GetAppVersion => get_app_version::handle(comm),
        Command::GetDeviceModel => get_device_model::handle(comm),
        Command::GetDeviceId => get_device_id::handle(comm),
        Command::GetPubKeyEd25519 => get_public_key_ed25519::handle(comm),
        Command::GetPrivKeyEd25519 => get_private_key_ed25519::handle(comm),
        Command::GetPubKeySecp256k1 => get_public_key_secp256k1::handle(comm),
        Command::GetPrivKeySecp256k1 => get_private_key_secp256k1::handle(comm),
        Command::SignTxEd25519 => sign_tx_ed25519::handle(comm, class, state),
        Command::SignTxEd25519Summary => sign_tx_ed25519_summary::handle(comm, class, state),
        Command::SignTxSecp256k1 => sign_tx_secp256k1::handle(comm, class, state),
        Command::SignTxSecp256k1Summary => sign_tx_secp256k1_summary::handle(comm, class, state),
        Command::SignAuthEd25519 => sign_auth_ed25519::handle(comm, class, state),
        Command::SignAuthSecp256k1 => sign_auth_secp256k1::handle(comm, class, state),
        Command::BadCommand => Err(AppError::NotImplemented),
    }
}
