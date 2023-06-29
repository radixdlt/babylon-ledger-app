use simple_bigint::bigint::{BigInt, BigIntError};

use crate::math::format_big_int;
use crate::static_vec::StaticVec;

#[derive(Copy, Clone, Debug)]
pub struct Decimal(BigInt<256>);

impl Decimal {
    pub const SIZE_IN_BYTES: usize = 32;
    pub const ZERO: Decimal = Decimal(BigInt::from_limbs([0, 0, 0, 0, 0, 0, 0, 0]));
    pub const SCALE: usize = 18;
    pub const MAX_PRINT_LEN: usize = 86;
    pub const MAX: Decimal = Decimal(BigInt::from_limbs([
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0x7FFF_FFFF,
    ]));
    pub const MIN: Decimal = Decimal(BigInt::from_limbs([
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x8000_0000,
    ]));

    pub fn new(value: u128) -> Self {
        Self(BigInt::from(value))
    }

    pub fn whole(value: u128) -> Decimal {
        Self(BigInt::from(value * 1000000000000000000u128))
    }

    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    pub fn format<const N: usize>(&self, output: &mut StaticVec<u8, N>) {
        format_big_int::<256, { Self::SCALE }, N>(&self.0, output);
    }

    pub fn accumulate(&mut self, other: &Self) -> &Self {
        self.0.accumulate(&other.0);
        self
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.0.is_same(&other.0)
    }
}

impl TryFrom<&[u8]> for Decimal {
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

    impl fmt::Display for Decimal {
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
    pub fn test_format_decimal() {
        assert_eq!(Decimal(                    1u128.into()).to_string(),   "0.000000000000000001");
        assert_eq!(Decimal(     1000000000000000u128.into()).to_string(),   "0.001");
        assert_eq!(Decimal(    10000000000000000u128.into()).to_string(),   "0.01");
        assert_eq!(Decimal(   100000000000000000u128.into()).to_string(),   "0.1");
        assert_eq!(Decimal(  1000000000000000000u128.into()).to_string(),   "1");
        assert_eq!(Decimal(  1200000000000000000u128.into()).to_string(),   "1.2");
        assert_eq!(Decimal( 10000000000000000000u128.into()).to_string(),  "10");
        assert_eq!(Decimal( 12000000000000000000u128.into()).to_string(),  "12");
        assert_eq!(Decimal( 12300000000000000000u128.into()).to_string(),  "12.3");
        assert_eq!(Decimal(120300000000000000000u128.into()).to_string(), "120.3");
        assert_eq!(Decimal(   123456789123456789u128.into()).to_string(),   "0.123456789123456789");
        assert_eq!(Decimal(123000000000000000000u128.into()).to_string(), "123");

        assert_eq!(Decimal(123456789123456789000000000000000000u128.into()).to_string(), "123456789123456789");
        assert_eq!(Decimal(BigInt::<256>::from_bytes(&[
            0x00, 0x00, 0x78, 0x62, 0xa4, 0x41, 0xa7, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ]).unwrap()).to_string(), "1.2");
        assert_eq!(Decimal::MAX.to_string(),
            //"57896044618658097711785492504343953926634992332820282019728.792003956564819967"
              "57896044618658097711785492504343953926634992332820282019728.792003956564819967");
        assert_eq!(Decimal::MIN.is_negative(), true);
        assert_eq!(Decimal::MIN.to_string(),
            //"-57896044618658097711785492504343953926634992332820282019728.792003956564819968"
              "-57896044618658097711785492504343953926634992332820282019728.792003956564819968"
        );
    }

    // ---------------------------------------------------- Decimal & PreciseDecimal
    const DECIMAL_1: [u8; 34] = [
        0x5c, 0xa0, 0x00, 0x00, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const DECIMAL_10: [u8; 34] = [
        0x5c, 0xa0, 0x00, 0x00, 0xe8, 0x89, 0x04, 0x23, 0xc7, 0x8a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const DECIMAL_12: [u8; 34] = [
        0x5c, 0xa0, 0x00, 0x00, 0xb0, 0xd8, 0x6b, 0x90, 0x88, 0xa6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const DECIMAL_123: [u8; 34] = [
        0x5c, 0xa0, 0x00, 0x00, 0x0c, 0x6d, 0x51, 0xc8, 0xf7, 0xaa, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const DECIMAL_1_23456789: [u8; 34] = [
        0x5c, 0xa0, 0x00, 0xb4, 0x8d, 0x76, 0xf4, 0x10, 0x22, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];
    const DECIMAL_0_23456789: [u8; 34] = [
        0x5c, 0xa0, 0x00, 0xb4, 0x29, 0xcf, 0x40, 0x5a, 0x41, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    #[test]
    fn test_decoding_decimal() {
        assert_eq!(Decimal::try_from(&DECIMAL_1[2..]).unwrap().to_string(), "1");
        assert_eq!(Decimal::try_from(&DECIMAL_10[2..]).unwrap().to_string(), "10");
        assert_eq!(Decimal::try_from(&DECIMAL_12[2..]).unwrap().to_string(), "12");
        assert_eq!(Decimal::try_from(&DECIMAL_123[2..]).unwrap().to_string(), "123");
        assert_eq!(Decimal::try_from(&DECIMAL_1_23456789[2..]).unwrap().to_string(), "1.23456789");
        assert_eq!(Decimal::try_from(&DECIMAL_0_23456789[2..]).unwrap().to_string(), "0.23456789");
    }
}
