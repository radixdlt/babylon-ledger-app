use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::{SborEvent, SubTypeKind};
use crate::type_info::to_type_name;

pub struct MapParameterPrinter {}

pub const MAP_PARAMETER_PRINTER: MapParameterPrinter = MapParameterPrinter {};

impl ParameterPrinter for MapParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::ElementType { kind, type_id } = event {
            match kind {
                SubTypeKind::Key => {
                    state.tty.print_text(to_type_name(type_id));
                    state.tty.print_text(b", ");
                }
                SubTypeKind::Value => {
                    state.tty.print_text(to_type_name(type_id));
                    state.tty.print_text(b">(");
                }
                _ => {}
            }
        }
    }

    fn start(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(b"Map<");
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(b") ");
    }

    fn subcomponent_end(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text(b", ");
    }
}
