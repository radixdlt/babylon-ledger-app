use crate::sign::instruction_processor::InstructionProcessor;
use crate::xui::{fee, titled_message};
use sbor::digest::digest::Digest;
use sbor::math::Decimal;

#[cfg(not(target_os = "stax"))]
pub fn display<T: Copy>(
    digest: &Digest,
    fee: &Option<Decimal>,
    processor: &mut InstructionProcessor<T>,
) {
    titled_message::display(b"TX Hash:", &digest.as_hex());

    if let Some(fee) = fee {
        fee::display(fee, processor);
    }
}

#[cfg(target_os = "stax")]
pub fn display<T: Copy>(
    digest: &Digest,
    fee: &Option<Decimal>,
    processor: &mut InstructionProcessor<T>,
) {
}
