#![feature(generic_const_exprs)]
#![feature(bigint_helper_methods)]
#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]

pub mod bcd;
pub mod bigint;

pub const fn ceil_div(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

#[cfg(test)]
mod tests {
    use crate::bigint::BigInt;

    use super::*;

    fn to_string<const N: usize>(digits: &BigInt<N>) -> String
    where
        [(); ceil_div(N, u32::BITS as usize)]:,
        [(); ceil_div(4 * ceil_div(N, 3), 8)]:,
    {
        let mut s = String::new();
        let bcd = digits.as_bcd();

        let num_digits = bcd.first_non_zero() + 1;

        let index = if bcd.is_negative() {
            s.push('-');
            1usize
        } else {
            0
        };

        for i in 0..num_digits {
            s.insert(index, (bcd.digit(i) + b'0') as char)
        }
        s
    }

    #[test]
    fn test_small_values() {
        assert_eq!(to_string(&BigInt::<32>::from_limbs([0x0000FEDC])), "65244");
        assert_eq!(to_string(&BigInt::<32>::from_limbs([0x00012345])), "74565");
        assert_eq!(
            to_string(&BigInt::<32>::from_limbs([0x00123456])),
            "1193046"
        );
        assert_eq!(
            to_string(&BigInt::<32>::from_limbs([0x01234567])),
            "19088743"
        );
        assert_eq!(
            to_string(&BigInt::<32>::from_limbs([0x12345678])),
            "305419896"
        );
    }

    #[test]
    fn test_bigger_values() {
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x0000FEDC, 0x00000000])),
            "65244"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x00012345, 0x00000000])),
            "74565"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x00123456, 0x00000000])),
            "1193046"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x01234567, 0x00000000])),
            "19088743"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x12345678, 0x00000000])),
            "305419896"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x23456789, 0x00000001])),
            "4886718345"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x3456789A, 0x00000012])),
            "78187493530"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x456789AB, 0x00000123])),
            "1250999896491"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x56789ABC, 0x00001234])),
            "20015998343868"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x6789ABCD, 0x00012345])),
            "320255973501901"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x789ABCDE, 0x00123456])),
            "5124095576030430"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x89ABCDEF, 0x01234567])),
            "81985529216486895"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0x9ABCDEF0, 0x12345678])),
            "1311768467463790320"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0xFFFFFFFF, 0x7FFFFFFF])),
            "9223372036854775807"
        );
        assert_eq!(
            to_string(&BigInt::<64>::from_limbs([0xFFFFFFFF, 0xFFFFFFFF])),
            "-1"
        );
    }

    #[test]
    pub fn test_large_values() {
        // 000064a7b3b6e00d000000000000000000000000000000000000000000000000 - 1.0
        let big_int256 = BigInt::<256>::from_bytes(&[
            0x00, 0x00, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ])
        .unwrap();
        assert_eq!(to_string(&big_int256), "1000000000000000000");

        // 0000000000000000023ed47ec9da71dcda2f4fb5e9f37fd2079e3000000000000000000000000000000000000000000000000000000000000000000000000000 - 2.0
        let big_int512 = BigInt::<512>::from_bytes(&[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x3e, 0xd4, 0x7e, 0xc9, 0xda,
            0x71, 0xdc, 0xda, 0x2f, 0x4f, 0xb5, 0xe9, 0xf3, 0x7f, 0xd2, 0x07, 0x9e, 0x30, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ])
        .unwrap();
        assert_eq!(
            to_string(&big_int512),
            "20000000000000000000000000000000000000000000000000000000000000000"
        );
    }
}
