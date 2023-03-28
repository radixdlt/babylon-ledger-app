use crate::print::state::ParameterPrinterState;
use crate::print::tty::TTY;
use crate::sbor_decoder::SborEvent;

pub trait ParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent);

    fn start(&self, _state: &mut ParameterPrinterState) {}

    fn end(&self, _state: &mut ParameterPrinterState) {}
}
