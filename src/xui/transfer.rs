use crate::sign::instruction_processor::InstructionProcessor;
use crate::xui::{fee, titled_message};
use sbor::print::tx_summary_detector::TransferDetails;

#[cfg(not(target_os = "stax"))]
pub fn display<T: Copy>(details: &TransferDetails, processor: &mut InstructionProcessor<T>) {
    titled_message::display(b"TX Type:", b"Transfer");
    titled_message::display(b"From:", processor.format_address(&details.src_address));
    titled_message::display(b"To:", processor.format_address(&details.dst_address));

    if details.res_address.is_xrd() {
        titled_message::display(
            b"Amount:",
            processor.format_decimal(&details.amount, b" XRD"),
        );
    } else {
        titled_message::display(b"Resource:", processor.format_address(&details.res_address));
        titled_message::display(b"Amount:", processor.format_decimal(&details.amount, b""));
    }

    if let Some(fee) = details.fee {
        fee::display(&fee, processor);
    }
}

#[cfg(target_os = "stax")]
pub fn display<T: Copy>(details: &TransferDetails, processor: &mut InstructionProcessor<T>) {}
