#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum SignMode {
    Ed25519Verbose,
    Ed25519Summary,
    Secp256k1Verbose,
    Secp256k1Summary,
    AuthEd25519,
    AuthSecp256k1,
    Ed25519PreAuthHash,
    Ed25519Subintent,
}
