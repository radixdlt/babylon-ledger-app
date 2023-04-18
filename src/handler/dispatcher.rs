use nanos_sdk::io;
use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::command::Command;
use crate::command_class::CommandClass;
use crate::handler::{get_device_id, get_private_key_ed25519, sign_tx_ed25519, sign_tx_secp256k1};
use crate::handler::{get_private_key_secp256k1, get_public_key_ed25519, get_public_key_secp256k1};
use crate::tx_sign_state::TxSignState;
use crate::utilities::version::{MODEL_DATA, VERSION_DATA};

fn ensure_zero_params(comm: &Comm) -> Result<(), AppError> {
    match (comm.get_p1(), comm.get_p2()) {
        (0u8, 0u8) => Ok(()),
        (_, _) => Err(AppError::BadParam),
    }
}

fn send_fixed(comm: &mut Comm, data: &[u8]) -> () {
    comm.append(data);
    ()
}

fn validate_request(comm: &Comm, class: CommandClass) -> Result<(), AppError> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match class {
        CommandClass::Regular | CommandClass::Continuation | CommandClass::LastData => Ok(()),
        _ => Err(AppError::BadCla),
    }
}

pub fn dispatcher(comm: &mut Comm, ins: Command, state: &mut TxSignState) -> Result<(), AppError> {
    let class = CommandClass::from(comm.get_cla_ins().0);

    validate_request(comm, class)?;

    match ins {
        Command::GetAppVersion => ensure_zero_params(comm).map(|_| send_fixed(comm, &VERSION_DATA)),
        Command::GetDeviceModel => ensure_zero_params(comm).map(|_| send_fixed(comm, &MODEL_DATA)),
        Command::GetDeviceId => ensure_zero_params(comm).and_then(|_| get_device_id::handle(comm)),
        Command::GetPubKeyEd25519 => get_public_key_ed25519::handle(comm),
        Command::GetPrivKeyEd25519 => get_private_key_ed25519::handle(comm),
        Command::GetPubKeySecp256k1 => get_public_key_secp256k1::handle(comm),
        Command::GetPrivKeySecp256k1 => get_private_key_secp256k1::handle(comm),
        // TODO: temporarily handled by the same function as the non-smart version
        Command::SignTxEd25519 | Command::SignTxEd25519Smart => {
            sign_tx_ed25519::handle(comm, class, state)
        }
        // TODO: temporarily handled by the same function as the non-smart version
        Command::SignTxSecp256k1 | Command::SignTxSecp256k1Smart => {
            sign_tx_secp256k1::handle(comm, class, state)
        }
        Command::BadCommand => Err(AppError::NotImplemented),
        Command::Exit => {
            nanos_sdk::exit_app(0);
        }
    }
}
