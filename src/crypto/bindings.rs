pub use nanos_sdk::bindings::{
    cx_curve_t, cx_ecfp_private_key_t, cx_ecfp_public_key_t, cx_err_t, size_t, CX_CURVE_Ed25519,
    CX_CURVE_SECP256K1, CX_SHA512, HDW_ED25519_SLIP10, HDW_NORMAL,
};

extern "C" {
    pub fn cx_ecfp_generate_pair_no_throw(
        curve: cx_curve_t,
        pubkey: *mut cx_ecfp_public_key_t,
        private_key: *const cx_ecfp_private_key_t,
        keep_private: bool,
    ) -> cx_err_t;

    pub fn cx_ecfp_init_public_key_no_throw(
        curve: cx_curve_t,
        raw_key: *const u8,
        key_len: size_t,
        key: *mut cx_ecfp_public_key_t,
    ) -> cx_err_t;

    pub fn cx_ecfp_init_private_key_no_throw(
        curve: cx_curve_t,
        raw_key: *const u8,
        key_len: size_t,
        private_key: *mut cx_ecfp_private_key_t,
    ) -> cx_err_t;

    pub fn os_perso_derive_node_with_seed_key(
        mode: core::ffi::c_uint,
        curve: cx_curve_t,
        path: *const core::ffi::c_uint,
        path_length: core::ffi::c_uint,
        private_key: *mut core::ffi::c_uchar,
        chain: *mut core::ffi::c_uchar,
        seed_key: *mut core::ffi::c_uchar,
        seed_key_length: core::ffi::c_uint,
    );
}
