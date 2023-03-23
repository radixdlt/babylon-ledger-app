use crate::math::MathError;
use core::fmt;
use core::fmt::Write;
use crypto_bigint::NonZero;
use crypto_bigint::{Zero, U512};
use staticvec::StaticVec;

#[derive(Copy, Clone)]
pub struct PreciseDecimal(U512);

impl PreciseDecimal {
    pub const SCALE: u32 = 64;
    pub const ZERO: PreciseDecimal = PreciseDecimal(U512::ZERO);
    pub const ONE: PreciseDecimal = PreciseDecimal(U512::from_words([
        0,
        7942358959831785217,
        16807427164405733357,
        1593091,
        0,
        0,
        0,
        0,
    ]));
    pub const MAX: Self = Self(U512::MAX);
    pub const MAX_PRINT_LEN: usize = 180;

    const LOW_TEN: PreciseDecimal = PreciseDecimal(U512::from_u64(10u64));

    fn fmt_uint(uint: U512) -> StaticVec<u8, { Self::MAX_PRINT_LEN }> {
        let divisor = NonZero::new(PreciseDecimal::LOW_TEN.0).unwrap();
        let mut value = uint;
        let mut vec = StaticVec::<u8, { Self::MAX_PRINT_LEN }>::new();

        loop {
            let (quotent, remainder) = value.div_rem(&divisor);
            vec.insert(0, remainder.as_words()[0] as u8 + b'0');

            if quotent.is_zero().into() {
                break;
            }
            value = quotent;
        }

        vec
    }
}

impl TryFrom<&[u8]> for PreciseDecimal {
    type Error = MathError;
    fn try_from(value: &[u8]) -> Result<Self, MathError> {
        if value.len() != U512::BYTES {
            Err(MathError::InvalidSliceLen)
        } else {
            Ok(Self(U512::from_le_slice(value)))
        }
    }
}

impl fmt::Display for PreciseDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let divisor = NonZero::new(PreciseDecimal::ONE.0).unwrap();
        let (quotent, remainder) = self.0.div_rem(&divisor);
        let whole = PreciseDecimal::fmt_uint(quotent);
        let no_decimals: bool = remainder.is_zero().into();

        for byte in whole {
            f.write_char(byte as char)?;
        }

        if !no_decimals {
            let mut decimals = PreciseDecimal::fmt_uint(remainder);

            // Add leading zeros if necessary
            while decimals.len() < (PreciseDecimal::SCALE as usize) {
                decimals.insert(0, b'0');
            }

            // Remove trailing zeros if necessary
            while let Some(b'0') = decimals.get(decimals.len() - 1) {
                decimals.remove(decimals.len() - 1);
            }

            f.write_char('.')?;

            for byte in decimals {
                f.write_char(byte as char)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_format_decimal() {
        assert_eq!(
            PreciseDecimal(1u128.into()).to_string(),
            "0.0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(
            PreciseDecimal(123456789123456789u128.into()).to_string(),
            "0.0000000000000000000000000000000000000000000000123456789123456789"
        );
        assert_eq!(
            PreciseDecimal::ONE.to_string(),
            "1"
        );
        assert_eq!(
            PreciseDecimal(123000000000000000000000000000000000000u128.into()).to_string(),
            "0.0000000000000000000000000123"
        );
        assert_eq!(
            PreciseDecimal(123456789123456789000000000000000000000u128.into()).to_string(),
            "0.0000000000000000000000000123456789123456789"
        );
        assert_eq!(
            PreciseDecimal::MAX.to_string(),
            "1340780792994259709957402499820584612747936582059239337772356144372176403007354697680187429.8166903427690031858186486050853753882811946569946433649006084095"
        );

        // Signed numbers are not supported yet
        // assert_eq!(format!("{}", PreciseDecimal::MAX),"57896044618658097711785492504343953926634992332820282019728.792003956564819967");
        // assert_eq!(PreciseDecimal::MIN.is_negative(), true);
        // assert_eq!(format!("{}", PreciseDecimal::MIN), "-57896044618658097711785492504343953926634992332820282019728.792003956564819968");
    }
}
