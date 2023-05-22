use nanos_sdk::io::{Comm, StatusWords};

use crate::app_error::AppError;

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum CommandClass {
    Regular,
    Continuation,
    LastData,
}

impl CommandClass {
    pub fn from_comm(comm: &Comm) -> Result<CommandClass, AppError> {
        if comm.rx == 0 {
            return Err(StatusWords::NothingReceived.into());
        }

        match comm.get_apdu_metadata().cla {
            0xAA => Ok(CommandClass::Regular),
            0xAB => Ok(CommandClass::Continuation),
            0xAC => Ok(CommandClass::LastData),
            _ => Err(AppError::BadCla),
        }
    }
}
