use crate::ui::multipage_validator::MultipageValidator;

#[repr(u8)]
pub enum SignType {
    TX,
    Proof,
    PreAuthHash,
}
#[cfg(not(target_os = "stax"))]
pub fn ask_user(sign_type: SignType) -> bool {
    let message: &[&str] = match sign_type {
        SignType::TX => &["Sign TX?"],
        SignType::Proof => &["Sign Proof?"],
        SignType::PreAuthHash => &["Sign", "Pre-auth?"],
    };
    MultipageValidator::new(message, &["Sign"], &["Reject"]).ask()
}
#[cfg(target_os = "stax")]
pub fn ask_user(sign_type: SignType) -> bool {
    false
}
