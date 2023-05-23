use crate::static_vec::StaticVec;

use crate::bech32::encoder::*;
use crate::bech32::hrp::*;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::type_info::*;

pub struct AddressParameterPrinter {}

pub const ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {};

fn to_hrp_type(id: u8) -> Option<HrpType> {
    // Depends on EntityType enum
    match id {
        0 => Some(HrpType::Package),
        1 => Some(HrpType::Resource),
        2 => Some(HrpType::Resource),
        3 => Some(HrpType::EpochManager),
        4 => Some(HrpType::Validator),
        5 => Some(HrpType::Clock),
        6 => Some(HrpType::AccessController),
        7 => Some(HrpType::Account),
        8 => Some(HrpType::Identity),
        9 => Some(HrpType::Component),
        10 => Some(HrpType::Account),
        11 => Some(HrpType::Account),
        12 => Some(HrpType::Identity),
        13 => Some(HrpType::Identity),
        14 => Some(HrpType::InternalVault),
        15 => Some(HrpType::InternalVault),
        16 => Some(HrpType::InternalAccount),
        17 => Some(HrpType::InternalKeyValueStore),
        18 => Some(HrpType::InternalComponent),
        _ => None,
    }
}

impl<T> ParameterPrinter<T> for AddressParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState<T>) {
        if state.data.len() != (ADDRESS_LEN as usize) {
            state.print_text(b"Invalid address format");
            return;
        }

        // Unwrap is safe because we checked the length
        match to_hrp_type(state.data.first().unwrap()) {
            Some(hrp_type) => format_address(state, hrp_prefix(hrp_type)),
            None => state.print_text(b"Address(unknown type)"),
        }
    }
}

fn format_address<T>(state: &mut ParameterPrinterState<T>, hrp_prefix: &str) {
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
