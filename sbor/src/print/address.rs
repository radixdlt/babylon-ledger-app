use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::type_info::*;

pub struct AddressParameterPrinter {}

pub const ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {};

impl<T: Copy> ParameterPrinter<T> for AddressParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState<T>) {
        match state.data.as_slice()[0] {
            0 => {
                state.print_static_address();
            }
            1 => {
                state.print_named_address();
            }
            _ => {
                state.print_text(b"Invalid address format");
            }
        }

        if state.data.len() != (ADDRESS_STATIC_LEN as usize) {
            state.print_text(b"Invalid address format");
            return;
        }

        state.print_named_address();
    }
}
