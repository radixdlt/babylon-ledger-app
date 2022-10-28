const MAX_ARR_LEN: usize = 256;

#[inline]
pub fn to_hex(m: &[u8]) -> Result<[u8; MAX_ARR_LEN], ()> {
    if 2 * m.len() > MAX_ARR_LEN {
        return Err(());
    }
    let mut hex = [0u8; MAX_ARR_LEN];
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

pub fn to_hex_str(m: u32) -> [u8; 8] {
    let mut res = [0u8; 8];

    let b0 = (m & 0xFF000000u32) >> 24 as u8;
    let b1 = (m & 0x00FF0000u32) >> 16 as u8;
    let b2 = (m & 0x0000FF00u32) >> 8 as u8;
    let b3 = (m & 0x000000FFu32) as u8;

    res[0] = char::from_digit((b0 >> 4).into(), 16).unwrap() as u8;
    res[1] = char::from_digit((b0 & 0xf).into(), 16).unwrap() as u8;
    res[2] = char::from_digit((b1 >> 4).into(), 16).unwrap() as u8;
    res[3] = char::from_digit((b1 & 0xf).into(), 16).unwrap() as u8;
    res[4] = char::from_digit((b2 >> 4).into(), 16).unwrap() as u8;
    res[5] = char::from_digit((b2 & 0xf).into(), 16).unwrap() as u8;
    res[6] = char::from_digit((b3 >> 4).into(), 16).unwrap() as u8;
    res[7] = char::from_digit((b3 & 0xf).into(), 16).unwrap() as u8;

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
    ((bytes[0] as u32) << 0)
        + ((bytes[1] as u32) << 8)
        + ((bytes[2] as u32) << 16)
        + ((bytes[3] as u32) << 24)
}

pub fn read_u32_be(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24)
        + ((bytes[1] as u32) << 16)
        + ((bytes[2] as u32) << 8)
        + ((bytes[3] as u32) << 0)
}
