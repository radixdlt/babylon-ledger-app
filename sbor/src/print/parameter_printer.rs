use crate::display_io::DisplayIO;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

pub trait ParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        display: &'static dyn DisplayIO,
    );
    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO); // TODO: can we break flow in the mid of instruction?

    fn is_value_printer(&self) -> bool {
        false
    }
}
