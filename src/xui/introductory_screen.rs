use crate::app_error::AppError;
use crate::sign::sign_mode::ReviewType;
use crate::ui::single_message::SingleMessage;

#[cfg(not(target_os = "stax"))]
pub fn display(review_type: ReviewType) -> Result<(), AppError> {
    let text = match review_type {
        ReviewType::Transaction => "Review\n\nTransaction",
        ReviewType::OwnershipProof => "Review\nOwnership\nProof",
        ReviewType::PreAuthHash => "Review\nPre-authorization\nHash",
        ReviewType::PreAuthRaw => "Review\nPre-authorization",
    };

    SingleMessage::with_right_arrow(text).show_and_wait();

    Ok(())
}

#[cfg(target_os = "stax")]
pub fn display(sign_mode: SignMode) -> Result<(), AppError> {
    Ok(())
}
