/// Errors which could appear during decoding.
/// First parameter is the position in the stream where error was encountered.
/// Second parameter (where available) is the byte which caused the error.
#[derive(Copy, Clone, Debug)]
pub enum DecoderError {
    UnknownType(usize, u8),          // Unknown main type id
    UnknownSubType(usize, u8),       // Unknown subtype id (element, key, value)
    UnknownDiscriminator(usize, u8), // Unknown discriminator for OWN or NFL ID
    InvalidLen(usize, u8),           // Incorrectly encoded element length
    InvalidState(usize),             // Input caused decoder to reach invalid state
    StackOverflow(usize),            // Decoding stack overflow
    StackUnderflow(usize),           // Decoding stack underflow
}
