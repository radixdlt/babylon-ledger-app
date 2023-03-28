use arrform::{arrform, ArrForm};

use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

pub struct NonFungibleLocalIdParameterPrinter {}

pub const NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER: NonFungibleLocalIdParameterPrinter =
    NonFungibleLocalIdParameterPrinter {};

impl ParameterPrinter for NonFungibleLocalIdParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        //See radix-engine-common/src/data/scrypto/model/non_fungible_local_id.rs
        match state.active_state().key_type_id {
            // String
            0 => {
                if state.data.len() == 0 || state.data.len() > 64 {
                    state
                        .tty
                        .print_text(b"<invalid non-fungible local id string>");
                    return;
                }
                state.tty.print_byte(b'<');
                state.tty.print_text(state.data.as_slice());
                state.tty.print_byte(b'>');
            }
            // Integer
            1 => {
                if state.data.len() != 8 {
                    state
                        .tty
                        .print_text(b"<invalid non-fungible local id integer>");
                    return;
                }

                fn to_array(input: &[u8]) -> [u8; 8] {
                    input.try_into().expect("<should not happen>")
                }

                let value = u64::from_be_bytes(to_array(state.data.as_slice()));
                state.tty.print_text(arrform!(20, "#{}#", value).as_bytes());
            }
            // Bytes
            2 => {
                if state.data.len() == 0 || state.data.len() > 64 {
                    state
                        .tty
                        .print_text(b"<invalid non-fungible local id bytes>");
                    return;
                }
                state.tty.print_byte(b'[');
                state.tty.print_hex_slice(state.data.as_slice());
                state.tty.print_byte(b']');
            }
            // UUID
            3 => {
                if state.data.len() != 16 {
                    state
                        .tty
                        .print_text(b"<invalid non-fungible local id UUID>");
                    return;
                }
                state.tty.print_byte(b'{');
                state.tty.print_hex_slice(&state.data.as_slice()[0..4]);
                state.tty.print_byte(b'-');
                state.tty.print_hex_slice(&state.data.as_slice()[4..6]);
                state.tty.print_byte(b'-');
                state.tty.print_hex_slice(&state.data.as_slice()[6..8]);
                state.tty.print_byte(b'-');
                state.tty.print_hex_slice(&state.data.as_slice()[8..10]);
                state.tty.print_byte(b'-');
                state.tty.print_hex_slice(&state.data.as_slice()[10..16]);
                state.tty.print_byte(b'}');
            }
            _ => {
                state
                    .tty
                    .print_text(b"Id(<unknown type of non-fungible local id>)");
                return;
            }
        };
    }
}
