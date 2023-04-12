use crate::print::parameter_printer::ParameterPrinter;
use crate::print::primitives::U32_PARAMETER_PRINTER;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

pub struct BlobParameterPrinter;
pub struct ExpressionParameterPrinter;
pub struct BucketParameterPrinter;
pub struct ProofParameterPrinter;

pub const BLOB_PARAMETER_PRINTER: BlobParameterPrinter = BlobParameterPrinter {};
pub const EXPRESSION_PARAMETER_PRINTER: ExpressionParameterPrinter = ExpressionParameterPrinter {};
pub const BUCKET_PARAMETER_PRINTER: BucketParameterPrinter = BucketParameterPrinter {};
pub const PROOF_PARAMETER_PRINTER: ProofParameterPrinter = ProofParameterPrinter {};

impl ParameterPrinter for BlobParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.print_hex_byte(byte);
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.print_text(b"Blob(");
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        state.print_byte(b')');
    }
}

impl ParameterPrinter for ExpressionParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.print_hex_byte(byte);
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.print_text(b"Expression(");
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        state.print_byte(b')');
    }
}

impl ParameterPrinter for BucketParameterPrinter {
    fn start(&self, state: &mut ParameterPrinterState) {
        state.print_text(b"Bucket(");
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        U32_PARAMETER_PRINTER.end(state);
        state.print_byte(b')');
    }
}

impl ParameterPrinter for ProofParameterPrinter {
    fn start(&self, state: &mut ParameterPrinterState) {
        state.print_text(b"Proof(");
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        U32_PARAMETER_PRINTER.end(state);
        state.print_byte(b')');
    }
}
