use crate::ui::utils;

#[cfg(not(target_os = "stax"))]
pub fn display(address: &[u8], origin: &[u8], nonce_hex: &[u8]) {
    utils::info_message(b"Origin:", origin);
    utils::info_message(b"dApp Address:", address);
    utils::info_message(b"Nonce:", nonce_hex);
}

#[cfg(target_os = "stax")]
pub fn display(address: &[u8], origin: &[u8], nonce_hex: &[u8; 64]) {}
