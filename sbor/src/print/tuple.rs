use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

pub struct TupleParameterPrinter {}

pub const TUPLE_PARAMETER_PRINTER: TupleParameterPrinter = TupleParameterPrinter {};

impl ParameterPrinter for TupleParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {}

    fn start(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(b"Tuple(");
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(b") ");
    }

    fn subcomponent_end(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(b", ");
    }
}
