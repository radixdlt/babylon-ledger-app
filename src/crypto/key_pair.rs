use core::intrinsics::write_bytes;

use ledger_secure_sdk_sys::{cx_ecfp_private_key_t, cx_ecfp_public_key_t};

#[derive(Clone)]
pub struct InternalKeyPair {
    pub public: cx_ecfp_public_key_t,
    pub private: cx_ecfp_private_key_t,
}

impl Drop for InternalKeyPair {
    fn drop(&mut self) {
        unsafe {
            write_bytes(self, 0, 1);
        }
    }
}
