// Events generated by SBOR decoder

#[derive(Debug, Clone, Copy)]
pub enum SborEvent {
    Start {
        type_id: u8,
        nesting_level: u8,
        fixed_size: u8,
    },
    Len(u32),
    Discriminator(u8),
    Data(u8),
    End {
        type_id: u8,
        nesting_level: u8,
    },
}
