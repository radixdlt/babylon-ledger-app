use nanos_ui::ui;

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

#[inline]
pub fn to_hex(m: &[u8]) -> Result<[u8; 64], ()> {
    if 2 * m.len() > 64 {
        return Err(());
    }
    let mut hex = [0u8; 64];
    let mut i = 0;
    for c in m {
        let c0 = char::from_digit((c >> 4).into(), 16).unwrap();
        let c1 = char::from_digit((c & 0xf).into(), 16).unwrap();
        hex[i] = c0 as u8;
        hex[i + 1] = c1 as u8;
        i += 2;
    }
    Ok(hex)
}

pub fn to_str(m: u32) -> [u8; 10] {
    let mut res = [0u8; 10];
    let mut val = m;

    for i in 0..9 {
        res[9 - i] = (b'0' as u32 + (val % 10)) as u8;
        val /= 10;
    }

    res
}

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

pub fn debug(message: &str) {
    ui::MessageScroller::new(message).event_loop();
}
