use crate::math::MathError;
use core::fmt;
use core::fmt::Write;
use crypto_bigint::subtle::Choice;
use crypto_bigint::NonZero;
use crypto_bigint::{Zero, U256};
use staticvec::StaticVec;

#[derive(Copy, Clone)]
pub struct Decimal(U256);

impl Decimal {
    pub const SCALE: u32 = 18;
    pub const ZERO: Decimal = Decimal(U256::ZERO);
    pub const ONE: Decimal = Decimal(U256::from_u64(10_u64.pow(Decimal::SCALE)));
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

            // TODO: is it necessary?
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
    pub fn test_fmt_uint() {
        let uint = U256::from_u64(123455667789u64);
        let vec = Decimal::fmt_uint(uint);

        println!(">>{}<<", Decimal(uint));
    }
}
