use core::intrinsics::write_bytes;
use nanos_sdk::bindings::{cx_hash_sha256, size_t};

#[derive(Default)]
pub struct Sha256 {
    buffer: [u8; 32],
}

impl Drop for Sha256 {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}

impl Sha256 {
    fn new() -> Self {
        Self { buffer: [0u8; 32] }
    }

    pub fn single(input: &[u8]) -> Sha256 {
        unsafe {
            let mut sha256 = Sha256::new();

            cx_hash_sha256(
                input.as_ptr(),
                input.len() as size_t,
                &mut sha256.buffer as *mut u8,
                sha256.buffer.len() as size_t,
            );
            sha256
        }
    }

    pub fn double(input: &[u8]) -> Sha256 {
        let step1 = Sha256::single(input);

        Sha256::single(&step1.buffer)
    }

    pub fn hash(&self) -> &[u8] {
        &self.buffer
    }
}
