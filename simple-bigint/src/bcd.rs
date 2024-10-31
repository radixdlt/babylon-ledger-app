use crate::ceil_div;

/// Implementation of the simple BCD convertor/accumulator
/// Algorithm is a quite straightforward implementation of the double-dabble algorithm.
/// https://en.wikipedia.org/wiki/Double_dabble

/// N - corresponds to number of bits of the equivalent binary representation
/// Actual storage is 4/3 of the N because BCD representation is less dense than binary.
#[derive(Copy, Clone)]
pub struct BCD<const N: usize>
where
    [(); ceil_div(4 * ceil_div(N, 3), 8)]:,
{
    digits: [u8; ceil_div(4 * ceil_div(N, 3), 8)],
    total_bits: u16,
    first_non_zero: i16,
    last_non_zero: i16,
    sign: bool,
}

impl<const N: usize> BCD<N>
where
    [(); ceil_div(4 * ceil_div(N, 3), 8)]:,
{
    pub const fn new() -> Self {
        Self::with_sign(false)
    }

    pub const fn with_sign(sign: bool) -> Self {
        Self {
            digits: [0; ceil_div(4 * ceil_div(N, 3), 8)],
            sign,
            total_bits: 0,
            first_non_zero: 0,
            last_non_zero: 0,
        }
    }

    pub fn num_digits(&self) -> usize {
        if self.total_bits == 0 {
            0
        } else {
            ceil_div(self.total_bits as usize, 2) as usize + 1
        }
    }

    pub fn is_zero(&self) -> bool {
        self.first_non_zero == -1
    }

    pub fn is_negative(&self) -> bool {
        self.sign
    }

    pub fn first_non_zero(&self) -> usize {
        if self.first_non_zero < 0 {
            0
        } else {
            self.first_non_zero as usize
        }
    }

    pub fn last_non_zero(&self) -> usize {
        self.last_non_zero as usize
    }

    pub fn digit(&self, ndx: usize) -> u8 {
        if ndx > self.num_digits() {
            return 0;
        }

        let index = ndx >> 1;
        self.digits[index] >> (4 * (ndx & 1)) & 0x0F
    }

    pub fn set_sign(&mut self, sign: bool) {
        self.sign = sign;
    }

    pub fn push_bit(&mut self, bit: bool) {
        self.dabble();

        let mut carry = bit;

        self.first_non_zero = -1;
        self.last_non_zero = 0;
        let mut found_non_zero = false;

        // Double part: shift entire number left by 1 bit,
        // inserting input bit as the lowest bit of the resulting number.
        // Among doubling, starting and trailing non-zero digits are calculated.
        for i in 0..self.digits.len() {
            let digit = self.digits[i];
            self.digits[i] = (digit << 1) | carry as u8;
            carry = digit & 0x80 != 0;

            // Find first/last non-zero digit
            if self.digits[i] != 0 {
                self.first_non_zero = (i as i16) << 1;

                if !found_non_zero {
                    self.last_non_zero = self.first_non_zero;
                }

                if (self.digits[i] & 0xF0) != 0 {
                    self.first_non_zero += 1;
                }

                if (self.digits[i] & 0x0F) == 0 {
                    if !found_non_zero {
                        self.last_non_zero += 1
                    }
                }

                found_non_zero = true;
            }
        }
        self.total_bits += 1;
    }

    fn dabble(&mut self) {
        for ndx in 0..ceil_div(N, 3) {
            let i = ndx >> 1;

            let mut nibble = if ndx & 1 == 0 {
                self.digits[i] & 0x0F
            } else {
                self.digits[i] >> 4
            };

            if nibble >= 5 {
                nibble += 3;
                nibble &= 0x0F;
            }

            self.digits[i] = if ndx & 1 == 0 {
                (self.digits[i] & 0xF0) | nibble
            } else {
                (self.digits[i] & 0x0F) | (nibble << 4)
            };
        }
    }
}
