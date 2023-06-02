use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;
use crate::type_info::{to_type_name, TYPE_U8};

pub struct ArrayParameterPrinter {}

pub const ARRAY_PARAMETER_PRINTER: ArrayParameterPrinter = ArrayParameterPrinter {};

impl<T: Copy> ParameterPrinter<T> for ArrayParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState<T>, event: SborEvent) {
        let type_id = state.active_state().element_type_id;

        if let SborEvent::ElementType { .. } = event {
            if type_id != TYPE_U8 {
                state.print_text(b"Array<");
                state.print_text(to_type_name(type_id));
                state.print_text(b">(");
            } else {
                state.print_text(b"Bytes(");
            }
            return;
        }

        if type_id == TYPE_U8 {
            if let SborEvent::Data(byte) = event {
                state.print_hex_byte(byte)
            }
            return;
        }
    }

    fn subcomponent_end(&self, state: &mut ParameterPrinterState<T>) {
        state.print_text(b", ");
    }

    fn end(&self, state: &mut ParameterPrinterState<T>) {
        state.print_byte(b')');
    }
}
