use crate::utilities::conversion::bytes_to_number;

// Note that CLion Rust plugin marks all choices as inactive, there is no way to configure this
#[cfg(target_os = "nanos")]
pub const MODEL_DATA: [u8; 1] = [0];
#[cfg(target_os = "nanosplus")]
pub const MODEL_DATA: [u8; 1] = [1];
#[cfg(target_os = "nanox")]
pub const MODEL_DATA: [u8; 1] = [2];

const VERSION_MAJOR: &[u8] = env!("CARGO_PKG_VERSION_MAJOR").as_bytes();
const VERSION_MINOR: &[u8] = env!("CARGO_PKG_VERSION_MINOR").as_bytes();
const VERSION_PATCH: &[u8] = env!("CARGO_PKG_VERSION_PATCH").as_bytes();

pub const VERSION_DATA: [u8; 3] = [
    bytes_to_number(VERSION_MAJOR),
    bytes_to_number(VERSION_MINOR),
    bytes_to_number(VERSION_PATCH),
];
