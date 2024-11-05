use crate::xui::titled_message;

#[cfg(not(target_os = "stax"))]
pub fn display(message_hex: &[u8]) {
    titled_message::display(b"Pre-auth Hash:", message_hex);
}

#[cfg(target_os = "stax")]
pub fn display(message_hex: &[u8]) {}
