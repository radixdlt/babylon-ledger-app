use crate::ui::multipage_validator::MultipageValidator;

#[repr(u8)]
pub enum SignType {
    TX,
    Proof,
}
#[cfg(not(target_os = "stax"))]
pub fn ask_user(sign_type: SignType) -> bool {
    let message = match sign_type {
        SignType::TX => &["Sign TX?"],
        SignType::Proof => &["Sign Proof?"],
    };
    MultipageValidator::new(message, &["Sign"], &["Reject"]).ask()
}
#[cfg(target_os = "stax")]
pub fn ask_user(sign_type: SignType) -> bool {
    false
}

