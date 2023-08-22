use simple_bigint::bigint::{BigInt, BigIntError};

use crate::math::format_big_int;
use crate::static_vec::StaticVec;

#[derive(Copy, Clone)]
pub struct PreciseDecimal(BigInt<256>);

impl PreciseDecimal {
    pub const SIZE_IN_BYTES: usize = BigInt::<256>::NUM_BYTES;
    pub const SCALE: usize = 36;
    pub const MAX_PRINT_LEN: usize = 90;
    pub const MAX: PreciseDecimal = PreciseDecimal(BigInt::from_limbs([
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0x7FFF_FFFF,
    ]));
    pub const MIN: PreciseDecimal = PreciseDecimal(BigInt::from_limbs([
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x8000_0000,
    ]));

    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    pub fn format<const N: usize>(&self, output: &mut StaticVec<u8, N>) {
        format_big_int::<256, { Self::SCALE }, N>(&self.0, output);
    }
}

impl TryFrom<&[u8]> for PreciseDecimal {
    type Error = BigIntError;
    fn try_from(value: &[u8]) -> Result<Self, BigIntError> {
        Ok(Self(BigInt::<256>::from_bytes(value)?))
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use core::fmt;
    use core::fmt::Write;

    use super::*;

    impl fmt::Display for PreciseDecimal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            let mut formatted = StaticVec::<u8, { Self::MAX_PRINT_LEN }>::new(0);
            format_big_int::<256, { Self::SCALE }, { Self::MAX_PRINT_LEN }>(
                &self.0,
                &mut formatted,
            );

            for &byte in formatted.as_slice() {
                f.write_char(byte as char)?;
            }
            Ok(())
        }
    }

    #[test]
    pub fn test_precise_decimal_size() {
        assert_eq!(PreciseDecimal::SIZE_IN_BYTES, 32);
    }

    #[test]
    pub fn test_format_precise_decimal_orig() {
        assert_eq!(
            PreciseDecimal(1u128.into()).to_string(),
            "0.000000000000000000000000000000000001"
        );
        assert_eq!(
            PreciseDecimal(123456789123456789u128.into()).to_string(),
            "0.000000000000000000123456789123456789"
        );

    }

    #[test]
    pub fn test_format_precise_decimal() {
        let one = PreciseDecimal(
            BigInt::<256>::from_bytes(&[
                0x00, 0x00, 0x00, 0x00, 0x10, 0x9f, 0x4b, 0xb3,
                0x15, 0x07, 0xc9, 0x7b, 0xce, 0x97, 0xc0, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ])
            .unwrap(),
        );

        assert_eq!(PreciseDecimal(1u128.into()).to_string(), "0.000000000000000000000000000000000001");
        assert_eq!(PreciseDecimal(123456789123456789u128.into()).to_string(), "0.000000000000000000123456789123456789");
        assert_eq!(one.to_string(), "1");
        assert_eq!(PreciseDecimal(123000000000000000000000000000000000000u128.into()).to_string(), "123");
        assert_eq!(PreciseDecimal(123456789123456789000000000000000000000u128.into()).to_string(), "123.456789123456789");
        assert_eq!(PreciseDecimal(BigInt::<256>::from_bytes(&[
            0x00, 0x00, 0x00, 0x00, 0xe0, 0xbe, 0x5a, 0x0a,
            0x1a, 0xa2, 0x57, 0x61, 0x91, 0x1c, 0xe7, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ]).unwrap()
            ).to_string(), "1.2");
        assert_eq!(PreciseDecimal::MAX.to_string(), "57896044618658097711785492504343953926634.992332820282019728792003956564819967");
        assert_eq!(PreciseDecimal::MIN.is_negative(), true);
        assert_eq!(PreciseDecimal::MIN.to_string(), "-57896044618658097711785492504343953926634.992332820282019728792003956564819968");
    }

    const PRECISE_DECIMAL_1: [u8; 34] = [
        0x5c, 0xb0,
        0x00, 0x00, 0x00, 0x00, 0x10, 0x9f, 0x4b, 0xb3,
        0x15, 0x07, 0xc9, 0x7b, 0xce, 0x97, 0xc0, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const PRECISE_DECIMAL_10: [u8; 34] = [
        0x5c, 0xb0,
        0x00, 0x00, 0x00, 0x00, 0xa0, 0x36, 0xf4, 0x00,
        0xd9, 0x46, 0xda, 0xd5, 0x10, 0xee, 0x85, 0x07,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const PRECISE_DECIMAL_12: [u8; 34] = [
        0x5c, 0xb0,
        0x00, 0x00, 0x00, 0x00, 0xc0, 0x74, 0x8b, 0x67,
        0x04, 0x55, 0x6c, 0xcd, 0xad, 0x1d, 0x07, 0x09,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const PRECISE_DECIMAL_123: [u8; 34] = [
        0x5c, 0xb0,
        0x00, 0x00, 0x00, 0x00, 0xb0, 0x6c, 0x55, 0x25,
        0x6d, 0x67, 0x96, 0x79, 0x35, 0xf0, 0x88, 0x5c,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const PRECISE_DECIMAL_1_23456789: [u8; 34] = [
        0x5c, 0xb0,
        0x00, 0x00, 0x00, 0x50, 0xc6, 0x9b, 0xe1, 0x3b,
        0xb7, 0x7f, 0x00, 0x7e, 0xe5, 0xc4, 0xed, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const PRECISE_DECIMAL_0_23456789: [u8; 34] = [
        0x5c, 0xb0,
        0x00, 0x00, 0x00, 0x50, 0xb6, 0xfc, 0x95, 0x88,
        0xa1, 0x78, 0x37, 0x02, 0x17, 0x2d, 0x2d, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    #[test]
    fn test_decoding_precise_decimal() {
        assert_eq!(PreciseDecimal::try_from(&PRECISE_DECIMAL_1[2..]).unwrap().to_string(), "1");
        assert_eq!(PreciseDecimal::try_from(&PRECISE_DECIMAL_10[2..]).unwrap().to_string(), "10");
        assert_eq!(PreciseDecimal::try_from(&PRECISE_DECIMAL_12[2..]).unwrap().to_string(), "12");
        assert_eq!(PreciseDecimal::try_from(&PRECISE_DECIMAL_123[2..]).unwrap().to_string(), "123");
        assert_eq!(PreciseDecimal::try_from(&PRECISE_DECIMAL_1_23456789[2..]).unwrap().to_string(), "1.23456789");
        assert_eq!(PreciseDecimal::try_from(&PRECISE_DECIMAL_0_23456789[2..]).unwrap().to_string(), "0.23456789");
    }
}
