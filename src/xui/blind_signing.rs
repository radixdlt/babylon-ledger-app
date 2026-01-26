use crate::xui::titled_message;

#[cfg(not(target_os = "stax"))]
pub fn error() {
    titled_message::display_error("\nBlind signing must\nbe enabled in Settings");
}

#[cfg(target_os = "stax")]
pub fn error() {}
