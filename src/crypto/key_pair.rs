use crate::crypto::bindings::{cx_ecfp_private_key_t, cx_ecfp_public_key_t};
use core::intrinsics::write_bytes;

pub struct KeyPair {
    pub public: cx_ecfp_public_key_t,
    pub private: cx_ecfp_private_key_t,
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}
