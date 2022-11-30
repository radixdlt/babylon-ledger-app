#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SborEvent {
    Start { type_id: u8, nesting_level: u8, fixed_size: u8 },
    Len(u32),
    NameLen(u32),
    Name(u8),
    Data(u8),
    End { type_id: u8, nesting_level: u8 },
}
