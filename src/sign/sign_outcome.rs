#[derive(Copy, Clone)]
#[repr(u8)]
pub enum SignOutcome {
    SendNextPacket,
    SigningRejected,
    SignatureEd25519,
    SignatureSecp256k1,
}
