use crypto_bigint::{NonZero, U512, Zero};
use crate::static_vec::StaticVec;

use crate::math::MathError;

#[derive(Copy, Clone)]
pub struct PreciseDecimal(U512);

impl PreciseDecimal {
    pub const SCALE: u32 = 64;
    pub const ZERO: PreciseDecimal = PreciseDecimal(U512::ZERO);
    pub const ONE: PreciseDecimal = PreciseDecimal(U512::from_le_slice(&[
        0, 0, 0, 0, 0, 0, 0, 0, 1, 31, 106, 191, 100, 237, 56, 110, 237, 151, 167, 218, 244, 249,
        63, 233, 3, 79, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]));
    pub const MAX: Self = Self(U512::MAX);
    pub const MAX_PRINT_LEN: usize = 160;

    const LOW_TEN: PreciseDecimal = PreciseDecimal(U512::from_u64(10u64));

    fn fmt_uint(uint: U512, vec: &mut StaticVec<u8, { Self::MAX_PRINT_LEN }>) -> u16 {
        let divisor = NonZero::new(PreciseDecimal::LOW_TEN.0).unwrap();
        let index = vec.len();
        let mut value = uint;
        let mut num_digits = 0;

        loop {
            let (quotent, remainder) = value.div_rem(&divisor);
            vec.insert(index, remainder.as_words()[0] as u8 + b'0');
            num_digits += 1;

            if quotent.is_zero().into() {
                break;
            }
            value = quotent;
        }
        num_digits
    }

    pub fn format(&self) -> StaticVec<u8, { Self::MAX_PRINT_LEN }> {
        let divisor = NonZero::new(PreciseDecimal::ONE.0).unwrap();
        let (quotent, remainder) = self.0.div_rem(&divisor);
        let no_decimals: bool = remainder.is_zero().into();
        let mut output = StaticVec::<u8, { Self::MAX_PRINT_LEN }>::new();

        PreciseDecimal::fmt_uint(quotent, &mut output);

        if !no_decimals {
            output.push(b'.');
            let decimal_start = output.len();
            let mut decimals = PreciseDecimal::fmt_uint(remainder, &mut output);

            // Add leading zeros if necessary
            while decimals < (PreciseDecimal::SCALE as u16) {
                output.insert(decimal_start as usize, b'0');
                decimals += 1;
            }

            // Remove trailing zeros if necessary
            while let Some(b'0') = output.get(output.len() - 1) {
                output.remove(output.len() - 1);
            }
        }

        output
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

#[cfg(test)]
mod tests {
    use core::fmt;
    use core::fmt::Write;

    use super::*;

    impl fmt::Display for PreciseDecimal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            let formatted = self.format();
            for byte in formatted {
                f.write_char(byte as char)?;
            }
            Ok(())
        }
    }

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
        assert_eq!(PreciseDecimal::ONE.to_string(), "1");
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
