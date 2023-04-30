use core::cmp::max;
use core::ffi::c_uint;
use core::intrinsics::write_bytes;
use core::mem::size_of;

use nanos_sdk::bindings::{cx_blake2b_t, cx_md_t, cx_sha512_t, size_t, CX_BLAKE2B, CX_SHA512};

use crate::app_error::{to_result, AppError};

const SHA512_DIGEST_SIZE: usize = 64; // 512 bits
const BLAKE2B_DIGEST_SIZE: usize = 32; // 256 bits

const MAX_DIGEST_SIZE: usize = max(BLAKE2B_DIGEST_SIZE, SHA512_DIGEST_SIZE);

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum HashType {
    SHA512,
    Blake2b,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Digest {
    container: [u8; MAX_DIGEST_SIZE],
    hash_type: HashType,
}

impl Digest {
    pub fn new(hash_type: HashType) -> Self {
        Self {
            container: [0; MAX_DIGEST_SIZE],
            hash_type,
        }
    }

    fn as_mut(&mut self) -> *mut u8 {
        self.container.as_mut_ptr()
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self.hash_type {
            HashType::SHA512 => &self.container[..SHA512_DIGEST_SIZE],
            HashType::Blake2b => &self.container[..BLAKE2B_DIGEST_SIZE],
        }
    }

    pub fn hash_type(&self) -> HashType {
        self.hash_type
    }
}

#[repr(C, align(4))]
pub struct Hasher {
    work_data: [u8; Self::WORK_AREA_SIZE],
    hash_type: HashType,
}

impl Drop for Hasher {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

extern "C" {
    pub fn cx_hash_init_ex(context: *mut u8, hash_type: cx_md_t, output_size: size_t) -> u32;
}

extern "C" {
    pub fn cx_hash_update(hash: *mut u8, input: *const u8, in_len: c_uint) -> u32;
}
extern "C" {
    pub fn cx_hash_final(hash: *mut u8, digest: *mut u8) -> u32;
}

impl Hasher {
    const WORK_AREA_SIZE: usize = max(size_of::<cx_sha512_t>(), size_of::<cx_blake2b_t>());

    pub const fn new() -> Self {
        Self {
            work_data: [0; Self::WORK_AREA_SIZE],
            hash_type: HashType::Blake2b,
        }
    }

    pub fn one_step(input: &[u8], hash_type: HashType) -> Result<Digest, AppError> {
        let mut hasher = Hasher::new();
        hasher.init(hash_type)?;
        hasher.update(input)?;
        hasher.finalize()
    }

    pub fn reset(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }

    pub fn init(&mut self, hash_type: HashType) -> Result<(), AppError> {
        self.reset();
        self.hash_type = hash_type;

        let hash_id = match hash_type {
            HashType::SHA512 => CX_SHA512,
            HashType::Blake2b => CX_BLAKE2B,
        };

        let output_size: size_t = match hash_type {
            HashType::SHA512 => SHA512_DIGEST_SIZE as size_t,
            HashType::Blake2b => BLAKE2B_DIGEST_SIZE as size_t,
        };
        let rc = unsafe { cx_hash_init_ex(self.work_data.as_mut_ptr(), hash_id, output_size) };
        to_result(rc)
    }

    pub fn update(&mut self, input: &[u8]) -> Result<(), AppError> {
        let rc = unsafe {
            cx_hash_update(
                self.work_data.as_mut_ptr(),
                input.as_ptr(),
                input.len() as c_uint,
            )
        };
        to_result(rc)
    }

    pub fn finalize(&mut self) -> Result<Digest, AppError> {
        let mut digest = Digest::new(self.hash_type);
        let rc = unsafe { cx_hash_final(self.work_data.as_mut_ptr(), digest.as_mut()) };
        to_result(rc).map(|_| digest)
    }
}
