use staticvec::StaticVec;

use crate::display_io::DisplayIO;
use crate::print::hex::HEX_PARAMETER_PRINTER;
use crate::print::manifest_value::{get_printer_for_discriminator};
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;
use crate::type_info::{to_kind_name, to_type_info, SIMPLE_TYPES, TYPE_U8};

// Printer for various arrays
pub struct ArrayParameterPrinter {}

pub const ARRAY_PARAMETER_PRINTER: ArrayParameterPrinter = ArrayParameterPrinter {};

impl ArrayParameterPrinter {
    const PRINTABLE_SIZE: usize = 80;
}

impl ParameterPrinter for ArrayParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::ElementType { kind: _, type_id } = event {
            state.discriminator = type_id;

            if state.discriminator != TYPE_U8 {
                display.scroll(b"Array<");
            }
            return;
        }

        if state.discriminator == TYPE_U8 {
            if let SborEvent::Data(byte) = event {
                state.push_byte(byte);
            }
            return;
        }

        if SIMPLE_TYPES.contains(&state.discriminator) {
            match event {
                SborEvent::Start { type_id, .. } if type_id == state.discriminator => {
                        state.reset();
                        state.discriminator = type_id;
                }
                SborEvent::End { type_id, .. }  if type_id == state.discriminator => {
                    get_printer_for_discriminator(state.discriminator).display(state, display);
                }
                _ => {
                    get_printer_for_discriminator(state.discriminator).handle_data_event(state, event, display);
                }
            };
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.discriminator == TYPE_U8 {
            HEX_PARAMETER_PRINTER.display(state, display);
            return;
        }

        let mut message = StaticVec::<u8, { ArrayParameterPrinter::PRINTABLE_SIZE }>::new();
        if !SIMPLE_TYPES.contains(&state.discriminator) {
            match to_type_info(state.discriminator) {
                None => message.extend_from_slice(b"(unknown)"),
                Some(info) => message.extend_from_slice(to_kind_name(info.type_kind)),
            };
        }
        message.push(b'>');
        display.scroll(message.as_slice());
    }
}
