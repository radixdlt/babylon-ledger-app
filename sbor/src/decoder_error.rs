// Errors which could appear during decoding
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderError {
    InvalidInput(usize, u8), // Unexpected byte received
    InvalidLen(usize, u8),   // Incorrectly encoded element length
    InvalidState(usize),     //
    StackOverflow(usize),
    StackUnderflow(usize),
}
