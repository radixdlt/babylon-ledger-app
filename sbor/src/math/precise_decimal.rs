use num_traits::{One, Pow, Zero};
use core::ops::*;
use core::fmt;
use arrform::{ArrForm, arrform};

use crate::math::bnum_integer::*;
use crate::math::decimal::*;
use crate::math::rounding_mode::*;
use crate::math::byte_receiver::ByteReceiver;

/// `PreciseDecimal` represents a 512 bit representation of a fixed-scale decimal number.
///
/// The finite set of values are of the form `m / 10^64`, where `m` is
/// an integer such that `-2^(512 - 1) <= m < 2^(512 - 1)`.
///
/// Unless otherwise specified, all operations will panic if underflow/overflow.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreciseDecimal(pub BnumI512);

impl Default for PreciseDecimal {
    fn default() -> Self {
        Self::zero()
    }
}

impl PreciseDecimal {
    /// The min value of `PreciseDecimal`.
    pub const MIN: Self = Self(BnumI512::MIN);

    /// The max value of `PreciseDecimal`.
    pub const MAX: Self = Self(BnumI512::MAX);

    /// The bit length of number storing `PreciseDecimal`.
    pub const BITS: usize = BnumI512::BITS as usize;

    /// The fixed scale used by `PreciseDecimal`.
    pub const SCALE: u32 = 64;

    pub const ZERO: Self = Self(BnumI512::ZERO);

    pub const ONE: Self = Self(BnumI512::from_digits([
        0,
        7942358959831785217,
        16807427164405733357,
        1593091,
        0,
        0,
        0,
        0,
    ]));

    /// Returns `PreciseDecimal` of 0.
    pub fn zero() -> Self {
        Self::ZERO
    }

    /// Returns `PreciseDecimal` of 1.
    pub fn one() -> Self {
        Self::ONE
    }

