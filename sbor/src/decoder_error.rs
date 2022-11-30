#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderError {
    InvalidInput(usize, u8),
    InvalidLen(usize, u8),
    InvalidState(usize),
    StackOverflow(usize),
    StackUnderflow(usize),
}