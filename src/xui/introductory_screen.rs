use crate::app_error::AppError;
use crate::sign::sign_mode::SignMode;
use crate::ui::single_message::SingleMessage;

#[cfg(not(target_os = "stax"))]
pub fn display(sign_mode: SignMode) -> Result<(), AppError> {
    let text = match sign_mode {
        SignMode::TxEd25519Verbose
        | SignMode::TxSecp256k1Verbose
        | SignMode::TxEd25519Summary
        | SignMode::TxSecp256k1Summary => "Review\n\nTransaction",
        SignMode::AuthEd25519 | SignMode::AuthSecp256k1 => "Review\nOwnership\nProof",
        SignMode::PreAuthHashEd25519 | SignMode::PreAuthHashSecp256k1 => {
            "Review\nPre-authorization\nHash"
        }
        SignMode::PreAuthRawEd25519 | SignMode::PreAuthRawSecp256k1 => "Review\nPre-authorization",
    };

    SingleMessage::with_right_arrow(text).show_and_wait();

    Ok(())
}

#[cfg(target_os = "stax")]
pub fn display(sign_mode: SignMode) -> Result<(), AppError> {
    Ok(())
}
