use crate::app_error::AppError;
use crate::io::ApduHeader;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Command {
    GetDeviceModel,
    GetDeviceId,
    GetAppVersion,
    GetAppSettings,
    GetPubKeyEd25519,
    GetPubKeySecp256k1,
    SignTxEd25519,
    SignTxSecp256k1,
    SignAuthEd25519,
    SignAuthSecp256k1,
    VerifyAddressEd25519,
    VerifyAddressSecp256k1,
    SignPreAuthHashEd25519,
    SignPreAuthHashSecp256k1,
    Unknown,
}

impl TryFrom<ApduHeader> for Command {
    type Error = AppError;

    fn try_from(header: ApduHeader) -> Result<Self, Self::Error> {
        match header.ins {
            0x10 => Ok(Command::GetAppVersion),
            0x11 => Ok(Command::GetDeviceModel),
            0x12 => Ok(Command::GetDeviceId),
            0x22 => Ok(Command::GetAppSettings),
            0x21 => Ok(Command::GetPubKeyEd25519),
            0x31 => Ok(Command::GetPubKeySecp256k1),
            0x41 => Ok(Command::SignTxEd25519),
            0x51 => Ok(Command::SignTxSecp256k1),
            0x61 => Ok(Command::SignAuthEd25519),
            0x71 => Ok(Command::SignAuthSecp256k1),
            0x81 => Ok(Command::VerifyAddressEd25519),
            0x91 => Ok(Command::VerifyAddressSecp256k1),
            0xA1 => Ok(Command::SignPreAuthHashEd25519),
            0xA2 => Ok(Command::SignPreAuthHashSecp256k1),
            _ => Err(AppError::NotImplemented),
        }
    }
}
