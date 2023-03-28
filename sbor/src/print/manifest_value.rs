use crate::print::tty::TTY;
use crate::print::address::*;
use crate::print::array::*;
use crate::print::decimals::*;
use crate::print::enums::*;
use crate::print::hex::*;
use crate::print::non_fungible::*;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::primitives::*;
use crate::print::state::ParameterPrinterState;
use crate::print::tuple::TUPLE_PARAMETER_PRINTER;
use crate::sbor_decoder::SborEvent;
use crate::type_info::*;

// Autoselecting parameter printer
pub struct ManifestValueParameterPrinter {}

pub const MANIFEST_VALUE_PARAMETER_PRINTER: ManifestValueParameterPrinter =
    ManifestValueParameterPrinter {};

pub fn get_printer_for_discriminator(discriminator: u8) -> &'static dyn ParameterPrinter {
    match discriminator {
        // Generic types
        TYPE_BOOL => &BOOL_PARAMETER_PRINTER,
        TYPE_I8 => &I8_PARAMETER_PRINTER,
        TYPE_I16 => &I16_PARAMETER_PRINTER,
        TYPE_I32 => &I32_PARAMETER_PRINTER,
        TYPE_I64 => &I64_PARAMETER_PRINTER,
        TYPE_I128 => &I128_PARAMETER_PRINTER,
        TYPE_U8 => &U8_PARAMETER_PRINTER,
        TYPE_U16 => &U16_PARAMETER_PRINTER,
        TYPE_U32 => &U32_PARAMETER_PRINTER,
        TYPE_U64 => &U64_PARAMETER_PRINTER,
        TYPE_U128 => &U128_PARAMETER_PRINTER,
        TYPE_STRING => &STRING_PARAMETER_PRINTER,
        TYPE_ARRAY => &ARRAY_PARAMETER_PRINTER,
        TYPE_TUPLE => &TUPLE_PARAMETER_PRINTER,
        TYPE_ENUM => &ENUM_PARAMETER_PRINTER,
        TYPE_MAP => &MANIFEST_VALUE_PARAMETER_PRINTER, // TODO: implement MAP_PARAMETER_PRINTER
        // Custom types
        TYPE_ADDRESS => &MANIFEST_ADDRESS_PARAMETER_PRINTER,
        TYPE_BUCKET => &U32_PARAMETER_PRINTER,
        TYPE_PROOF => &U32_PARAMETER_PRINTER,
        TYPE_EXPRESSION => &HEX_PARAMETER_PRINTER,
        TYPE_BLOB => &MANIFEST_BLOB_REF_PARAMETER_PRINTER,
        TYPE_DECIMAL => &DECIMAL_PARAMETER_PRINTER,
        TYPE_PRECISE_DECIMAL => &PRECISE_DECIMAL_PARAMETER_PRINTER,
        TYPE_NON_FUNGIBLE_LOCAL_ID => &NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER,
        _ => &IGNORED_PARAMETER_PRINTER,
    }
}

impl ParameterPrinter for ManifestValueParameterPrinter {
    fn handle_data(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent
    ) {
        // if state.nesting_level < 5 {
        //     return;
        // }
        // if state.nesting_level == 5 {
        //     match event {
        //         SborEvent::Start { type_id, .. } => {
        //             state.reset();
        //             state.manifest_discriminator = type_id;
        //
        //         },
        //         SborEvent::End { .. } => {
        //             //get_printer_for_discriminator(state.manifest_discriminator).tty(state, tty);
        //         },
        //         _ => {}
        //     };
        // }

        // let printer = get_printer_for_discriminator(state.manifest_discriminator);

        // if !printer.is_value_printer() {
        //     printer.handle_data(state, event, tty);
        // }
    }
    //
    // fn is_value_printer(&self) -> bool {
    //     true
    // }
}