    /// Whether this decimal is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == BnumI512::zero()
    }

    /// Whether this decimal is positive.
    pub fn is_positive(&self) -> bool {
        self.0 > BnumI512::zero()
    }

    /// Whether this decimal is negative.
    pub fn is_negative(&self) -> bool {
        self.0 < BnumI512::zero()
    }

    /// Returns the absolute value.
    pub fn abs(&self) -> PreciseDecimal {
        PreciseDecimal(self.0.abs())
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

        let divisor: BnumI512 = BnumI512::from(10i8).pow(Self::SCALE - decimal_places);
        match mode {
            RoundingMode::TowardsPositiveInfinity => {
                if self.0 % divisor == BnumI512::zero() {
                    self.clone()
                } else if self.is_negative() {
                    Self(self.0 / divisor * divisor)
                } else {
                    Self((self.0 / divisor + BnumI512::one()) * divisor)
                }
            }
            RoundingMode::TowardsNegativeInfinity => {
                if self.0 % divisor == BnumI512::zero() {
                    self.clone()
                } else if self.is_negative() {
                    Self((self.0 / divisor - BnumI512::one()) * divisor)
                } else {
                    Self(self.0 / divisor * divisor)
                }
            }
            RoundingMode::TowardsZero => {
                if self.0 % divisor == BnumI512::zero() {
                    self.clone()
                } else {
                    Self(self.0 / divisor * divisor)
                }
            }
            RoundingMode::AwayFromZero => {
                if self.0 % divisor == BnumI512::zero() {
                    self.clone()
                } else if self.is_negative() {
                    Self((self.0 / divisor - BnumI512::one()) * divisor)
                } else {
                    Self((self.0 / divisor + BnumI512::one()) * divisor)
                }
            }
            RoundingMode::TowardsNearestAndHalfTowardsZero => {
                if self.0 % divisor == BnumI512::zero() {
                    self.clone()
                } else {
                    let digit = (self.0 / (divisor / BnumI512::from(10i128))
                        % BnumI512::from(10i128))
                    .abs();
                    if digit > 5.into() {
                        if self.is_negative() {
                            Self((self.0 / divisor - BnumI512::one()) * divisor)
                        } else {
                            Self((self.0 / divisor + BnumI512::one()) * divisor)
                        }
                    } else {
                        Self(self.0 / divisor * divisor)
                    }
                }
            }
            RoundingMode::TowardsNearestAndHalfAwayFromZero => {
                if self.0 % divisor == BnumI512::zero() {
                    self.clone()
                } else {
                    let digit = (self.0 / (divisor / BnumI512::from(10i128))
                        % BnumI512::from(10i128))
                    .abs();
                    if digit < 5.into() {
                        Self(self.0 / divisor * divisor)
                    } else {
                        if self.is_negative() {
                            Self((self.0 / divisor - BnumI512::one()) * divisor)
                        } else {
                            Self((self.0 / divisor + BnumI512::one()) * divisor)
                        }
                    }
                }
            }
        }
    }

    /// Calculates power using exponentiation by squaring.
    pub fn powi(&self, exp: i64) -> Self {
        let one_768 = BnumI768::from(Self::ONE.0);
        let base_768 = BnumI768::from(self.0);
        let div = |x: i64, y: i64| x.checked_div(y).expect("Overflow");
        let sub = |x: i64, y: i64| x.checked_sub(y).expect("Overflow");
        let mul = |x: i64, y: i64| x.checked_mul(y).expect("Overflow");

        if exp < 0 {
            let sub_768 = one_768 * one_768 / base_768;
            let sub_512 = BnumI512::try_from(sub_768).expect("Overflow");
            return PreciseDecimal(sub_512).powi(mul(exp, -1));
        }
        if exp == 0 {
            return Self::ONE;
        }
        if exp == 1 {
            return *self;
        }
        if exp % 2 == 0 {
            let sub_768 = base_768 * base_768 / one_768;
            let sub_512 = BnumI512::try_from(sub_768).expect("Overflow");
            PreciseDecimal(sub_512).powi(div(exp, 2))
        } else {
            let sub_768 = base_768 * base_768 / one_768;
            let sub_512 = BnumI512::try_from(sub_768).expect("Overflow");
            let sub_pdec = PreciseDecimal(sub_512);
            *self * sub_pdec.powi(div(sub(exp, 1), 2))
        }
    }

    /// Square root of a PreciseDecimal
    pub fn sqrt(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }
        if self.is_zero() {
            return Some(Self::ZERO);
        }

        // The BnumI512 i associated to a Decimal d is : i = d*10^64.
        // Therefore, taking sqrt yields sqrt(i) = sqrt(d)*10^32 => We lost precision
        // To get the right precision, we compute : sqrt(i*10^64) = sqrt(d)*10^64
        let self_768 = BnumI768::from(self.0);
        let correct_nb = self_768 * BnumI768::from(PreciseDecimal::one().0);
        let sqrt = BnumI512::try_from(correct_nb.sqrt()).expect("Overflow");
        Some(PreciseDecimal(sqrt))
    }
}

