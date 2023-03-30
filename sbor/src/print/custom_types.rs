use arrform::{arrform, ArrForm};

use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

pub struct HexParameterPrinter {
    name: &'static [u8],
}

pub struct UintParameterPrinter {
    name: &'static [u8],
}

pub const BLOB_PARAMETER_PRINTER: HexParameterPrinter = HexParameterPrinter { name: b"Blob" };
pub const EXPRESSION_PARAMETER_PRINTER: HexParameterPrinter = HexParameterPrinter { name: b"Expression" };
pub const BUCKET_PARAMETER_PRINTER: UintParameterPrinter = UintParameterPrinter { name: b"Bucket" };
pub const PROOF_PARAMETER_PRINTER: UintParameterPrinter = UintParameterPrinter { name: b"Proof" };

impl ParameterPrinter for HexParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.tty.print_hex_byte(byte);
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(self.name);
        state.tty.print_byte(b'(');
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        state.tty.print_byte(b')');
    }
}

impl ParameterPrinter for UintParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(self.name);
        state.tty.print_byte(b'(');
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        if state.data.len() != 4 {
            state.tty.print_text(b"<Invalid encoding>");
            return;
        }

        fn to_array(input: &[u8]) -> [u8; 4] {
            input.try_into().expect("<should not happen>")
        }

        let value = u32::from_le_bytes(to_array(state.data.as_slice()));

        state.tty.print_text(arrform!(20, "{})", value).as_bytes());
    }
}
