use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

// Autoselecting parameter printer
pub struct ManifestValueParameterPrinter {}

pub const MANIFEST_VALUE_PARAMETER_PRINTER: ManifestValueParameterPrinter =
    ManifestValueParameterPrinter {};

impl ParameterPrinter for ManifestValueParameterPrinter {
    fn handle_data(&self, state: &mut ParameterPrinterState, event: SborEvent) {
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
