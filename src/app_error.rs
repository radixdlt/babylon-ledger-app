use nanos_sdk::bindings::{
    CX_CARRY, CX_EC_INFINITE_POINT, CX_EC_INVALID_CURVE, CX_EC_INVALID_POINT, CX_INTERNAL_ERROR,
    CX_INVALID_PARAMETER, CX_INVALID_PARAMETER_SIZE, CX_INVALID_PARAMETER_VALUE, CX_LOCKED,
    CX_MEMORY_FULL, CX_NOT_INVERTIBLE, CX_NOT_LOCKED, CX_NOT_UNLOCKED, CX_NO_RESIDUE, CX_OK,
    CX_OVERFLOW, CX_UNLOCKED,
};
use nanos_sdk::io::{Reply, StatusWords};
use sbor::decoder_error::DecoderError;

#[derive(Copy, Clone, Debug)]
pub enum AppError {
    Ok = 0x9000,
    NothingReceived = 0x6982,

    BadCla = 0x6e00,
    BadIns = 0x6e01,
    BadP1P2 = 0x6e02,
    BadLen = 0x6e03,
    UserCancelled = 0x6e04,

    BadBip32PathLen = 0x6e10,
    BadBip32PathDataLen = 0x6e11,
    BadBip32PathLeadWord = 0x6e12,
    BadBip32PathCoinType = 0x6e13,
    BadBip32PathNetworkId = 0x6e14,
    BadBip32PathEntity = 0x6e15,
    BadBip32PathKeyType = 0x6e16,
    BadBip32PathMustBeHardened = 0x6e17,

    BadSecp256k1PublicKeyLen = 0x6e21,
    BadSecp256k1PublicKeyType = 0x6e22,

    BadTxSignSequence = 0x6e31,
    BadTxSignLen = 0x6e32,
    BadTxSignInitialState = 0x6e33,
    BadTxSignStart = 0x6e34,
    BadTxSignType = 0x6e35,
    BadTxSignDigestState = 0x6e36,
    BadTxSignRequestedState = 0x6e37,

    BadTxSignDecoderErrorInvalidInput = 0x6e41,
    BadTxSignDecoderErrorInvalidLen = 0x6e42,
    BadTxSignDecoderErrorInvalidState = 0x6e43,
    BadTxSignDecoderErrorStackOverflow = 0x6e44,
    BadTxSignDecoderErrorStackUnderflow = 0x6e45,
    BadTxSignDecoderErrorUnknownType = 0x6e46,
    BadTxSignDecoderErrorUnknownParameterType = 0x6e47,
    BadTxSignDecoderErrorUnknownEnum = 0x6e48,

    BadTxSignUserRejected = 0x6e50,

    NotImplemented = 0x6eff,
    Unknown = 0x6d00,
    CxErrorCarry = 0x6f01,
    CxErrorLocked = 0x6f02,
    CxErrorUnlocked = 0x6f03,
    CxErrorNotLocked = 0x6f04,
    CxErrorNotUnlocked = 0x6f05,
    CxErrorInternalError = 0x6f06,
    CxErrorInvalidParameterSize = 0x6f07,
    CxErrorInvalidParameterValue = 0x6f08,
    CxErrorInvalidParameter = 0x6f09,
    CxErrorNotInvertible = 0x6f0a,
    CxErrorOverflow = 0x6f0b,
    CxErrorMemoryFull = 0x6f0c,
    CxErrorNoResidue = 0x6f0d,
    CxErrorEcInfinitePoint = 0x6f0e,
    CxErrorEcInvalidPoint = 0x6f0f,
    CxErrorEcInvalidCurve = 0x6f10,
    Panic = 0xe000,
}

impl From<AppError> for Reply {
    fn from(sw: AppError) -> Reply {
        Reply(sw as u16)
    }
}

impl From<StatusWords> for AppError {
    fn from(sw: StatusWords) -> AppError {
        match sw {
            StatusWords::Ok => AppError::Ok,
            StatusWords::NothingReceived => AppError::NothingReceived,
            StatusWords::BadCla => AppError::BadCla,
            StatusWords::BadIns => AppError::BadIns,
            StatusWords::BadP1P2 => AppError::BadP1P2,
            StatusWords::BadLen => AppError::BadLen,
            StatusWords::UserCancelled => AppError::UserCancelled,
            StatusWords::Unknown => AppError::Unknown,
            StatusWords::Panic => AppError::Panic,
        }
    }
}

impl From<DecoderError> for AppError {
    fn from(value: DecoderError) -> AppError {
        match value {
            DecoderError::UnknownType(_, _) => AppError::BadTxSignDecoderErrorUnknownType,
            DecoderError::UnknownSubType(_, _) => {
                AppError::BadTxSignDecoderErrorUnknownParameterType
            }
            DecoderError::UnknownDiscriminator(_, _) => AppError::BadTxSignDecoderErrorUnknownEnum,
            DecoderError::InvalidLen(_, _) => AppError::BadTxSignDecoderErrorInvalidLen,
            DecoderError::InvalidState(_) => AppError::BadTxSignDecoderErrorInvalidState,
            DecoderError::StackOverflow(_) => AppError::BadTxSignDecoderErrorStackOverflow,
            DecoderError::StackUnderflow(_) => AppError::BadTxSignDecoderErrorStackUnderflow,
        }
    }
}

impl From<u32> for AppError {
    fn from(value: u32) -> Self {
        match value {
            CX_CARRY => AppError::CxErrorCarry,
            CX_EC_INFINITE_POINT => AppError::CxErrorEcInfinitePoint,
            CX_EC_INVALID_CURVE => AppError::CxErrorEcInvalidCurve,
            CX_EC_INVALID_POINT => AppError::CxErrorEcInvalidPoint,
            CX_INTERNAL_ERROR => AppError::CxErrorInternalError,
            CX_INVALID_PARAMETER => AppError::CxErrorInvalidParameter,
            CX_INVALID_PARAMETER_SIZE => AppError::CxErrorInvalidParameterSize,
            CX_INVALID_PARAMETER_VALUE => AppError::CxErrorInvalidParameterValue,
            CX_LOCKED => AppError::CxErrorLocked,
            CX_MEMORY_FULL => AppError::CxErrorMemoryFull,
            CX_NOT_INVERTIBLE => AppError::CxErrorNotInvertible,
            CX_NOT_LOCKED => AppError::CxErrorNotLocked,
            CX_NOT_UNLOCKED => AppError::CxErrorNotUnlocked,
            CX_NO_RESIDUE => AppError::CxErrorNoResidue,
            CX_OVERFLOW => AppError::CxErrorOverflow,
            CX_UNLOCKED => AppError::CxErrorUnlocked,
            _ => AppError::Unknown,
        }
    }
}

pub fn to_result(rc: u32) -> Result<(), AppError> {
    if rc == CX_OK {
        Ok(())
    } else {
        Err(rc.into())
    }
}
