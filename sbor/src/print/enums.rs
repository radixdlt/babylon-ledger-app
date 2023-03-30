use arrform::{arrform, ArrForm};

use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

pub struct EnumParameterPrinter {}

pub const ENUM_PARAMETER_PRINTER: EnumParameterPrinter = EnumParameterPrinter {};

impl ParameterPrinter for EnumParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Discriminator(discriminator) = event {
            state
                .tty
                .print_text(arrform!(8, "{}u8", discriminator).as_bytes());
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(b"Enum(");
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        state.tty.print_byte(b')');
    }

    fn subcomponent_start(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(b", ");
    }
}
