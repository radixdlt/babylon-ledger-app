use arrform::{arrform, ArrForm};
use core::fmt;
use core::ops::*;
use num_traits::{One, Pow, Zero};

use crate::math::bnum_integer::*;
use crate::math::byte_receiver::ByteReceiver;
use crate::math::rounding_mode::*;

/// `Decimal` represents a 256 bit representation of a fixed-scale decimal number.
///
/// The finite set of values are of the form `m / 10^18`, where `m` is
/// an integer such that `-2^(256 - 1) <= m < 2^(256 - 1)`.
///
/// Unless otherwise specified, all operations will panic if underflow/overflow.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal(pub BnumI256);

impl Default for Decimal {
    fn default() -> Self {
        Self::zero()
    }
}

impl Decimal {
    /// The min value of `Decimal`.
    pub const MIN: Self = Self(BnumI256::MIN);

    /// The max value of `Decimal`.
    pub const MAX: Self = Self(BnumI256::MAX);

    /// The bit length of number storing `Decimal`.
    pub const BITS: usize = BnumI256::BITS as usize;

    /// The fixed scale used by `Decimal`.
    pub const SCALE: u32 = 18;

    pub const ZERO: Self = Self(BnumI256::ZERO);

    pub const ONE: Self = Self(BnumI256::from_digits([10_u64.pow(Decimal::SCALE), 0, 0, 0]));

    /// Returns `Decimal` of 0.
    pub fn zero() -> Self {
        Self::ZERO
    }

    /// Returns `Decimal` of 1.
    pub fn one() -> Self {
        Self::ONE
    }

    /// Whether this decimal is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == BnumI256::zero()
    }

    /// Whether this decimal is positive.
    pub fn is_positive(&self) -> bool {
        self.0 > BnumI256::zero()
    }

    /// Whether this decimal is negative.
    pub fn is_negative(&self) -> bool {
        self.0 < BnumI256::zero()
    }

    /// Returns the absolute value.
    pub fn abs(&self) -> Decimal {
        Decimal(self.0.abs())
    }

    /// Returns the largest integer that is equal to or less than this number.
    pub fn floor(&self) -> Self {
        self.round(0, RoundingMode::TowardsNegativeInfinity)
    }

    /// Returns the smallest integer that is equal to or greater than this number.
    pub fn ceiling(&self) -> Self {
        self.round(0, RoundingMode::TowardsPositiveInfinity)
    }

    pub fn round(&self, decimal_places: u32, mode: RoundingMode) -> Self {
        assert!(decimal_places <= Self::SCALE);

        let divisor: BnumI256 = BnumI256::from(10i8).pow(Self::SCALE - decimal_places);
        match mode {
            RoundingMode::TowardsPositiveInfinity => {
                if self.0 % divisor == BnumI256::zero() {
                    self.clone()
                } else if self.is_negative() {
                    Self(self.0 / divisor * divisor)
                } else {
                    Self((self.0 / divisor + BnumI256::one()) * divisor)
                }
            }
            RoundingMode::TowardsNegativeInfinity => {
                if self.0 % divisor == BnumI256::zero() {
                    self.clone()
                } else if self.is_negative() {
                    Self((self.0 / divisor - BnumI256::one()) * divisor)
                } else {
                    Self(self.0 / divisor * divisor)
                }
            }
            RoundingMode::TowardsZero => {
                if self.0 % divisor == BnumI256::zero() {
                    self.clone()
                } else {
                    Self(self.0 / divisor * divisor)
                }
            }
            RoundingMode::AwayFromZero => {
                if self.0 % divisor == BnumI256::zero() {
                    self.clone()
                } else if self.is_negative() {
                    Self((self.0 / divisor - BnumI256::one()) * divisor)
                } else {
                    Self((self.0 / divisor + BnumI256::one()) * divisor)
                }
            }
            RoundingMode::TowardsNearestAndHalfTowardsZero => {
                if self.0 % divisor == BnumI256::zero() {
                    self.clone()
                } else {
                    let digit = (self.0 / (divisor / BnumI256::from(10i128))
                        % BnumI256::from(10i128))
                    .abs();
                    if digit > 5.into() {
                        if self.is_negative() {
                            Self((self.0 / divisor - BnumI256::one()) * divisor)
                        } else {
                            Self((self.0 / divisor + BnumI256::one()) * divisor)
                        }
                    } else {
                        Self(self.0 / divisor * divisor)
                    }
                }
            }
            RoundingMode::TowardsNearestAndHalfAwayFromZero => {
                if self.0 % divisor == BnumI256::zero() {
                    self.clone()
                } else {
                    let digit = (self.0 / (divisor / BnumI256::from(10i128))
                        % BnumI256::from(10i128))
                    .abs();
                    if digit < 5.into() {
                        Self(self.0 / divisor * divisor)
                    } else {
                        if self.is_negative() {
                            Self((self.0 / divisor - BnumI256::one()) * divisor)
                        } else {
                            Self((self.0 / divisor + BnumI256::one()) * divisor)
                        }
                    }
                }
            }
        }
    }

    /// Calculates power using exponentiation by squaring".
    pub fn powi(&self, exp: i64) -> Self {
        let one_384 = BnumI384::from(Self::ONE.0);
        let base_384 = BnumI384::from(self.0);
        let div = |x: i64, y: i64| x.checked_div(y).expect("Overflow");
        let sub = |x: i64, y: i64| x.checked_sub(y).expect("Overflow");
        let mul = |x: i64, y: i64| x.checked_mul(y).expect("Overflow");

        if exp < 0 {
            let dec_256 = BnumI256::try_from(one_384 * one_384 / base_384).expect("Overflow");
            return Decimal(dec_256).powi(mul(exp, -1));
        }
        if exp == 0 {
            return Self::ONE;
        }
        if exp == 1 {
            return *self;
        }
        if exp % 2 == 0 {
            let dec_256 = BnumI256::try_from(base_384 * base_384 / one_384).expect("Overflow");
            Decimal(dec_256).powi(div(exp, 2))
        } else {
            let dec_256 = BnumI256::try_from(base_384 * base_384 / one_384).expect("Overflow");
            let sub_dec = Decimal(dec_256);
            *self * sub_dec.powi(div(sub(exp, 1), 2))
        }
    }
}

