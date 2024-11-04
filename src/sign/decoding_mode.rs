/// Decoding modes for the signing flow
/// There are two types of the decoding modes - "short" (they contain only 2 steps) and "long", which may contain
/// 2 or more steps, depending on the blob size.
/// "Short" types are `Auth` and `PreAuth`, which assume that host will send (1) derivation path and (2) relevant data.
/// "Long" type is `Transaction`, which processes (1) derivation path and then one or more parts of the blob (SBOR-encoded transaction or subintent).
/// It is assumed that each new 2-step commands will require adding dedicated decoding mode, although multistep processing most likely will go through
/// the same SBOR decoding process and reuse the same `Transaction` decoding mode.
#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum DecodingMode {
    Auth,
    PreAuth,
    Transaction,
}
