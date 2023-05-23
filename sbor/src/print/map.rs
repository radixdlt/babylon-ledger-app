use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::{SborEvent, SubTypeKind};
use crate::type_info::to_type_name;

pub struct MapParameterPrinter {}

pub const MAP_PARAMETER_PRINTER: MapParameterPrinter = MapParameterPrinter {};

impl<T> ParameterPrinter<T> for MapParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState<T>, event: SborEvent) {
        if let SborEvent::ElementType { kind, type_id } = event {
            match kind {
                SubTypeKind::Key => {
                    state.print_text(to_type_name(type_id));
                    state.print_text(b", ");
                }
                SubTypeKind::Value => {
                    state.print_text(to_type_name(type_id));
                    state.print_text(b">(");
                }
                _ => {}
            }
        }
    }

    fn start(&self, state: &mut ParameterPrinterState<T>) {
        state.print_text(b"Map<");
    }

    fn end(&self, state: &mut ParameterPrinterState<T>) {
        state.print_text(b")");
    }

    fn subcomponent_start(&self, state: &mut ParameterPrinterState<T>) {
        state.active_state().flip_flop = !state.active_state().flip_flop;

        if state.active_state().flip_flop {
            state.print_byte(b'{');
        }
    }

    fn subcomponent_end(&self, state: &mut ParameterPrinterState<T>) {
        if !state.active_state().flip_flop {
            state.print_text(b"}, ");
        } else {
            state.print_text(b", ");
        }
    }
}
