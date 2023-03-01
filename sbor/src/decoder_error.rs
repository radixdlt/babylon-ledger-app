// Errors which could appear during decoding
#[derive(Debug, Copy, Clone)]
pub enum DecoderError {
    UnknownType(usize, u8),          // Unknown main type id
    UnknownSubType(usize, u8),       // Unknown sub type id (element, key, value)
    UnknownDiscriminator(usize, u8), // Unknown discriminator for OWN or NFL ID
    InvalidLen(usize, u8),           // Incorrectly encoded element length
    InvalidState(usize),             // Input caused decoder to reach invalid state
    StackOverflow(usize),            // Decoding stack overflow
    StackUnderflow(usize),           // Decoding stack underflow
}
