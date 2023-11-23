use crate::app_error::AppError;
use crate::io::Comm;

pub trait ParamsZero {
    fn check_params_zero(&self) -> Result<(), AppError>;
}

impl ParamsZero for Comm {
    fn check_params_zero(&self) -> Result<(), AppError> {
        let metadata = self.get_apdu_metadata();

        match (metadata.p1, metadata.p2) {
            (0u8, 0u8) => Ok(()),
            (_, _) => Err(AppError::BadP1P2),
        }
    }
}
