use staticvec::StaticVec;

use crate::print::tty::TTY;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::{ParameterPrinterState, PARAMETER_AREA_SIZE};
use crate::sbor_decoder::SborEvent;
use crate::type_info::{to_kind_name, to_type_info};

// Printer for various arrays
pub struct TupleParameterPrinter {}

pub const TUPLE_PARAMETER_PRINTER: TupleParameterPrinter = TupleParameterPrinter {};

impl TupleParameterPrinter {
    const PRINTABLE_SIZE: usize = PARAMETER_AREA_SIZE * 2;
}

impl ParameterPrinter for TupleParameterPrinter {
    fn handle_data(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent
    ) {
        // if state.flip_flop == false {
        //     // Extract first Start - it comes for the tuple itself
        //     if let SborEvent::Start { .. } = event {
        //         state.phase = state.nesting_level + 1;
        //     }
        //     state.flip_flop = true;
        //     return;
        // }
        //
        // if state.nesting_level == state.phase {
        //     if let SborEvent::Start { type_id, .. } = event {
        //         state.push_byte(type_id);
        //         return;
        //     }
        // }
    }
}

impl TupleParameterPrinter {
    pub fn tty(&self, state: &mut ParameterPrinterState) {
        let mut message = StaticVec::<u8, { TupleParameterPrinter::PRINTABLE_SIZE }>::new();
        message.extend_from_slice(b"Tuple<");

        for &type_id in state.data.as_slice() {
            match to_type_info(type_id) {
                None => message.extend_from_slice(b"(unknown)"),
                Some(info) => message.extend_from_slice(to_kind_name(info.type_kind)),
            };
            message.extend_from_slice(b", ");
        }

        if state.data.len() > 0 {
            // Strip last comma
            message.pop();
            message.pop();
        }

        message.push(b'>');

        state.tty.print_text(message.as_slice());
    }
}
