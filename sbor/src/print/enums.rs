use crate::print::parameter_printer::ParameterPrinter;
use crate::print::primitives::U8ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::print::tty::TTY;
use crate::sbor_decoder::SborEvent;

pub struct EnumParameterPrinter {}

pub const ENUM_PARAMETER_PRINTER: EnumParameterPrinter = EnumParameterPrinter {};

impl ParameterPrinter for EnumParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Discriminator(discriminator) = event {
            U8ParameterPrinter::print(state, discriminator);
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.print_text(b"Enum(");
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        state.print_byte(b')');
    }

    fn subcomponent_start(&self, state: &mut ParameterPrinterState) {
        state.print_text(b", ");
    }
}
