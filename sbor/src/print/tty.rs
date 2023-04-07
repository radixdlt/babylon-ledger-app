#[derive(Copy, Clone)]
pub struct TTY {
    pub show_message: fn(message: &[u8]),
}
