#[derive(Copy, Clone)]
pub struct TTY<T> {
    pub data: T,
    pub show_message: fn(&mut T, title: &[u8], message: &[u8]),
}