macro_rules! from_int {
    ($type:ident) => {
        impl From<$type> for Decimal {
            fn from(val: $type) -> Self {
                Self(BnumI256::from(val) * Self::ONE.0)
            }
        }
    };
}
from_int!(u8);
from_int!(u16);
from_int!(u32);
from_int!(u64);
from_int!(u128);
from_int!(usize);
from_int!(i8);
from_int!(i16);
from_int!(i32);
from_int!(i64);
from_int!(i128);
from_int!(isize);

impl From<bool> for Decimal {
    fn from(val: bool) -> Self {
        if val {
            Self::from(1u8)
        } else {
            Self::from(0u8)
        }
    }
}

impl<T: TryInto<Decimal>> Add<T> for Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    type Output = Decimal;

    fn add(self, other: T) -> Self::Output {
        let a = self.0;
        let b_dec: Decimal = other.try_into().expect("Overflow");
        let b: BnumI256 = b_dec.0;
        let c = a + b;
        Decimal(c)
    }
}

impl<T: TryInto<Decimal>> Sub<T> for Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    type Output = Decimal;

    fn sub(self, other: T) -> Self::Output {
        let a = self.0;
        let b_dec: Decimal = other.try_into().expect("Overflow");
        let b: BnumI256 = b_dec.0;
        let c: BnumI256 = a - b;
        Decimal(c)
    }
}

impl<T: TryInto<Decimal>> Mul<T> for Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    type Output = Decimal;

    fn mul(self, other: T) -> Self::Output {
        // Use BnumI384 (BInt<6>) to not overflow.
        let a = BnumI384::from(self.0);
        let b_dec: Decimal = other.try_into().expect("Overflow");
        let b = BnumI384::from(b_dec.0);
        let c = a * b / BnumI384::from(Self::ONE.0);
        let c_256 = BnumI256::try_from(c).expect("Overflow");
        Decimal(c_256)
    }
}

impl<T: TryInto<Decimal>> Div<T> for Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    type Output = Decimal;

    fn div(self, other: T) -> Self::Output {
        // Use BnumI384 (BInt<6>) to not overflow.
        let a = BnumI384::from(self.0);
        let b_dec: Decimal = other.try_into().expect("Overflow");
        let b = BnumI384::from(b_dec.0);
        let c = a * BnumI384::from(Self::ONE.0) / b;
        let c_256 = BnumI256::try_from(c).expect("Overflow");
        Decimal(c_256)
    }
}

impl Neg for Decimal {
    type Output = Decimal;

    fn neg(self) -> Self::Output {
        Decimal(-self.0)
    }
}

impl<T: TryInto<Decimal>> AddAssign<T> for Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    fn add_assign(&mut self, other: T) {
        let other: Decimal = other.try_into().expect("Overflow");
        self.0 += other.0;
    }
}

impl<T: TryInto<Decimal>> SubAssign<T> for Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    fn sub_assign(&mut self, other: T) {
        let other: Decimal = other.try_into().expect("Overflow");
        self.0 -= other.0;
    }
}

