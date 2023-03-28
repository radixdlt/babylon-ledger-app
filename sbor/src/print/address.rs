use arrform::{arrform, ArrForm};

use crate::bech32::encoder::*;
use crate::bech32::hrp::*;
use crate::print::tty::TTY;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;
use crate::type_info::*;

// Address printers for ResourceAddress/ComponentAddress/PackageAddress/ManifestAddress
pub struct AddressParameterPrinter {
    resource_id: HrpType,
}

pub const RESOURCE_ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {
    resource_id: HrpType::FungibleResource,
};
pub const COMPONENT_ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {
    resource_id: HrpType::Component,
};
pub const PACKAGE_ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {
    resource_id: HrpType::Package,
};
pub const MANIFEST_ADDRESS_PARAMETER_PRINTER: AddressParameterPrinter = AddressParameterPrinter {
    resource_id: HrpType::Autodetect,
};

impl ParameterPrinter for AddressParameterPrinter {
    fn handle_data(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent
    ) {
        if let SborEvent::Data(byte) = event {
            // if state.flip_flop == false {
            //     state.flip_flop = true;
            //     if self.resource_id == HrpType::Autodetect {
            //         // See ManifestAddress enum in radixdlt-scrypto
            //         state.resource_id = match byte {
            //             0x00 => HrpType::Package,
            //             0x01 => HrpType::FungibleResource,
            //             0x02 => HrpType::NonFungibleResource,
            //             0x03..=0x0d => HrpType::Component,
            //             _ => HrpType::Autodetect,
            //         };
            //      } else {
            //          state.resource_id = self.resource_id;
            //      }
            // }

            state.push_byte(byte);
        }
    }
}

impl AddressParameterPrinter {
    pub fn tty(&self, state: &mut ParameterPrinterState) {
        if state.data.len() != (ADDRESS_LEN as usize) {
            state.tty.print_text(b"Invalid address format");
            return;
        }

        // match hrp_prefix(state.resource_id, state.data[0]) {
        //     None => {
        //         state.tty.print_text(b"Unknown address type");
        //         return;
        //     }
        //     Some(hrp_prefix) => self.format_address(&state, tty, hrp_prefix),
        // }
    }
}

impl AddressParameterPrinter {
    fn format_address(
        &self,
        state: &mut ParameterPrinterState,
        hrp_prefix: &str,
    ) {
        let encodind_result = Bech32::encode(
            arrform!(
                { Bech32::HRP_MAX_LEN },
                "{}{}",
                hrp_prefix,
                hrp_suffix(state.network_id)
            )
                .as_bytes(),
            state.data.as_slice(),
        );
        // match encodind_result {
        //     Ok(encoder) => state.tty.print_text(encoder.encoded()),
        //     Err(err) => {
        //         state.tty.print_text(
        //             arrform!(
        //                 { Bech32::HRP_MAX_LEN + 250 },
        //                 "Error decoding {:?}({}) address {:?}: >>{:?}<<",
        //                 state.resource_id,
        //                 state.data[0],
        //                 err,
        //                 state.data.as_slice()
        //             )
        //                 .as_bytes(),
        //         );
        //     }
        // }
    }
}

