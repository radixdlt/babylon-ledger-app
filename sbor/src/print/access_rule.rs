use crate::display_io::DisplayIO;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

// AccessRule parameter printer
pub struct AccessRuleParameterPrinter {}

pub const ACCESS_RULE_PARAMETER_PRINTER: AccessRuleParameterPrinter = AccessRuleParameterPrinter {};

impl ParameterPrinter for AccessRuleParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Discriminator(byte) = event {
            if state.data.len() > 0 {
                return;
            }

            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        let message: &[u8] = match (state.data.len(), state.data[0]) {
            (1, 0) => b"Access(AllowAll)",
            (1, 1) => b"Access(DenyAll)",
            (1, 2) => b"Access(Protected(<rules not decoded>))",
            (1, _) => b"Access(<unknown access rule>)",
            (_, _) => b"Access(<decoding failure>)",
        };

        display.scroll(message);
    }
}

