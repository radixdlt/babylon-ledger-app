use crate::app_error::AppError;
use crate::sign::sign_outcome::SignOutcome;
use crate::utilities::debug::display_memory;

pub fn process_sign_outcome(outcome: SignOutcome) -> Result<(), AppError> {
    display_memory(b'U'); //536873756
    match outcome {
        SignOutcome::SigningRejected => Err(AppError::BadTxSignUserRejected),
        SignOutcome::SendNextPacket
        | SignOutcome::SignatureEd25519
        | SignOutcome::SignatureSecp256k1 => Ok(()),
    }
}
