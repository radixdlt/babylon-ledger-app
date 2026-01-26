use crate::bcd::BCD;
use crate::ceil_div;

/// Simple big integer implementation which supports very limited number of operations
/// necessary for the Babylon Ledger App.
/// N - number of bits in the big integer
#[derive(Clone, Copy, Debug)]
pub struct BigInt<const N: usize>
where
    [(); ceil_div(N, u32::BITS as usize)]:,
{
    limbs: [u32; ceil_div(N, u32::BITS as usize)],
}

impl<const N: usize> BigInt<N>
where
    [(); ceil_div(N, u32::BITS as usize)]:,
{
    pub const NUM_BITS: usize = N;
    pub const NUM_BYTES: usize = ceil_div(N, u8::BITS as usize);
    const NUM_LIMBS: usize = ceil_div(N, u32::BITS as usize);

    pub const fn new() -> Self {
        Self {
            limbs: [0; ceil_div(N, u32::BITS as usize)],
        }
    }

    pub const fn from_limbs(limbs: [u32; ceil_div(N, u32::BITS as usize)]) -> Self {
        Self { limbs }
    }

    pub fn is_negative(&self) -> bool {
        self.limbs[self.limbs.len() - 1] & 0x80000000 != 0
    }

    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.limbs == other.limbs
    }

    pub fn as_bcd(&self) -> BCD<N>
    where
        [(); ceil_div(4 * ceil_div(N, 3), 8)]:,
    {
        let mut bcd = BCD::<N>::with_sign(self.is_negative());

        if self.is_positive() {
            // Simple case, push limbs one by one, starting from the most significant
            Self::push_limbs(&mut bcd, &self.limbs);
        } else {
            let mut limbs = self.limbs.clone();
            Self::twos_complement(&mut limbs);
            Self::push_limbs(&mut bcd, &limbs);
        }
        bcd
    }

    fn push_limbs(bcd: &mut BCD<N>, limbs: &[u32])
    where
        [(); ceil_div(4 * ceil_div(N, 3), 8)]:,
    {
        for &limb in limbs.iter().rev() {
            let mut mask = 0x80_00_00_00;

            for _ in 0..32 {
                let bit = limb & mask != 0;
                bcd.push_bit(bit);
                mask >>= 1;
            }
        }
    }

    fn twos_complement(limbs: &mut [u32]) {
        let mut carry = true; // equivalent of +1 to least significant bit

        for i in 0..limbs.len() {
            // invert all bits
            limbs[i] = !limbs[i];

            // add 1
            if carry {
                if limbs[i] == 0xFFFFFFFF {
                    limbs[i] = 0;
                    carry = true;
                } else {
                    limbs[i] += 1;
                    carry = false; // stop propagatig carry
                }
            }
        }
    }

    pub fn accumulate(&mut self, other: &Self) {
        let mut carry = false;

        for i in 0..self.limbs.len() {
            let (sum, carry1) = self.limbs[i].carrying_add(other.limbs[i], carry);
            carry = carry1;
            self.limbs[i] = sum;
        }
    }

    pub fn from_bytes(value: &[u8]) -> Result<Self, BigIntError> {
        if value.len() < Self::NUM_BYTES {
            return Err(BigIntError::TooShortInput);
        }

        if value.len() > Self::NUM_BYTES {
            return Err(BigIntError::TooLongInput);
        }

        let mut limb_array = [0u8; 4];
        let mut limbs = [0u32; ceil_div(N, u32::BITS as usize)];

        for i in 0..Self::NUM_LIMBS {
            for j in 0..4usize {
                limb_array[j] = value[i * 4 + j];
            }
            limbs[i] = u32::from_le_bytes(limb_array);
        }

        Ok(BigInt::from_limbs(limbs))
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum BigIntError {
    TooLongInput,
    TooShortInput,
}

impl From<u128> for BigInt<192> {
    fn from(value: u128) -> Self {
        Self {
            limbs: [
                (value & 0xFFFFFFFF) as u32,
                ((value >> 32) & 0xFFFFFFFF) as u32,
                ((value >> 64) & 0xFFFFFFFF) as u32,
                (value >> 96) as u32,
                0,
                0,
            ],
        }
    }
}

impl From<u128> for BigInt<256> {
    fn from(value: u128) -> Self {
        Self {
            limbs: [
                (value & 0xFFFFFFFF) as u32,
                ((value >> 32) & 0xFFFFFFFF) as u32,
                ((value >> 64) & 0xFFFFFFFF) as u32,
                (value >> 96) as u32,
                0,
                0,
                0,
                0,
            ],
        }
    }
}
