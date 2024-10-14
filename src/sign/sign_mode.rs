#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum SignMode {
    TxEd25519Verbose,
    TxEd25519Summary,
    TxSecp256k1Verbose,
    TxSecp256k1Summary,
    AuthEd25519,
    AuthSecp256k1,
    PreAuthHashEd25519,
    PreAuthHashSecp256k1,
}
