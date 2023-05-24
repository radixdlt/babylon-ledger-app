pub mod decimal;
pub mod precise_decimal;

pub use crate::static_vec::StaticVec;
pub use decimal::*;
pub use precise_decimal::*;
pub use simple_bigint::bcd::BCD;
pub use simple_bigint::bigint::{BigInt, BigIntError};
pub use simple_bigint::ceil_div;

pub fn format_big_int<const N: usize, const SCALE: usize, const DISPLAY_WIDTH: usize>(
    input: &BigInt<N>,
    output: &mut StaticVec<u8, DISPLAY_WIDTH>,
) where
    [(); ceil_div(4 * ceil_div(N, 3), 8)]:,
    [(); ceil_div(N, u32::BITS as usize)]:,
{
    let bcd = input.as_bcd();

    if bcd.is_zero() {
        output.push(b'0');
        return;
    }

    if bcd.is_negative() {
        output.push(b'-');
    }

    // Case when only Integer part is present
    if bcd.last_non_zero() >= SCALE {
        // Value
        for i in (SCALE..=bcd.first_non_zero()).rev() {
            output.push(b'0' + bcd.digit(i as usize));
        }

        // Trailing zeros
        for _ in bcd.last_non_zero()..SCALE {
            output.push(b'0');
        }

        return;
    }

    if bcd.first_non_zero() < SCALE {
        // Only Fractional part is present
        output.push(b'0');
        output.push(b'.');

        // Leading zeros
        for _ in 0..(SCALE - bcd.first_non_zero() - 1) {
            output.push(b'0');
        }

        // Value
        for i in (bcd.last_non_zero()..=bcd.first_non_zero()).rev() {
            output.push(b'0' + bcd.digit(i as usize));
        }

        return;
    }

    // Both Integer and Fractional parts are present

    // Integer part
    for i in (SCALE..=bcd.first_non_zero()).rev() {
        output.push(b'0' + bcd.digit(i as usize));
    }

    // Decimal point
    output.push(b'.');

    // Fractional part
    for i in (bcd.last_non_zero()..SCALE).rev() {
        output.push(b'0' + bcd.digit(i));
    }
}
