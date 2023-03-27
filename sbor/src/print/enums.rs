use arrform::{arrform, ArrForm};
use staticvec::StaticVec;

use crate::display_io::DisplayIO;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;
use crate::type_info::{to_kind_name, to_type_info};

// Printer for various arrays
pub struct EnumParameterPrinter {}

pub const ENUM_PARAMETER_PRINTER: EnumParameterPrinter = EnumParameterPrinter {};

impl EnumParameterPrinter {
    const USER_INFO_SPACE_LEN: usize = 20;
    const PRINTABLE_SIZE: usize =
        ParameterPrinterState::PARAMETER_AREA_SIZE * 2 + EnumParameterPrinter::USER_INFO_SPACE_LEN;
}

impl ParameterPrinter for EnumParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if state.nesting_level == 5 {
            // Extract first Discriminator
            if let SborEvent::Discriminator(discriminator) = event {
                state.discriminator = discriminator;
            }
            state.flip_flop = true;
            return;
        }

        if state.nesting_level == 6 {
            if let SborEvent::Start { type_id, .. } = event {
                state.push_byte(type_id);
            }
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        let mut message = StaticVec::<u8, { EnumParameterPrinter::PRINTABLE_SIZE }>::new();
        message.extend_from_slice(b"Enum(");
        message.extend_from_slice(arrform!(20, "{}u8, ", state.discriminator).as_bytes());

        for &type_id in &state.data {
            match to_type_info(type_id) {
                None => message.extend_from_slice(b"(unknown)"),
                Some(info) => message.extend_from_slice(to_kind_name(info.type_kind)),
            };
            message.extend_from_slice(b", ");
        }

        // Strip last comma
        message.pop();
        message.pop();

        message.push(b')');

        display.scroll(message.as_slice());
    }
}
