#[derive(Copy, Clone)]
pub struct TTY {
    pub show_message: fn(title: &[u8], message: &[u8]),
}
