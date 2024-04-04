use sbor::math::Decimal;
use crate::sign::instruction_processor::InstructionProcessor;
use crate::xui::titled_message;

#[cfg(not(target_os = "stax"))]
pub fn display<T: Copy>(fee: &Decimal, processor: &mut InstructionProcessor<T>) {
    let text = processor.format_decimal(fee, b" XRD");

    titled_message::display(b"Max TX Fee:", text);
}

#[cfg(target_os = "stax")]
pub fn display<T: Copy>(fee: &Decimal, processor: &InstructionProcessor<T>) {}
