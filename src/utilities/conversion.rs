const MAX_ARR_LEN: usize = 64;
const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

#[inline(always)]
pub fn lower_as_hex(byte: u8) -> u8 {
    HEX_DIGITS[(byte & 0x0F) as usize]
}

#[inline(always)]
pub fn upper_as_hex(byte: u8) -> u8 {
    HEX_DIGITS[((byte >> 4) & 0x0F) as usize]
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

pub fn to_hex_str(m: u32) -> [u8; 8] {
    let mut res = [0u8; 8];

    let b0 = ((m & 0xFF000000u32) >> 24) as u8;
    let b1 = ((m & 0x00FF0000u32) >> 16) as u8;
    let b2 = ((m & 0x0000FF00u32) >> 8) as u8;
    let b3 = (m & 0x000000FFu32) as u8;

    res[0] = HEX_DIGITS[(b0 >> 4) as usize];
    res[1] = HEX_DIGITS[(b0 & 0xf) as usize];
    res[2] = HEX_DIGITS[(b1 >> 4) as usize];
    res[3] = HEX_DIGITS[(b1 & 0xf) as usize];
    res[4] = HEX_DIGITS[(b2 >> 4) as usize];
    res[5] = HEX_DIGITS[(b2 & 0xf) as usize];
    res[6] = HEX_DIGITS[(b3 >> 4) as usize];
    res[7] = HEX_DIGITS[(b3 & 0xf) as usize];

    res
}

pub const fn bytes_to_number(bytes: &[u8]) -> u8 {
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

pub fn read_u32_le(bytes: &[u8]) -> u32 {
    (bytes[0] as u32)
        + ((bytes[1] as u32) << 8)
        + ((bytes[2] as u32) << 16)
        + ((bytes[3] as u32) << 24)
}

pub fn read_u32_be(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24)
        + ((bytes[1] as u32) << 16)
        + ((bytes[2] as u32) << 8)
        + (bytes[3] as u32)
}
