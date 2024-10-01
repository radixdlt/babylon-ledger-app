use crate::app_error::AppError;
use crate::sign::sign_mode::SignMode;
use crate::ui::single_message::SingleMessage;

#[cfg(not(target_os = "stax"))]
pub fn display(sign_mode: SignMode) -> Result<(), AppError> {
    let text = match sign_mode {
        SignMode::Ed25519Verbose
        | SignMode::Secp256k1Verbose
        | SignMode::Ed25519Summary
        | SignMode::Secp256k1Summary => "Review\n\nTransaction",
        SignMode::AuthEd25519 | SignMode::AuthSecp256k1 => "Review\nOwnership\nProof",
        SignMode::Ed25519PreAuthHash => "Review\nPre-authorization\nHash",
        SignMode::Ed25519Subintent => "Review\nPre-authorization",
    };

    SingleMessage::with_right_arrow(text).show_and_wait();

    Ok(())
}

#[cfg(target_os = "stax")]
pub fn display(sign_mode: SignMode) -> Result<(), AppError> {
    Ok(())
}
