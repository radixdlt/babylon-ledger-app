const CHARSET: [u8; 32] = *b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

#[derive(Copy, Clone, Debug)]
pub struct Bech32 {
    chk: u32,
    encoded: [u8; Self::MAX_LEN],
    actual_len: usize,
}

struct Base32Expander {
    expanded: [u8; Bech32::MAX_LEN],
    expanded_len: usize,
}

impl Base32Expander {
    fn new() -> Self {
        Self {
            expanded: [0; Bech32::MAX_LEN],
            expanded_len: 0,
        }
    }

    fn expanded(&mut self) -> &[u8] {
        &self.expanded[..self.expanded_len]
    }

    fn append(&mut self, byte: u8) {
        // NOTE: No bound checking as caller already verified that data fits into the buffer
        self.expanded[self.expanded_len] = byte;
        self.expanded_len += 1;
    }

    // Base32 expansion code is based on rust-bech32 implementation. Original copyright is cited below:
    // Copyright (c) 2017 Clark Moody
    //
    // Permission is hereby granted, free of charge, to any person obtaining a copy
    // of this software and associated documentation files (the "Software"), to deal
    // in the Software without restriction, including without limitation the rights
    // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    // copies of the Software, and to permit persons to whom the Software is
    // furnished to do so, subject to the following conditions:
    //
    // The above copyright notice and this permission notice shall be included in
    // all copies or substantial portions of the Software.
    //
    // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
    // THE SOFTWARE.
    fn expand(&mut self, data: &[u8]) -> Result<(), Bech32Error> {
        let max_len = (data.len() * 8 + 4) / 5;

        if max_len > self.expanded.len() {
            return Err(Bech32Error::InputTooLong);
        }

        let mut remaining_bits = 0u32;
        let mut work_buffer: u8 = 0;

        for &b in data {
            if remaining_bits >= 5 {
                self.append((work_buffer & 0b1111_1000) >> 3);
                work_buffer <<= 5;
                remaining_bits -= 5;
            }

            let from_buffer = work_buffer >> 3;
            let from_byte = b >> (3 + remaining_bits);

            self.append(from_buffer | from_byte);
            work_buffer = b << (5 - remaining_bits);
            remaining_bits += 3;
        }

        if remaining_bits >= 5 {
            self.append((work_buffer & 0b1111_1000) >> 3);
            work_buffer <<= 5;
            remaining_bits -= 5;
        }

        if remaining_bits != 0 {
            self.append(work_buffer >> 3);
        }

        Ok(())
    }
}

impl Bech32 {
    pub const MAX_LEN: usize = 90; // BIP173
    pub const HRP_MAX_LEN: usize = 83; // BIP173
    const BECH32M_CONSTANT: u32 = 0x2bc830a3; // BIP350

    fn new() -> Self {
        Self {
            chk: 1,
            encoded: [0; Self::MAX_LEN],
            actual_len: 0,
        }
    }

    pub fn encode(hrp: &[u8], data: &[u8]) -> Result<Bech32, Bech32Error> {
        let mut expander = Base32Expander::new();
        expander.expand(data)?;

        Self::encode_from_base32(hrp, expander.expanded())
    }

    pub fn encode_from_base32(hrp: &[u8], data: &[u8]) -> Result<Bech32, Bech32Error> {
        Self::check_hrp(hrp)?;
        let mut encoder = Bech32::new();
        encoder.encode_hrp(hrp)?;
        encoder.append_separator()?;
        encoder.encode_data(data)?;
        encoder.encode_checksum()?;
        Ok(encoder)
    }

    pub fn encoded(&self) -> &[u8] {
        &self.encoded[..self.actual_len]
    }

    fn check_hrp(hrp: &[u8]) -> Result<(), Bech32Error> {
        if hrp.len() == 0 || hrp.len() > Self::HRP_MAX_LEN {
            return Err(Bech32Error::InvalidHrpLen);
        }

        for b in hrp {
            if !(33..=126).contains(b) {
                return Err(Bech32Error::InvalidHrpChar);
            }

            if (b'A'..b'Z').contains(b) {
                return Err(Bech32Error::UpperCaseNotSupported);
            }
        }

        Ok(())
    }

    fn encode_hrp(&mut self, hrp: &[u8]) -> Result<(), Bech32Error> {
        for &ch in hrp {
            self.polymod_step(ch >> 5);
        }

        self.polymod_step(0);

        for &ch in hrp {
            self.polymod_step(ch & 0x1F);
            self.append(ch)?;
        }

        Ok(())
    }

