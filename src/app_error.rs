use nanos_sdk::bindings::{
    CX_CARRY, CX_EC_INFINITE_POINT, CX_EC_INVALID_CURVE, CX_EC_INVALID_POINT, CX_INTERNAL_ERROR,
    CX_INVALID_PARAMETER, CX_INVALID_PARAMETER_SIZE, CX_INVALID_PARAMETER_VALUE, CX_LOCKED,
    CX_MEMORY_FULL, CX_NOT_INVERTIBLE, CX_NOT_LOCKED, CX_NOT_UNLOCKED, CX_NO_RESIDUE, CX_OK,
    CX_OVERFLOW, CX_UNLOCKED,
};
use nanos_sdk::io::{Reply, StatusWords};

#[derive(Eq, PartialEq)]
pub enum AppError {
    Ok = 0x9000,
    NothingReceived = 0x6982,
    BadCla = 0x6e00,
    BadLen = 0x6e01,
    UserCancelled = 0x6e02,
    BadBip32PathLen = 0x6e03,
    BadBip32PathDataLen = 0x6e04,
    BadBip32PathLeadWord = 0x6e05,
    BadBip32PathCoinType = 0x6e06,
    BadBip32PathNetworkId = 0x6e07,
    BadBip32PathEntity = 0x6e08,
    BadBip32PathKeyType = 0x6e09,
    BadBip32PathMustBeHardened = 0x6e0a,
    BadParam = 0x6e0b,
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
            StatusWords::BadLen => AppError::BadLen,
            StatusWords::UserCancelled => AppError::UserCancelled,
            StatusWords::Unknown => AppError::Unknown,
            StatusWords::Panic => AppError::Panic,
        }
    }
}

impl From<u32> for AppError {
    fn from(code: u32) -> Self {
        match code {
            CX_OK => AppError::Ok,
            CX_CARRY => AppError::CxErrorCarry,
            CX_LOCKED => AppError::CxErrorLocked,
            CX_UNLOCKED => AppError::CxErrorUnlocked,
            CX_NOT_LOCKED => AppError::CxErrorNotLocked,
            CX_NOT_UNLOCKED => AppError::CxErrorNotUnlocked,
            CX_INTERNAL_ERROR => AppError::CxErrorInternalError,
            CX_INVALID_PARAMETER_SIZE => AppError::CxErrorInvalidParameterSize,
            CX_INVALID_PARAMETER_VALUE => AppError::CxErrorInvalidParameterValue,
            CX_INVALID_PARAMETER => AppError::CxErrorInvalidParameter,
            CX_NOT_INVERTIBLE => AppError::CxErrorNotInvertible,
            CX_OVERFLOW => AppError::CxErrorOverflow,
            CX_MEMORY_FULL => AppError::CxErrorMemoryFull,
            CX_NO_RESIDUE => AppError::CxErrorNoResidue,
            CX_EC_INFINITE_POINT => AppError::CxErrorEcInfinitePoint,
            CX_EC_INVALID_POINT => AppError::CxErrorEcInvalidPoint,
            CX_EC_INVALID_CURVE => AppError::CxErrorEcInvalidCurve,
            _ => AppError::Unknown,
        }
    }
}
