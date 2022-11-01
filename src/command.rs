use crate::command::Command::{
    Exit, GetAppVersion, GetDeviceId, GetDeviceModel, GetPrivKeyEd25519, GetPubKeyEd25519,
};

#[repr(u8)]
pub enum Command {
    GetAppVersion,
    GetDeviceModel,
    GetDeviceId,
    GetPubKeyEd25519,
    GetPrivKeyEd25519,
    Exit,
}

impl From<u8> for Command {
    fn from(ins: u8) -> Command {
        match ins {
            0x10 => GetAppVersion,
            0x11 => GetDeviceModel,
            0x12 => GetDeviceId,
            0x21 => GetPubKeyEd25519,
            0x22 => GetPrivKeyEd25519,
            0xff => Exit,
            _ => panic!(),
        }
    }
}
