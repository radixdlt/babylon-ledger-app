use core::ffi::c_uint;
use core::intrinsics::write_bytes;
use core::mem::size_of;

use nanos_sdk::bindings::{cx_blake2b_t, cx_md_t, size_t, CX_BLAKE2B};
use sbor::digest::digest::{Digest, BLAKE2B_DIGEST_SIZE};
use sbor::digest::digester::Digester;

use crate::app_error::{to_result, AppError};

#[repr(C, align(4))]
pub struct Blake2bHasher([u8; Self::WORK_AREA_SIZE]);

impl Drop for Blake2bHasher {
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

impl Digester for Blake2bHasher {
    type Error = AppError;

    fn new() -> Self {
        Self::new()
    }

    fn reset(&mut self) {
        Self::reset(self)
    }

    fn init(&mut self) -> Result<(), Self::Error> {
        Self::init(self)
    }

    fn update(&mut self, input: &[u8]) -> Result<(), Self::Error> {
        Self::update(self, input)
    }

    fn finalize(&mut self) -> Result<Digest, Self::Error> {
        Self::finalize(self)
    }
}

impl Blake2bHasher {
    const WORK_AREA_SIZE: usize = size_of::<cx_blake2b_t>();

    pub const fn new() -> Self {
        Self([0; Self::WORK_AREA_SIZE])
    }

    pub fn one_step(input: &[u8]) -> Result<Digest, AppError> {
        let mut hasher = Blake2bHasher::new();
        hasher.init()?;
        hasher.update(input)?;
        hasher.finalize()
    }

    pub fn for_auth(
        &mut self,
        nonce: &[u8],
        address: &[u8],
        origin: &[u8],
    ) -> Result<Digest, AppError> {
        self.init()?;
        self.update(nonce)?;
        self.update(address)?;
        self.update(origin)?;
        self.finalize()
    }

    pub fn reset(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }

    pub fn init(&mut self) -> Result<(), AppError> {
        self.reset();

        let rc = unsafe {
            cx_hash_init_ex(
                self.0.as_mut_ptr(),
                CX_BLAKE2B,
                BLAKE2B_DIGEST_SIZE as size_t,
            )
        };

        to_result(rc)
    }

    pub fn update(&mut self, input: &[u8]) -> Result<(), AppError> {
        let rc =
            unsafe { cx_hash_update(self.0.as_mut_ptr(), input.as_ptr(), input.len() as c_uint) };
        to_result(rc)
    }

    pub fn finalize(&mut self) -> Result<Digest, AppError> {
        let mut digest = Digest::new();
        let rc = unsafe { cx_hash_final(self.0.as_mut_ptr(), digest.as_mut()) };
        to_result(rc).map(|_| digest)
    }
}
