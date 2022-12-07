#[repr(u8)]
pub enum Command {
    GetAppVersion,
    GetDeviceModel,
    GetDeviceId,
    GetPubKeyEd25519,
    GetPrivKeyEd25519,
    GetPubKeySecp256k1,
    GetPrivKeySecp256k1,
    SignTxEd25519,
    SignTxSecp256k1,
    BadCommand,
    Exit,
}

impl From<u8> for Command {
    fn from(ins: u8) -> Command {
        match ins {
            0x10 => Command::GetAppVersion,
            0x11 => Command::GetDeviceModel,
            0x12 => Command::GetDeviceId,
            0x21 => Command::GetPubKeyEd25519,
            0x22 => Command::GetPrivKeyEd25519,
            0x31 => Command::GetPubKeySecp256k1,
            0x32 => Command::GetPrivKeySecp256k1,
            0xff => Command::Exit,
            _ => Command::BadCommand,
        }
    }
}
