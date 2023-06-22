use core::result::Result;

use crate::digest::digest::Digest;

pub trait Digester {
    type Error;

    fn new() -> Self;
    fn reset(&mut self);
    fn init(&mut self) -> Result<(), Self::Error>;
    fn update(&mut self, input: &[u8]) -> Result<(), Self::Error>;
    fn finalize(&mut self) -> Result<Digest, Self::Error>;
}
