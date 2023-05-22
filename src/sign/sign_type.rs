#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum SignType {
    Ed25519,
    Ed25519Summary,
    Secp256k1,
    Secp256k1Summary,
    AuthEd25519,
    AuthSecp256k1,
}
