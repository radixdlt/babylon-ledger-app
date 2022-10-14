use nanos_sdk::bindings::{
    CX_CARRY, CX_EC_INFINITE_POINT, CX_EC_INVALID_CURVE, CX_EC_INVALID_POINT, CX_INTERNAL_ERROR,
    CX_INVALID_PARAMETER, CX_INVALID_PARAMETER_SIZE, CX_INVALID_PARAMETER_VALUE, CX_LOCKED,
    CX_MEMORY_FULL, CX_NOT_INVERTIBLE, CX_NOT_LOCKED, CX_NOT_UNLOCKED, CX_NO_RESIDUE, CX_OK,
    CX_OVERFLOW, CX_UNLOCKED,
};
use nanos_sdk::io::{Reply, StatusWords};

#[derive(Eq, PartialEq)]
pub enum AppErrors {
    Ok = 0x9000,
    NothingReceived = 0x6982,
    BadCla = 0x6e00,
    BadLen = 0x6e01,
    UserCancelled = 0x6e02,
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

impl From<AppErrors> for Reply {
    fn from(sw: AppErrors) -> Reply {
        Reply(sw as u16)
    }
}

impl From<StatusWords> for AppErrors {
    fn from(sw: StatusWords) -> AppErrors {
        match sw {
            StatusWords::Ok => AppErrors::Ok,
            StatusWords::NothingReceived => AppErrors::NothingReceived,
            StatusWords::BadCla => AppErrors::BadCla,
            StatusWords::BadLen => AppErrors::BadLen,
            StatusWords::UserCancelled => AppErrors::UserCancelled,
            StatusWords::Unknown => AppErrors::Unknown,
            StatusWords::Panic => AppErrors::Panic,
        }
    }
}

impl From<u32> for AppErrors {
    fn from(code: u32) -> Self {
        match code {
            CX_OK => AppErrors::Ok,
            CX_CARRY => AppErrors::CxErrorCarry,
            CX_LOCKED => AppErrors::CxErrorLocked,
            CX_UNLOCKED => AppErrors::CxErrorUnlocked,
            CX_NOT_LOCKED => AppErrors::CxErrorNotLocked,
            CX_NOT_UNLOCKED => AppErrors::CxErrorNotUnlocked,
            CX_INTERNAL_ERROR => AppErrors::CxErrorInternalError,
            CX_INVALID_PARAMETER_SIZE => AppErrors::CxErrorInvalidParameterSize,
            CX_INVALID_PARAMETER_VALUE => AppErrors::CxErrorInvalidParameterValue,
            CX_INVALID_PARAMETER => AppErrors::CxErrorInvalidParameter,
            CX_NOT_INVERTIBLE => AppErrors::CxErrorNotInvertible,
            CX_OVERFLOW => AppErrors::CxErrorOverflow,
            CX_MEMORY_FULL => AppErrors::CxErrorMemoryFull,
            CX_NO_RESIDUE => AppErrors::CxErrorNoResidue,
            CX_EC_INFINITE_POINT => AppErrors::CxErrorEcInfinitePoint,
            CX_EC_INVALID_POINT => AppErrors::CxErrorEcInvalidPoint,
            CX_EC_INVALID_CURVE => AppErrors::CxErrorEcInvalidCurve,
            _ => AppErrors::Unknown,
        }
    }
}
