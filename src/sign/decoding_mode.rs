#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum DecodingMode {
    Auth,
    PreAuth,
    Transaction,
}
