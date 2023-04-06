use arrform::{arrform, ArrForm};

use crate::math::{Decimal, PreciseDecimal};
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::print::tty::TTY;
use crate::sbor_decoder::SborEvent;

// Decimal parameter printer
// TODO: at present only positive values are printed properly
pub struct DecimalParameterPrinter {}

pub const DECIMAL_PARAMETER_PRINTER: DecimalParameterPrinter = DecimalParameterPrinter {};

impl ParameterPrinter for DecimalParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        match Decimal::try_from(state.data.as_slice()) {
            Ok(value) => state.print_text(
                arrform!({ Decimal::MAX_PRINT_LEN + 10 }, "Decimal({})", value).as_bytes(),
            ),
            Err(_) => state.print_text(b"Decimal(<invalid value>)"),
        }
    }
}

// PreciseDecimal parameter printer
// TODO: at present only positive values are printed properly
pub struct PreciseDecimalParameterPrinter {}

pub const PRECISE_DECIMAL_PARAMETER_PRINTER: PreciseDecimalParameterPrinter =
    PreciseDecimalParameterPrinter {};

impl ParameterPrinter for PreciseDecimalParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        match PreciseDecimal::try_from(state.data.as_slice()) {
            Ok(value) => state.print_text(
                arrform!(
                    { PreciseDecimal::MAX_PRINT_LEN + 20 },
                    "PreciseDecimal({})",
                    value
                )
                .as_bytes(),
            ),
            Err(_) => state.print_text(b"PreciseDecimal(<invalid value>)"),
        }
    }
}
