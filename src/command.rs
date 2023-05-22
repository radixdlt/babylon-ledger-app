use crate::app_error::AppError;
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

impl TryFrom<ApduHeader> for Command {
    type Error = AppError;

    fn try_from(header: ApduHeader) -> Result<Self, Self::Error> {
        match header.ins {
            0x10 => Ok(Command::GetAppVersion),
            0x11 => Ok(Command::GetDeviceModel),
            0x12 => Ok(Command::GetDeviceId),
            0x21 => Ok(Command::GetPubKeyEd25519),
            0x22 => Ok(Command::GetPrivKeyEd25519),
            0x31 => Ok(Command::GetPubKeySecp256k1),
            0x32 => Ok(Command::GetPrivKeySecp256k1),
            0x41 => Ok(Command::SignTxEd25519),
            0x42 => Ok(Command::SignTxEd25519Summary),
            0x51 => Ok(Command::SignTxSecp256k1),
            0x52 => Ok(Command::SignTxSecp256k1Summary),
            0x61 => Ok(Command::SignAuthEd25519),
            0x71 => Ok(Command::SignAuthSecp256k1),
            _ => Err(AppError::NotImplemented),
        }
    }
}
