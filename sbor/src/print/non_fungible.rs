use crate::print::parameter_printer::ParameterPrinter;
use crate::print::primitives::U64ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::print::tty::TTY;
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
                    state.print_text(b"<invalid non-fungible local id string>");
                    return;
                }
                state.print_byte(b'<');
                state.print_data_as_text();
                state.print_byte(b'>');
            }
            // Integer
            1 => {
                if state.data.len() != 8 {
                    state.print_text(b"<invalid non-fungible local id integer>");
                    return;
                }

                fn to_array(input: &[u8]) -> [u8; 8] {
                    input.try_into().expect("<should not happen>")
                }

                let value = u64::from_be_bytes(to_array(state.data.as_slice()));
                state.print_byte(b'#');
                U64ParameterPrinter::print(state, value);
                state.print_byte(b'#');
            }
            // Bytes
            2 => {
                if state.data.len() == 0 || state.data.len() > 64 {
                    state.print_text(b"<invalid non-fungible local id bytes>");
                    return;
                }
                state.print_byte(b'[');
                state.print_data_as_hex();
                state.print_byte(b']');
            }
            // UUID
            3 => {
                if state.data.len() != 16 {
                    state.print_text(b"<invalid non-fungible local id UUID>");
                    return;
                }
                state.print_byte(b'{');
                state.print_data_as_hex_slice(0..4);
                state.print_byte(b'-');
                state.print_data_as_hex_slice(4..6);
                state.print_byte(b'-');
                state.print_data_as_hex_slice(6..8);
                state.print_byte(b'-');
                state.print_data_as_hex_slice(8..10);
                state.print_byte(b'-');
                state.print_data_as_hex_slice(10..16);
                state.print_byte(b'}');
            }
            _ => {
                state.print_text(b"Id(<unknown type of non-fungible local id>)");
                return;
            }
        };
    }
}
