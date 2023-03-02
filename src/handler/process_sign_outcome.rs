use crate::app_error::AppError;
use crate::tx_sign_state::SignOutcome;
use nanos_sdk::io::Comm;

pub fn process_sign_outcome(comm: &mut Comm, outcome: SignOutcome) -> Result<(), AppError> {
    match outcome {
        SignOutcome::SigningRejected => Err(AppError::BadTxSignUserRejected),
        SignOutcome::Signature { len, signature } => {
            comm.append(&signature[..(len as usize)]);
            Ok(())
        }
        SignOutcome::SendNextPacket => Ok(()),
    }
}
