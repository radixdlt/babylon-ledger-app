use nanos_sdk::io::ApduHeader;

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
    SignTxEd25519Summary,
    SignTxSecp256k1,
    SignTxSecp256k1Summary,
    SignAuthEd25519,
    SignAuthSecp256k1,
    BadCommand,
}

impl From<ApduHeader> for Command {
    fn from(header: ApduHeader) -> Command {
        match header.ins {
            0x10 => Command::GetAppVersion,
            0x11 => Command::GetDeviceModel,
            0x12 => Command::GetDeviceId,
            0x21 => Command::GetPubKeyEd25519,
            0x22 => Command::GetPrivKeyEd25519,
            0x31 => Command::GetPubKeySecp256k1,
            0x32 => Command::GetPrivKeySecp256k1,
            0x41 => Command::SignTxEd25519,
            0x42 => Command::SignTxEd25519Summary,
            0x51 => Command::SignTxSecp256k1,
            0x52 => Command::SignTxSecp256k1Summary,
            0x61 => Command::SignAuthEd25519,
            0x71 => Command::SignAuthSecp256k1,
            _ => Command::BadCommand,
        }
    }
}
