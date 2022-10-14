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

const fn bytes_to_number(bytes: &[u8]) -> u8 {
    let mut i = 0;
    let mut acc = 0;

    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'0'..=b'9' => {
                acc = (c - b'0') as u32;
            }
            _ => panic!("expected digit"),
        }
        i += 1;
    }

    if acc > 255 {
        panic!("too big version element value");
    }

    acc as u8
}
