use sbor::digest::digest::Digest;

use crate::crypto::ed25519::{ED25519_PUBLIC_KEY_LEN, ED25519_SIGNATURE_LEN};
use crate::crypto::secp256k1::{SECP256K1_PUBLIC_KEY_LEN, SECP256K1_SIGNATURE_LEN};
use crate::utilities::max::max;

pub const MAX_SIGNATURE_SIZE: usize = max(ED25519_SIGNATURE_LEN, SECP256K1_SIGNATURE_LEN);
pub const MAX_PUBKEY_SIZE: usize = max(ED25519_PUBLIC_KEY_LEN, SECP256K1_PUBLIC_KEY_LEN);
pub const MAX_SIGNATURE_PAYLOAD: usize = MAX_SIGNATURE_SIZE + MAX_PUBKEY_SIZE;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SignOutcome {
    SendNextPacket,
    SigningRejected,
    SignatureEd25519 {
        signature: [u8; ED25519_SIGNATURE_LEN],
        key: [u8; ED25519_PUBLIC_KEY_LEN],
        digest: [u8; Digest::DIGEST_LENGTH],
    },
    SignatureSecp256k1 {
        signature: [u8; SECP256K1_SIGNATURE_LEN],
        key: [u8; SECP256K1_PUBLIC_KEY_LEN],
        digest: [u8; Digest::DIGEST_LENGTH],
    },
}
