use crate::app_error::AppError;
use crate::command::Command;
use crate::command::Command::{
    Exit, GetAppVersion, GetDeviceId, GetDeviceModel, GetPrivKeyEd25519, GetPubKeyEd25519,
};
use crate::handler::get_public_key_ed25519;
use crate::handler::{get_device_id, get_priv_key_ed25519};
use crate::utilities::version::{MODEL_DATA, VERSION_DATA};
use nanos_sdk::io;
use nanos_sdk::io::Comm;

// APDU Command Class for Radix Ledger Apps
const RADIX_CLASS: u8 = 0xAA;

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

fn validate_request(comm: &Comm) -> Result<(), AppError> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    if comm.get_cla_ins().0 != RADIX_CLASS {
        return Err(AppError::BadCla);
    }

    Ok(())
}

pub fn dispatcher(comm: &mut Comm, ins: Command) -> Result<(), AppError> {
    validate_request(comm)?;

    match ins {
        GetAppVersion => ensure_zero_params(comm).map(|_| send_fixed(comm, &VERSION_DATA))?,
        GetDeviceModel => ensure_zero_params(comm).map(|_| send_fixed(comm, &MODEL_DATA))?,
        GetDeviceId => ensure_zero_params(comm).and_then(|_| get_device_id::handle(comm))?,
        GetPubKeyEd25519 => get_public_key_ed25519::handle(comm)?,
        GetPrivKeyEd25519 => get_priv_key_ed25519::handle(comm)?,
        Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}
