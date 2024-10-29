use crate::app_error::AppError;
use crate::command::Command;
use crate::command_class::CommandClass;
use crate::handler::*;
use crate::io::Comm;
use crate::sign::tx_state::TxState;

pub fn dispatcher<T: Copy>(
    comm: &mut Comm,
    ins: Command,
    state: &mut TxState<T>,
) -> Result<(), AppError> {
    let class = core::intrinsics::black_box(CommandClass::from_comm(comm))?;

    match ins {
        Command::GetAppVersion => get_app_version::handle(comm),
        Command::GetAppSettings => get_app_settings::handle(comm, state),
        Command::GetDeviceModel => get_device_model::handle(comm),
        Command::GetDeviceId => get_device_id::handle(comm),
        Command::GetPubKeyEd25519 => get_public_key_ed25519::handle(comm),
        Command::GetPubKeySecp256k1 => get_public_key_secp256k1::handle(comm),
        Command::SignTxEd25519 => sign_tx_ed25519::handle(comm, class, state),
        Command::SignTxSecp256k1 => sign_tx_secp256k1::handle(comm, class, state),
        Command::SignAuthEd25519 => sign_auth_ed25519::handle(comm, class, state),
        Command::SignAuthSecp256k1 => sign_auth_secp256k1::handle(comm, class, state),
        Command::VerifyAddressEd25519 => verify_address_ed25519::handle(comm),
        Command::VerifyAddressSecp256k1 => verify_address_secp256k1::handle(comm),
        Command::SignPreAuthHashEd25519 => sign_preauth_hash_ed25519::handle(comm, class, state),
        Command::SignPreAuthHashSecp256k1 => {
            sign_preauth_hash_secp256k1::handle(comm, class, state)
        }
        Command::SignPreAuthRawEd25519 => sign_preauth_raw_ed25519::handle(comm, class, state),
        Command::SignPreAuthRawSecp256k1 => sign_preauth_raw_secp256k1::handle(comm, class, state),
    }
}
