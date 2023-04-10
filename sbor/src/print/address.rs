use staticvec::StaticVec;

use crate::bech32::encoder::*;
use crate::bech32::hrp::*;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;
use crate::type_info::*;

pub struct AddressParameterPrinter {}

pub const ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {};

impl ParameterPrinter for AddressParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState) {
        if state.data.len() != (ADDRESS_LEN as usize) {
            state.print_text(b"Invalid address format");
            return;
        }

        let resource_id = match state.data[0] {
            0x00 => HrpType::Package,
            0x01 => HrpType::FungibleResource,
            0x02 => HrpType::NonFungibleResource,
            0x03..=0x0d => HrpType::Component,
            _ => HrpType::Autodetect,
        };

        match hrp_prefix(resource_id, state.data[0]) {
            None => {
                state.print_text(b"Address(unknown type)");
                return;
            }
            Some(hrp_prefix) => format_address(state, hrp_prefix),
        }
    }
}

fn format_address(state: &mut ParameterPrinterState, hrp_prefix: &str) {
    let mut vec = StaticVec::<u8, { Bech32::MAX_LEN }>::new();
    vec.extend_from_slice(hrp_prefix.as_bytes());
    vec.extend_from_slice(hrp_suffix(state.network_id).as_bytes());

    let encodind_result = Bech32::encode(
        vec.as_slice(),
        state.data.as_slice(),
    );

    match encodind_result {
        Ok(encoder) => {
            state.print_text(b"Address(");
            state.print_text(encoder.encoded());
            state.print_byte(b')');
        }
        Err(..) => state.print_text(b"Address(<bech32 error>)"),
    }
}