    fn append_separator(&mut self) -> Result<(), Bech32Error> {
        self.append(b'1')
    }

    fn encode_data(&mut self, data: &[u8]) -> Result<(), Bech32Error> {
        for &byte in data {
            if (byte >> 5) != 0 {
                return Err(Bech32Error::InvalidDataByte);
            }

            self.polymod_step(byte);
            self.append(CHARSET[byte as usize])?;
        }
        Ok(())
    }

    fn encode_checksum(&mut self) -> Result<(), Bech32Error> {
        for _ in 0..6 {
            self.polymod_step(0);
        }

        self.chk ^= Self::BECH32M_CONSTANT;

        for i in 0..6 {
            let byte = (self.chk >> ((5 - i) * 5)) as u8;
            self.append(CHARSET[(byte & 0x1f) as usize])?;
        }
        Ok(())
    }

    fn append(&mut self, byte: u8) -> Result<(), Bech32Error> {
        if self.actual_len == Self::MAX_LEN {
            return Err(Bech32Error::EncodedAddressTooLong);
        }

        self.encoded[self.actual_len] = byte;
        self.actual_len += 1;

        Ok(())
    }

    const GEN: [u32; 5] = [
        0x3b6a_57b2,
        0x2650_8e6d,
        0x1ea1_19fa,
        0x3d42_33dd,
        0x2a14_62b3,
    ];

    fn polymod_step(&mut self, byte: u8) {
        let b = (self.chk >> 25) as u8;
        self.chk = (self.chk & 0x01ff_ffff) << 5 ^ (byte as u32);

        for (i, item) in Self::GEN.iter().enumerate() {
            if (b >> i) & 1 == 1 {
                self.chk ^= item;
            }
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bech32Error {
    InvalidHrpLen,
    UpperCaseNotSupported,
    InvalidHrpChar,
    InvalidDataByte,
    EncodedAddressTooLong,
    InputTooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_encode(hrp: &[u8], data: &[u8], encoded: &[u8]) {
        match Bech32::encode_from_base32(hrp, data) {
            Ok(data) => assert_eq!(data.encoded(), encoded),
            Err(err) => assert!(
                false,
                "Encoding failed for hrp:{:?}, data: {:?} with error {:?}",
                hrp, data, err
            ),
        }
    }

    #[test]
    fn test_hrp_len() {
        match Bech32::encode_from_base32(b"", &[1u8, 2, 3, 4]) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Bech32Error::InvalidHrpLen),
        };
    }

    #[test]
    fn test_hrp_case() {
        match Bech32::encode_from_base32(b"A", &[1u8, 2, 3, 4]) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, Bech32Error::UpperCaseNotSupported),
        };
    }

    #[test]
    fn test_valid_encode1() {
        test_encode(b"a", &[], b"a1lqfn3a");
    }
    #[test]
    fn test_valid_encode2() {
        test_encode(b"an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber1", &[], b"an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11sg7hg6");
    }
    #[test]
    fn test_valid_encode3() {
        test_encode(
            b"abcdef",
            &[
                31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11,
                10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
            ],
            b"abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx",
        );
    }
    #[test]
    fn test_valid_encode4() {
        test_encode(b"1", &[31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31], b"11llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllludsr8");
    }
    #[test]
    fn test_valid_encode5() {
        test_encode(
            b"split",
            &[
                24, 23, 25, 24, 22, 28, 1, 16, 11, 29, 8, 25, 23, 29, 19, 13, 16, 23, 29, 22, 25,
                28, 1, 16, 11, 3, 25, 29, 27, 25, 3, 3, 29, 19, 11, 25, 3, 3, 25, 13, 24, 29, 1,
                25, 3, 3, 25, 13,
            ],
            b"split1checkupstagehandshakeupstreamerranterredcaperredlc445v",
        );
    }
    #[test]
    fn test_valid_encode6() {
        test_encode(b"?", &[], b"?1v759aa");
    }
    #[test]
    fn test_valid_encode7() {
        test_encode(
            b"lntb",
            &[
                9, 1, 18, 22, 24, 27, 3, 15, 4, 1, 11, 22, 30, 28, 19, 12, 12, 16, 16, 16,
            ],
            b"lntb1fpjkcmr0yptk7unvvsssm7flcy",
        );
    }
}
