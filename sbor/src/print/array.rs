use staticvec::StaticVec;

use crate::display_io::DisplayIO;
use crate::print::hex::HEX_PARAMETER_PRINTER;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;
use crate::type_info::{to_kind_name, to_type_info, TYPE_U8};

// Printer for various arrays
pub struct ArrayParameterPrinter {}

pub const ARRAY_PARAMETER_PRINTER: ArrayParameterPrinter = ArrayParameterPrinter {};

impl ArrayParameterPrinter {
    const USER_INFO_SPACE_LEN: usize = 20;
    const PRINTABLE_SIZE: usize =
        ParameterPrinterState::PARAMETER_AREA_SIZE * 2 + ArrayParameterPrinter::USER_INFO_SPACE_LEN;
}

impl ParameterPrinter for ArrayParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::ElementType { kind: _, type_id } = event {
            state.discriminator = type_id;
            return;
        }

        if state.discriminator == TYPE_U8 {
            if let SborEvent::Data(byte) = event {
                state.push_byte(byte);
            }
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.discriminator == TYPE_U8 {
            HEX_PARAMETER_PRINTER.display(state, display);
            return;
        }

        let mut message = StaticVec::<u8, { ArrayParameterPrinter::PRINTABLE_SIZE }>::new();
        message.extend_from_slice(b"Array<");

        match to_type_info(state.discriminator) {
            None => message.extend_from_slice(b"(unknown)"),
            Some(info) => message.extend_from_slice(to_kind_name(info.type_kind)),
        };
        message.push(b'>');

        display.scroll(message.as_slice());
    }
}
