// APDU Command Class for Radix Ledger Apps

#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum CommandClass {
    Regular,
    Continuation,
    LastData,
    Unknown,
}

impl From<u8> for CommandClass {
    fn from(ins: u8) -> CommandClass {
        match ins {
            0xAA => CommandClass::Regular,
            0xAB => CommandClass::Continuation,
            0xAC => CommandClass::LastData,
            _ => CommandClass::Unknown,
        }
    }
}
