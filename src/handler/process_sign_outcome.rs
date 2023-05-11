use nanos_sdk::io::Comm;

use crate::app_error::AppError;
use crate::tx_sign_state::SignOutcome;

pub fn process_sign_outcome(comm: &mut Comm, outcome: SignOutcome) -> Result<(), AppError> {
    match outcome {
        SignOutcome::SigningRejected => Err(AppError::BadTxSignUserRejected),
        SignOutcome::SendNextPacket => Ok(()),
        SignOutcome::SignatureEd25519 {
            signature,
            key,
            digest,
        } => {
            comm.append(&signature);
            comm.append(&key);
            comm.append(&digest);
            Ok(())
        }
        SignOutcome::SignatureSecp256k1 {
            signature,
            key,
            digest,
        } => {
            comm.append(&signature);
            comm.append(&key);
            comm.append(&digest);
            Ok(())
        }
    }
}