impl<T: TryInto<Decimal>> MulAssign<T> for Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    fn mul_assign(&mut self, other: T) {
        let other: Decimal = other.try_into().expect("Overflow");
        self.0 *= other.0;
    }
}

impl<T: TryInto<Decimal>> DivAssign<T> for Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    fn div_assign(&mut self, other: T) {
        let other: Decimal = other.try_into().expect("Overflow");
        self.0 /= other.0;
    }
}

//========
// binary
//========

impl TryFrom<&[u8]> for Decimal {
    type Error = ParseDecimalError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() == Self::BITS / 8 {
            match BnumI256::try_from(slice) {
                Ok(val) => Ok(Self(val)),
                Err(_) => Err(ParseDecimalError::Overflow),
            }
        } else {
            Err(ParseDecimalError::InvalidLength(slice.len()))
        }
    }
}

//======
// text
//======

impl Decimal {
    pub fn fmt(&self, output: &mut dyn ByteReceiver) {
        const MULTIPLIER: BnumI256 = Decimal::ONE.0;
        let quotient = self.0 / MULTIPLIER;
        let remainder = self.0 % MULTIPLIER;
        let mut strip_trailing_zeros = false;

        let text = if !remainder.is_zero() {
            // print remainder with leading zeroes
            let mut sign = false;
            strip_trailing_zeros = true;

            // take care of sign in case quotient == zere and remainder < 0,
            // eg.
            //  self.0=-100000000000000000 -> -0.1
            if remainder < BnumI256::ZERO && quotient == BnumI256::ZERO {
                sign = true;
            }

            arrform!(
                { 128 + 16 },
                "{}{}.{:018}",
                if sign { "-" } else { "" },
                quotient,
                remainder.abs()
            )
        } else {
            arrform!({ 128 + 16 }, "{}", quotient)
        };

        let bytes = text.as_bytes();
        let mut len : usize = bytes.len();

        if strip_trailing_zeros {
            while len > 0 && bytes[len - 1] == b'0' {
                len -= 1;
            }
        }

        output.push_all(&bytes[0..len]);
    }
}

//========
// ParseDecimalError, ParsePreciseDecimalError
//========

/// Represents an error when parsing Decimal from another type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseDecimalError {
    InvalidLength(usize),
    Overflow,
}

macro_rules! try_from_integer {
    ($($t:ident),*) => {
        $(
            impl TryFrom<$t> for Decimal {
                type Error = ParseDecimalError;

                fn try_from(val: $t) -> Result<Self, Self::Error> {
                    match BnumI256::try_from(val) {
                        Ok(val) => {
                            match val.checked_mul(Self::ONE.0) {
                                Some(mul) => Ok(Self(mul)),
                                None => Err(ParseDecimalError::Overflow),
                            }
                        },
                        Err(_) => Err(ParseDecimalError::Overflow),
                    }
                }
            }
        )*
    };
}
try_from_integer!(BnumI256, BnumI512, BnumU256, BnumU512);

#[cfg(test)]
mod tests {
    use super::*;
    struct ByteBuffer {
        data: [u8; 256],
        counter: usize,
    }

    impl ByteBuffer {
        fn new() -> Self {
            Self {
                data: [0; 256],
                counter: 0,
            }
        }

        fn data(&self) -> &[u8] {
            &self.data[0..self.counter]
        }
    }

    impl ByteReceiver for ByteBuffer {
        fn push(&mut self, byte: u8) {
            if self.counter < self.data.len() {
                self.data[self.counter] = byte;
                self.counter += 1;
            }
        }
    }

    fn assert_equals(decimal: Decimal, expected: &[u8]) {
        let mut buffer = ByteBuffer::new();
        decimal.fmt(&mut buffer);
        assert_eq!(&buffer.data(), &expected);
    }

    #[test]
    fn test_format_decimal() {
        assert_equals(Decimal(1i128.into()), b"0.000000000000000001");
        assert_equals(Decimal(123456789123456789i128.into()), b"0.123456789123456789");
        assert_equals(Decimal(1000000000000000000i128.into()), b"1");
        assert_equals(Decimal(123000000000000000000i128.into()), b"123");
        assert_equals(Decimal(123456789123456789000000000000000000i128.into()), b"123456789123456789");
        assert_equals(Decimal::MAX, b"57896044618658097711785492504343953926634992332820282019728.792003956564819967");
        assert_equals(Decimal::MIN, b"-57896044618658097711785492504343953926634992332820282019728.792003956564819968");

        assert_eq!(Decimal::MIN.is_negative(), true);
    }
}
