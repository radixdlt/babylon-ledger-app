use crate::app_error::AppError;
use crate::sign::sign_outcome::SignOutcome;

pub fn process_sign_outcome(outcome: SignOutcome) -> Result<(), AppError> {
    match outcome {
        SignOutcome::SigningRejected => Err(AppError::BadTxSignUserRejected),
        SignOutcome::SendNextPacket
        | SignOutcome::SignatureEd25519
        | SignOutcome::SignatureSecp256k1 => Ok(()),
    }
}
