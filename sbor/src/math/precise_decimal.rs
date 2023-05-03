use simple_bigint::bigint::{BigInt, BigIntError};

use crate::math::format_big_int;
use crate::static_vec::StaticVec;

#[derive(Copy, Clone)]
pub struct PreciseDecimal(BigInt<512>);

impl PreciseDecimal {
    pub const SCALE: usize = 64;
    pub const MAX_PRINT_LEN: usize = 160;
    pub const MAX: PreciseDecimal = PreciseDecimal(BigInt::from_limbs([
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
        0xFFFF_FFFF,
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
        0x0000_0000,
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
        format_big_int::<512, { Self::SCALE }, N>(&self.0, output);
    }
}

impl TryFrom<&[u8]> for PreciseDecimal {
    type Error = BigIntError;
    fn try_from(value: &[u8]) -> Result<Self, BigIntError> {
        Ok(Self(BigInt::<512>::from_bytes(value)?))
    }
}

#[cfg(test)]
mod tests {
    use core::fmt;
    use core::fmt::Write;

    use super::*;

    impl fmt::Display for PreciseDecimal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            let mut formatted = StaticVec::<u8, { Self::MAX_PRINT_LEN }>::new(0);
            format_big_int::<512, { Self::SCALE }, { Self::MAX_PRINT_LEN }>(
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
        let one = PreciseDecimal(
            BigInt::<512>::from_bytes(&[
                0, 0, 0, 0, 0, 0, 0, 0, 1, 31, 106, 191, 100, 237, 56, 110, 237, 151, 167, 218,
                244, 249, 63, 233, 3, 79, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ])
            .unwrap(),
        );

        assert_eq!(
            PreciseDecimal(1u128.into()).to_string(),
            "0.0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(
            PreciseDecimal(123456789123456789u128.into()).to_string(),
            "0.0000000000000000000000000000000000000000000000123456789123456789"
        );
        assert_eq!(one.to_string(), "1");
        assert_eq!(
            PreciseDecimal(123000000000000000000000000000000000000u128.into()).to_string(),
            "0.0000000000000000000000000123"
        );
        assert_eq!(
            PreciseDecimal(123456789123456789000000000000000000000u128.into()).to_string(),
            "0.0000000000000000000000000123456789123456789"
        );
        assert_eq!(
            PreciseDecimal(
                BigInt::<512>::from_bytes(&[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0xbe, 0xe5, 0x18, 0xac,
                    0xe9, 0xdd, 0x1d, 0x50, 0xb6, 0x62, 0x06, 0x59, 0x92, 0x19, 0x4b, 0x9e, 0x2b,
                    0x1d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                ])
                .unwrap()
            )
            .to_string(),
            "1.2"
        );
        assert_eq!(
            PreciseDecimal::MAX.to_string(),
            "670390396497129854978701249910292306373968291029619668886178072186088201503677348840093714.9083451713845015929093243025426876941405973284973216824503042047"
        );
        assert_eq!(PreciseDecimal::MIN.is_negative(), true);
        assert_eq!(
            PreciseDecimal::MIN.to_string(),
            "-670390396497129854978701249910292306373968291029619668886178072186088201503677348840093714.9083451713845015929093243025426876941405973284973216824503042048"
        );
    }
}
