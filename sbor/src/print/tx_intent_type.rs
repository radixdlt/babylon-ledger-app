#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TxIntentType {
    Transfer = 0x00,
    General = 0xFF,
}

impl From<u8> for TxIntentType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => TxIntentType::Transfer,
            0xFF => TxIntentType::General,
            _ => TxIntentType::General,
        }
    }
}
