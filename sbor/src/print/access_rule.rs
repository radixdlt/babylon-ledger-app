use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

// AccessRule parameter printer
pub struct AccessRuleParameterPrinter {}

pub const ACCESS_RULE_PARAMETER_PRINTER: AccessRuleParameterPrinter = AccessRuleParameterPrinter {};

impl ParameterPrinter for AccessRuleParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {}

    fn end(&self, state: &mut ParameterPrinterState) {
        let text: &[u8] = match state.data[0] {
            0 => b"Access(AllowAll)",
            1 => b"Access(DenyAll)",
            2 => b"Access(Protected(<rules not decoded>))",
            _ => b"Access(<unknown access rule>)",
        };

        state.tty.print_text(text);
    }
}
