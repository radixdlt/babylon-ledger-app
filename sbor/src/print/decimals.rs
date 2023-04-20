use crate::math::{Decimal, PreciseDecimal};
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;

// Decimal parameter printer
// TODO: at present only positive values are printed properly
pub struct DecimalParameterPrinter {}

pub const DECIMAL_PARAMETER_PRINTER: DecimalParameterPrinter = DecimalParameterPrinter {};

impl ParameterPrinter for DecimalParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState) {
        match Decimal::try_from(state.data.as_slice()) {
            Ok(value) => {
                state.data.clear();
                value.format(&mut state.data);
                state.print_text(b"Decimal(");
                state.print_data_as_text();
                state.print_text(b")");
            }
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
    #[cfg(target_os = "nanos")]
    fn end(&self, state: &mut ParameterPrinterState) {
        match PreciseDecimal::try_from(state.data.as_slice()) {
            Ok(value) => {
                state.print_text(b"PreciseDecimal(<not decoded>");
            }
            Err(_) => state.print_text(b"PreciseDecimal(<invalid value>)"),
        }
    }

    #[cfg(not(target_os = "nanos"))]
    fn end(&self, state: &mut ParameterPrinterState) {
        match PreciseDecimal::try_from(state.data.as_slice()) {
            Ok(value) => {
                state.data.clear();
                value.format(&mut state.data);
                state.print_text(b"PreciseDecimal(");
                state.print_data_as_text();
                state.print_text(b")")
            }
            Err(_) => state.print_text(b"PreciseDecimal(<invalid value>)"),
        }
    }
}
