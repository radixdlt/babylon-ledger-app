use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;
use crate::type_info::*;

pub struct AddressParameterPrinter {}

pub const ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {};

impl<T: Copy> ParameterPrinter<T> for AddressParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState<T>, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
        if let SborEvent::Discriminator(byte) = event {
            state.push_byte(byte);
        }
    }

    fn end(&self, state: &mut ParameterPrinterState<T>) {
        match state.data.as_slice()[0] {
            0 => {
                if state.data.len() != (ADDRESS_STATIC_LEN as usize) + 1 {
                    state.print_text(b"Invalid address format");
                    return;
                }
                state.print_static_address();
            }
            1 => {
                if state.data.len() != (ADDRESS_NAMED_LEN as usize) + 1 {
                    state.print_text(b"Invalid address format");
                    return;
                }
                state.print_named_address();
            }
            _ => {
                state.print_text(b"Invalid address format");
            }
        }
    }
}
