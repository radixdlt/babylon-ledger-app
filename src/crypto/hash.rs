use crate::app_error::AppError;
use core::ffi::c_uint;
use core::intrinsics::write_bytes;
use core::mem::size_of;
use nanos_sdk::bindings::{cx_sha512_t, CX_OK};

const SHA256_DIGEST_SIZE: usize = 32; // 256 bits
const SHA512_DIGEST_SIZE: usize = 64; // 512 bits
const MAX_DIGEST_SIZE: usize = SHA512_DIGEST_SIZE;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum HashType {
    DoubleSHA256,
    SHA512,
}

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
            HashType::DoubleSHA256 => &self.container[..SHA256_DIGEST_SIZE],
            HashType::SHA512 => &self.container[..SHA512_DIGEST_SIZE],
        }
    }

    pub fn hash_type(&self) -> HashType {
        self.hash_type
    }
}

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
    pub fn cx_sha256_init_no_throw(context: *mut u8) -> u32;
}

extern "C" {
    pub fn cx_hash_sha256(in_: *const u8, len: c_uint, out: *mut u8, out_len: c_uint) -> c_uint;
}

extern "C" {
    pub fn cx_sha512_init_no_throw(context: *mut u8) -> u32;
}

extern "C" {
    pub fn cx_hash_update(hash: *mut u8, input: *const u8, in_len: c_uint) -> u32;
}
extern "C" {
    pub fn cx_hash_final(hash: *mut u8, digest: *mut u8) -> u32;
}

impl Hasher {
    const WORK_AREA_SIZE: usize = size_of::<cx_sha512_t>();

    fn to_result(rc: u32) -> Result<(), AppError> {
        if rc == CX_OK {
            Ok(())
        } else {
            Err(rc.into())
        }
    }

    pub fn new() -> Self {
        Self {
            work_data: [0; Self::WORK_AREA_SIZE],
            hash_type: HashType::DoubleSHA256,
        }
    }

    pub fn one_step_double_sha256(input: &[u8]) -> Result<Digest, AppError> {
        let mut hasher = Hasher::new();
        hasher.init(HashType::DoubleSHA256)?;
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

        let rc = match self.hash_type {
            HashType::DoubleSHA256 => unsafe {
                cx_sha256_init_no_throw(self.work_data.as_mut_ptr())
            },
            HashType::SHA512 => unsafe { cx_sha512_init_no_throw(self.work_data.as_mut_ptr()) },
        };

        Self::to_result(rc)
    }

    pub fn update(&mut self, input: &[u8]) -> Result<(), AppError> {
        let rc = unsafe {
            cx_hash_update(
                self.work_data.as_mut_ptr(),
                input.as_ptr(),
                input.len() as c_uint,
            )
        };

        Self::to_result(rc)
    }

    fn finalize_double_sha256(&mut self) -> Result<Digest, AppError> {
        let mut first_pass_digest = [0u8; SHA256_DIGEST_SIZE];

        let rc =
            unsafe { cx_hash_final(self.work_data.as_mut_ptr(), first_pass_digest.as_mut_ptr()) };

        if rc != CX_OK {
            return Err(rc.into());
        }

        let mut digest = Digest::new(HashType::DoubleSHA256);

        self.init(HashType::DoubleSHA256)?;
        self.update(&first_pass_digest)?;
        let rc =
            unsafe { cx_hash_final(self.work_data.as_mut_ptr(), digest.container.as_mut_ptr()) };

        self.reset();

        Self::to_result(rc).map(|_| digest)
    }

    fn finalize_sha512(&mut self) -> Result<Digest, AppError> {
        let mut digest = Digest::new(HashType::SHA512);

        let rc = unsafe { cx_hash_final(self.work_data.as_mut_ptr(), digest.as_mut()) };

        Self::to_result(rc).map(|_| digest)
    }

    pub fn finalize(&mut self) -> Result<Digest, AppError> {
        match self.hash_type {
            HashType::DoubleSHA256 => self.finalize_double_sha256(),
            HashType::SHA512 => self.finalize_sha512(),
        }
    }
}
