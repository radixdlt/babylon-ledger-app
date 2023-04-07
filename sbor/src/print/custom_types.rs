use crate::print::parameter_printer::ParameterPrinter;
use crate::print::primitives::U32_PARAMETER_PRINTER;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

pub struct HexParameterPrinter {
    name: &'static [u8],
}

pub struct UintParameterPrinter {
    name: &'static [u8],
}

pub const BLOB_PARAMETER_PRINTER: HexParameterPrinter = HexParameterPrinter { name: b"Blob" };
pub const EXPRESSION_PARAMETER_PRINTER: HexParameterPrinter = HexParameterPrinter {
    name: b"Expression",
};
pub const BUCKET_PARAMETER_PRINTER: UintParameterPrinter = UintParameterPrinter { name: b"Bucket" };
pub const PROOF_PARAMETER_PRINTER: UintParameterPrinter = UintParameterPrinter { name: b"Proof" };

impl ParameterPrinter for HexParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.print_hex_byte(byte);
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.print_text(self.name);
        state.print_byte(b'(');
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        state.print_byte(b')');
    }
}

impl ParameterPrinter for UintParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.print_text(self.name);
        state.print_byte(b'(');
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        U32_PARAMETER_PRINTER.end(state);
        state.print_byte(b')');
    }
}
