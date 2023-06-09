use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

pub trait ParameterPrinter<T: Copy> {
    fn handle_data(&self, state: &mut ParameterPrinterState<T>, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }
    fn start(&self, _state: &mut ParameterPrinterState<T>) {}
    fn end(&self, _state: &mut ParameterPrinterState<T>) {}
    fn subcomponent_start(&self, _state: &mut ParameterPrinterState<T>) {}
    fn subcomponent_end(&self, _state: &mut ParameterPrinterState<T>) {}
}
