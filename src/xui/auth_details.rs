use crate::xui::titled_message;

#[cfg(not(target_os = "stax"))]
pub fn display(address: &[u8], origin: &[u8], nonce_hex: &[u8]) {
    titled_message::display(b"Origin:", origin);
    titled_message::display(b"dApp Address:", address);
    titled_message::display(b"Nonce:", nonce_hex);
}

#[cfg(target_os = "stax")]
pub fn display(address: &[u8], origin: &[u8], nonce_hex: &[u8; 64]) {}
