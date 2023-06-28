use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::type_info::*;

pub struct AddressParameterPrinter {}

pub const ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {};

impl<T: Copy> ParameterPrinter<T> for AddressParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState<T>) {
        if state.data.len() != (ADDRESS_STATIC_LEN as usize) {
            state.print_text(b"Invalid address format");
            return;
        }
        state.print_static_address();
    }
}
