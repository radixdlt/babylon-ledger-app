use arrform::{arrform, ArrForm};
use staticvec::StaticVec;

use crate::display_io::DisplayIO;
use crate::print::hex::to_hex;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

// NonFungibleLocalId parameter printer
pub struct NonFungibleLocalIdParameterPrinter {}

pub const NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER: NonFungibleLocalIdParameterPrinter =
    NonFungibleLocalIdParameterPrinter {};

impl NonFungibleLocalIdParameterPrinter {
    const MAX_DISPLAY_LEN: usize = 80;
}

impl ParameterPrinter for NonFungibleLocalIdParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        match event {
            SborEvent::Data(byte) => state.push_byte(byte),
            SborEvent::Discriminator(discriminator) => {
                state.inner_discriminator = discriminator;
                state.flip_flop = true;
            }
            _ => {}
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if !state.flip_flop {
            display.scroll(b"Id(<unable to recognize non-fungible local id>)");
            return;
        }

        let mut message =
            StaticVec::<u8, { NonFungibleLocalIdParameterPrinter::MAX_DISPLAY_LEN }>::new();
        // See radix-engine-common/src/data/scrypto/model/non_fungible_local_id.rs
        match state.inner_discriminator {
            // String
            0 => {
                if state.data_counter == 0 || state.data_counter > 64 {
                    display.scroll(b"<invalid non-fungible local id string>");
                    return;
                }
                message.push(b'<');
                message.extend_from_slice(state.data());
                message.push(b'>');
            }
            // Integer
            1 => {
                if state.data_counter != 8 {
                    display.scroll(b"<invalid non-fungible local id integer>");
                    return;
                }

                fn to_array(input: &[u8]) -> [u8; 8] {
                    input.try_into().expect("<should not happen>")
                }

                let value = u64::from_be_bytes(to_array(state.data()));
                message.extend_from_slice(arrform!(20, "#{}#", value).as_bytes());
            }
            // Bytes
            2 => {
                if state.data_counter == 0 || state.data_counter > 64 {
                    display.scroll(b"<invalid non-fungible local id bytes>");
                    return;
                }
                message.push(b'[');
                to_hex(state.data(), &mut message);
                message.push(b']');
            }
            // UUID
            3 => {
                if state.data_counter != 16 {
                    display.scroll(b"<invalid non-fungible local id UUID>");
                    return;
                }
                message.push(b'{');
                to_hex(&state.data[0..4], &mut message);
                message.push(b'-');
                to_hex(&state.data[4..6], &mut message);
                message.push(b'-');
                to_hex(&state.data[6..8], &mut message);
                message.push(b'-');
                to_hex(&state.data[8..10], &mut message);
                message.push(b'-');
                to_hex(&state.data[10..16], &mut message);
                message.push(b'}');
            }
            _ => {
                display.scroll(b"Id(<unknown type of non-fungible local id>)");
                return;
            }
        };

        display.scroll(message.as_slice());
    }
}
