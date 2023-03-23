use arrform::{arrform, ArrForm};

use crate::display_io::DisplayIO;
use crate::math::{Decimal, PreciseDecimal};
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

// Decimal parameter printer
// TODO: at present only positive values are printed properly
pub struct DecimalParameterPrinter {}

pub const DECIMAL_PARAMETER_PRINTER: DecimalParameterPrinter = DecimalParameterPrinter {};

impl DecimalParameterPrinter {
    const DECORATION_LEN: usize = b"Dec(".len();
    const MAX_DISPLAY_LEN: usize = Decimal::MAX_PRINT_LEN + Self::DECORATION_LEN;
}

impl ParameterPrinter for DecimalParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        match Decimal::try_from(state.data()) {
            Ok(value) => display.scroll(
                arrform!(
                    { DecimalParameterPrinter::MAX_DISPLAY_LEN },
                    "Dec({})",
                    value
                )
                    .as_bytes(),
            ),
            Err(_) => display.scroll(b"Dec(<invalid value>)"),
        }
    }
}

// PreciseDecimal parameter printer
// TODO: at present only positive values are printed properly
pub struct PreciseDecimalParameterPrinter {}

pub const PRECISE_DECIMAL_PARAMETER_PRINTER: PreciseDecimalParameterPrinter =
    PreciseDecimalParameterPrinter {};

impl PreciseDecimalParameterPrinter {
    const DECORATION_LEN: usize = b"PDec(".len();
    const MAX_DISPLAY_LEN: usize = PreciseDecimal::MAX_PRINT_LEN + Self::DECORATION_LEN;
}

impl ParameterPrinter for PreciseDecimalParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        match PreciseDecimal::try_from(state.data()) {
            Ok(value) => display.scroll(
                arrform!(
                    { PreciseDecimalParameterPrinter::MAX_DISPLAY_LEN },
                    "PDec({})",
                    value
                )
                    .as_bytes(),
            ),
            Err(_) => display.scroll(b"Dec(<invalid value>)"),
        }
    }
}