macro_rules! from_int {
    ($type:ident) => {
        impl From<$type> for PreciseDecimal {
            fn from(val: $type) -> Self {
                Self(BnumI512::from(val) * Self::ONE.0)
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

impl From<bool> for PreciseDecimal {
    fn from(val: bool) -> Self {
        if val {
            Self::from(1u8)
        } else {
            Self::from(0u8)
        }
    }
}

impl<T: TryInto<PreciseDecimal>> Add<T> for PreciseDecimal
where
    <T as TryInto<PreciseDecimal>>::Error: fmt::Debug,
{
    type Output = PreciseDecimal;

    fn add(self, other: T) -> Self::Output {
        let a = self.0;
        let b_dec: PreciseDecimal = other.try_into().expect("Overflow");
        let b: BnumI512 = b_dec.0;
        let c = a + b;
        PreciseDecimal(c)
    }
}

impl<T: TryInto<PreciseDecimal>> Sub<T> for PreciseDecimal
where
    <T as TryInto<PreciseDecimal>>::Error: fmt::Debug,
{
    type Output = PreciseDecimal;

    fn sub(self, other: T) -> Self::Output {
        let a = self.0;
        let b_dec: PreciseDecimal = other.try_into().expect("Overflow");
        let b: BnumI512 = b_dec.0;
        let c: BnumI512 = a - b;
        PreciseDecimal(c)
    }
}

impl<T: TryInto<PreciseDecimal>> Mul<T> for PreciseDecimal
where
    <T as TryInto<PreciseDecimal>>::Error: fmt::Debug,
{
    type Output = PreciseDecimal;

    fn mul(self, other: T) -> Self::Output {
        // Use BnumI768 to not overflow.
        let a = BnumI768::from(self.0);
        let b_dec: PreciseDecimal = other.try_into().expect("Overflow");
        let b = BnumI768::from(b_dec.0);
        let c = a * b / BnumI768::from(Self::ONE.0);
        let c_512 = BnumI512::try_from(c).expect("Overflow");
        PreciseDecimal(c_512)
    }
}

impl<T: TryInto<PreciseDecimal>> Div<T> for PreciseDecimal
where
    <T as TryInto<PreciseDecimal>>::Error: fmt::Debug,
{
    type Output = PreciseDecimal;

    fn div(self, other: T) -> Self::Output {
        // Use BnumI768 to not overflow.
        let a = BnumI768::from(self.0);
        let b_dec: PreciseDecimal = other.try_into().expect("Overflow");
        let b = BnumI768::from(b_dec.0);
        let c = a * BnumI768::from(Self::ONE.0) / b;
        let c_512 = BnumI512::try_from(c).expect("Overflow");
        PreciseDecimal(c_512)
    }
}

impl Neg for PreciseDecimal {
    type Output = PreciseDecimal;

    fn neg(self) -> Self::Output {
        PreciseDecimal(-self.0)
    }
}

impl<T: TryInto<PreciseDecimal>> AddAssign<T> for PreciseDecimal
where
    <T as TryInto<PreciseDecimal>>::Error: fmt::Debug,
{
    fn add_assign(&mut self, other: T) {
        let other: PreciseDecimal = other.try_into().expect("Overflow");
        self.0 += other.0;
    }
}

impl<T: TryInto<PreciseDecimal>> SubAssign<T> for PreciseDecimal
where
    <T as TryInto<PreciseDecimal>>::Error: fmt::Debug,
{
    fn sub_assign(&mut self, other: T) {
        let other: PreciseDecimal = other.try_into().expect("Overflow");
        self.0 -= other.0;
    }
}

impl<T: TryInto<PreciseDecimal>> MulAssign<T> for PreciseDecimal
where
    <T as TryInto<PreciseDecimal>>::Error: fmt::Debug,
{
    fn mul_assign(&mut self, other: T) {
        let other: PreciseDecimal = other.try_into().expect("Overflow");
        self.0 *= other.0;
    }
}

impl<T: TryInto<PreciseDecimal>> DivAssign<T> for PreciseDecimal
where
    <T as TryInto<PreciseDecimal>>::Error: fmt::Debug,
{
    fn div_assign(&mut self, other: T) {
        let other: PreciseDecimal = other.try_into().expect("Overflow");
        self.0 /= other.0;
    }
}

//========
// binary
//========

impl TryFrom<&[u8]> for PreciseDecimal {
    type Error = ParsePreciseDecimalError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() == Self::BITS / 8 {
            match BnumI512::try_from(slice) {
                Ok(val) => Ok(Self(val)),
                Err(_) => Err(ParsePreciseDecimalError::Overflow),
            }
        } else {
            Err(ParsePreciseDecimalError::InvalidLength(slice.len()))
        }
    }
}

//======
// text
//======

impl PreciseDecimal {
    pub fn fmt(&self, output: &mut dyn ByteReceiver) {
        const MULTIPLIER: BnumI512 = PreciseDecimal::ONE.0;
        let quotient = self.0 / MULTIPLIER;
        let remainder = self.0 % MULTIPLIER;

        let text = if !remainder.is_zero() {
            // print remainder with leading zeroes
            let mut sign = false;

            // take care of sign in case quotient == zere and remainder < 0,
            // eg.
            //  self.0=-100000000000000000 -> -0.1
            if remainder < BnumI512::ZERO && quotient == BnumI512::ZERO {
                sign = true;
            }

            arrform!({128 + 32}, "{}{}.{:064}", if sign {"-"} else {""}, quotient, remainder.abs())
        } else {
            arrform!({128 + 32}, "{}", quotient)
        };

        output.push_all(text.as_bytes());
    }
}

//========
// ParseDecimalError, ParsePreciseDecimalError
//========

/// Represents an error when parsing PreciseDecimal from another type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsePreciseDecimalError {
    InvalidLength(usize),
    Overflow,
}

impl From<Decimal> for PreciseDecimal {
    fn from(val: Decimal) -> Self {
        Self(
            BnumI512::try_from(val.0).unwrap()
                * BnumI512::from(10i8).pow(Self::SCALE - Decimal::SCALE),
        )
    }
}

pub trait Truncate<T> {
    type Output;
    fn truncate(self) -> Self::Output;
}

impl Truncate<Decimal> for PreciseDecimal {
    type Output = Decimal;

    fn truncate(self) -> Self::Output {
        Decimal(
            (self.0 / BnumI512::from(10i8).pow(PreciseDecimal::SCALE - Decimal::SCALE))
                .try_into()
                .expect("Overflow"),
        )
    }
}

macro_rules! from_integer {
    ($($t:ident),*) => {
        $(
            impl From<$t> for PreciseDecimal {
                fn from(val: $t) -> Self {
                    Self(BnumI512::from(val) * Self::ONE.0)
                }
            }
        )*
    };
}
macro_rules! try_from_integer {
    ($($t:ident),*) => {
        $(
            impl TryFrom<$t> for PreciseDecimal {
                type Error = ParsePreciseDecimalError;

                fn try_from(val: $t) -> Result<Self, Self::Error> {
                    match BnumI512::try_from(val) {
                        Ok(val) => {
                            match val.checked_mul(Self::ONE.0) {
                                Some(mul) => Ok(Self(mul)),
                                None => Err(ParsePreciseDecimalError::Overflow),
                            }
                        },
                        Err(_) => Err(ParsePreciseDecimalError::Overflow),
                    }
                }
            }
        )*
    };
}

from_integer!(BnumI256, BnumU256);
try_from_integer!(BnumI512, BnumU512);

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

    fn assert_equals(precise_decimal: PreciseDecimal, expected: &[u8]) {
        let mut buffer = ByteBuffer::new();
        precise_decimal.fmt(&mut buffer);
        assert_eq!(&buffer.data(), &expected);
    }

    #[test]
    fn test_format_precise_decimal() {
        // assert_equals(PreciseDecimal(1i128.into()), b"0.0000000000000000000000000000000000000000000000000000000000000001");
        // assert_equals(PreciseDecimal(123456789123456789i128.into()), b"0.0000000000000000000000000000000000000000000000123456789123456789");
        // assert_equals(PreciseDecimal(BnumI512::from(10).pow(64)), b"1");
        // assert_equals(PreciseDecimal(BnumI512::from(10).pow(64).mul(BnumI512::from(123))), b"123");
        assert_equals(PreciseDecimal::MAX, b"670390396497129854978701249910292306373968291029619668886178072186088201503677348840093714.9083451713845015929093243025426876941405973284973216824503042047");
        assert_equals(PreciseDecimal::MIN, b"-670390396497129854978701249910292306373968291029619668886178072186088201503677348840093714.9083451713845015929093243025426876941405973284973216824503042048");
        // assert_eq!(PreciseDecimal::MIN.is_negative(), true);
    }
}
