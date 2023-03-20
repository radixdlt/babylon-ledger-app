use crate::math::MathError;
use core::fmt;
use core::fmt::Write;
use crypto_bigint::NonZero;
use crypto_bigint::{Zero, U256};
use staticvec::StaticVec;

#[derive(Copy, Clone)]
pub struct Decimal(U256);

impl Decimal {
    pub const SCALE: u32 = 18;
    pub const ZERO: Decimal = Decimal(U256::ZERO);
    pub const ONE: Decimal = Decimal(U256::from_u64(10_u64.pow(Decimal::SCALE)));
    pub const MAX: Self = Self(U256::MAX);

    const LOW_TEN: Decimal = Decimal(U256::from_u64(10u64));

    fn fmt_uint(uint: U256) -> StaticVec<u8, 80> {
        let divisor = NonZero::new(Decimal::LOW_TEN.0).unwrap();
        let mut value = uint;
        let mut vec = StaticVec::<u8, 80>::new();

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

impl TryFrom<&[u8]> for Decimal {
    type Error = MathError;
    fn try_from(value: &[u8]) -> Result<Self, MathError> {
        if value.len() != U256::BYTES {
            Err(MathError::InvalidSliceLen)
        } else {
            Ok(Self(U256::from_le_slice(value)))
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let divisor = NonZero::new(Decimal::ONE.0).unwrap();
        let (quotent, remainder) = self.0.div_rem(&divisor);
        let whole = Decimal::fmt_uint(quotent);
        let no_decimals: bool = remainder.is_zero().into();

        for byte in whole {
            f.write_char(byte as char)?;
        }

        if !no_decimals {
            let mut decimals = Decimal::fmt_uint(remainder);

            // Add leading zeros if necessary
            while decimals.len() < (Decimal::SCALE as usize) {
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
        assert_eq!(Decimal(1u128.into()).to_string(), "0.000000000000000001");
        assert_eq!(Decimal(123456789123456789u128.into()).to_string(), "0.123456789123456789");
        assert_eq!(Decimal(1000000000000000000u128.into()).to_string(), "1");
        assert_eq!(Decimal(123000000000000000000u128.into()).to_string(), "123");
        assert_eq!(Decimal(123456789123456789000000000000000000u128.into()).to_string(),"123456789123456789");
        assert_eq!(Decimal::MAX.to_string(), "115792089237316195423570985008687907853269984665640564039457.584007913129639935");

        // Signed numbers are not supported yet
        // assert_eq!(format!("{}", Decimal::MAX),"57896044618658097711785492504343953926634992332820282019728.792003956564819967");
        // assert_eq!(Decimal::MIN.is_negative(), true);
        // assert_eq!(format!("{}", Decimal::MIN), "-57896044618658097711785492504343953926634992332820282019728.792003956564819968");
    }
}
