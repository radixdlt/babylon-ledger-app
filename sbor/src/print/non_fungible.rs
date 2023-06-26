use crate::print::parameter_printer::ParameterPrinter;
use crate::print::primitives::U64ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::type_info::*;

pub struct NonFungibleLocalIdParameterPrinter {}

pub const NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER: NonFungibleLocalIdParameterPrinter =
    NonFungibleLocalIdParameterPrinter {};

impl<T: Copy> ParameterPrinter<T> for NonFungibleLocalIdParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState<T>) {
        //See radix-engine-common/src/data/scrypto/model/non_fungible_local_id.rs
        match state.active_state().key_type_id {
            NFL_STRING => {
                if state.data.len() == 0 || state.data.len() > 64 {
                    state.print_text(b"<invalid non-fungible local id string>");
                    return;
                }
                state.print_byte(b'<');
                state.print_data_as_text();
                state.print_byte(b'>');
            }
            NFL_INTEGER => {
                if state.data.len() != (INTEGER_LEN as usize) {
                    state.print_text(b"<invalid non-fungible local id integer>");
                    return;
                }

                fn to_array(input: &[u8]) -> [u8; INTEGER_LEN as usize] {
                    input.try_into().expect("<should not happen>")
                }

                let value = u64::from_be_bytes(to_array(state.data.as_slice()));
                state.print_byte(b'#');
                U64ParameterPrinter::print(state, value);
                state.print_byte(b'#');
            }
            NFL_BYTES => {
                if state.data.len() == 0 || state.data.len() > 64 {
                    state.print_text(b"<invalid non-fungible local id bytes>");
                    return;
                }
                state.print_byte(b'[');
                state.print_data_as_hex();
                state.print_byte(b']');
            }
            NFL_RUID => {
                if state.data.len() != (RUID_LEN as usize) {
                    state.print_text(b"<invalid non-fungible local id UUID>");
                    return;
                }
                state.print_byte(b'{');
                state.print_data_as_hex_slice(0..8);
                state.print_byte(b'-');
                state.print_data_as_hex_slice(8..16);
                state.print_byte(b'-');
                state.print_data_as_hex_slice(16..24);
                state.print_byte(b'-');
                state.print_data_as_hex_slice(24..32);
                state.print_byte(b'}');
            }
            _ => {
                state.print_text(b"Id(<unknown type of non-fungible local id>)");
                return;
            }
        };
    }
}
