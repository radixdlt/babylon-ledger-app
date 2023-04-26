use simple_bigint::bigint::{BigInt, BigIntError};

use crate::math::format_big_int;
use crate::static_vec::StaticVec;

#[derive(Copy, Clone)]
pub struct Decimal(BigInt<256>);

impl Decimal {
    pub const SCALE: usize = 18;
    // 2ˆ256 = 1.1579209e+77, 78 digits + 1 decimal point + 1 sign = 80
    // 2ˆ128 = 3.4028237e+38, 39 digits + 1 decimal point + 1 sign = 41
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

    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    pub fn format<const N: usize>(&self, output: &mut StaticVec<u8, N>) {
        format_big_int::<256, { Self::SCALE }, N>(&self.0, output);
    }
}

impl TryFrom<&[u8]> for Decimal {
    type Error = BigIntError;
    fn try_from(value: &[u8]) -> Result<Self, BigIntError> {
        Ok(Self(BigInt::<256>::from_bytes(value)?))
    }
}

#[cfg(test)]
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
        assert_eq!(Decimal(1u128.into()).to_string(), "0.000000000000000001");
        assert_eq!(Decimal(1000000000000000u128.into()).to_string(), "0.001");
        assert_eq!(Decimal(10000000000000000u128.into()).to_string(), "0.01");
        assert_eq!(Decimal(100000000000000000u128.into()).to_string(), "0.1");
        assert_eq!(Decimal(001000000000000000000u128.into()).to_string(),   "1");
        assert_eq!(Decimal(001200000000000000000u128.into()).to_string(),   "1.2");
        assert_eq!(Decimal(012000000000000000000u128.into()).to_string(),  "12");
        assert_eq!(Decimal(012300000000000000000u128.into()).to_string(),  "12.3");
        assert_eq!(Decimal(120300000000000000000u128.into()).to_string(), "120.3");
        assert_eq!(
            Decimal(123456789123456789u128.into()).to_string(),
            "0.123456789123456789"
        );
        assert_eq!(Decimal(123000000000000000000u128.into()).to_string(), "123");
        assert_eq!(
            Decimal(123456789123456789000000000000000000u128.into()).to_string(),
            "123456789123456789"
        );
        assert_eq!(
            Decimal(
                BigInt::<256>::from_bytes(&[
                    0x00, 0x00, 0x78, 0x62, 0xa4, 0x41, 0xa7, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ])
                .unwrap()
            )
            .to_string(),
            "1.2"
        );
        assert_eq!(Decimal::MAX.to_string(),"57896044618658097711785492504343953926634992332820282019728.792003956564819967");
        assert_eq!(Decimal::MIN.is_negative(), true);
        assert_eq!(Decimal::MIN.to_string(), "-57896044618658097711785492504343953926634992332820282019728.792003956564819968");
    }
}
