use nanos_sdk::io::Comm;
use crate::app_error::AppError;
use crate::tx_sign_state::SignOutcome;

pub fn process_sign_outcome(comm: &mut Comm, outcome: SignOutcome) -> Result<(), AppError> {
    match outcome {
        SignOutcome::SigningRejected => Err(AppError::BadTxSignUserRejected),
        SignOutcome::Signature(signature) => {
            comm.append(&signature);
            Ok(())
        },
        SignOutcome::SendNextPacket => Ok(())
    }
}
