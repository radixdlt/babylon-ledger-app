use crate::static_vec::StaticVec;

use crate::bech32::encoder::*;
use crate::bech32::hrp::*;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::type_info::*;

pub struct AddressParameterPrinter {}

pub const ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {};

impl<T: Copy> ParameterPrinter<T> for AddressParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState<T>) {
        if state.data.len() != (ADDRESS_LEN as usize) {
            state.print_text(b"Invalid address format");
            return;
        }

        // Unwrap is safe because we checked the length
        match hrp_prefix(state.data.first().unwrap()) {
            Some(prefix) => format_address(state, prefix),
            None => state.print_text(b"Address(unknown type)"),
        }
    }
}

fn format_address<T: Copy>(state: &mut ParameterPrinterState<T>, hrp_prefix: &str) {
    let mut vec = StaticVec::<u8, { Bech32::MAX_LEN }>::new(0);
    vec.extend_from_slice(hrp_prefix.as_bytes());
    vec.extend_from_slice(hrp_suffix(state.network_id).as_bytes());

    let encodind_result = Bech32::encode(vec.as_slice(), state.data.as_slice());

    match encodind_result {
        Ok(encoder) => {
            state.print_text(b"Address(");
            state.print_text(encoder.encoded());
            state.print_byte(b')');
        }
        Err(..) => state.print_text(b"Address(<bech32 error>)"),
    }
}
