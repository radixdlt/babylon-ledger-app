use core::str::from_utf8;

use arrform::{arrform, ArrForm};

use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::print::tty::TTY;
use crate::sbor_decoder::SborEvent;
use crate::type_info::*;

// MethodKey parameter printer
pub struct MethodKeyParameterPrinter {}

pub const METHOD_KEY_PARAMETER_PRINTER: MethodKeyParameterPrinter = MethodKeyParameterPrinter {};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum MethodKeyPhase {
    Init,
    ModuleIdDiscrimitor,
    Ident,
}

impl From<u8> for MethodKeyPhase {
    fn from(ins: u8) -> MethodKeyPhase {
        match ins {
            0 => MethodKeyPhase::Init,
            1 => MethodKeyPhase::ModuleIdDiscrimitor,
            2 => MethodKeyPhase::Ident,
            _ => MethodKeyPhase::Init,
        }
    }
}

impl From<MethodKeyPhase> for u8 {
    fn from(ins: MethodKeyPhase) -> u8 {
        match ins {
            MethodKeyPhase::Init => 0,
            MethodKeyPhase::ModuleIdDiscrimitor => 1,
            MethodKeyPhase::Ident => 2,
        }
    }
}

fn module_id_to_name(byte: u8) -> &'static str {
    match byte {
        0 => "SELF",
        1 => "TypeInfo",
        2 => "Metadata",
        3 => "AccessRules",
        4 => "AccessRules1",
        5 => "ComponentRoyalty",
        6 => "PackageRoyalty",
        7 => "FunctionAccessRules",
        _ => "<Unknown>",
    }
}

impl ParameterPrinter for MethodKeyParameterPrinter {
    fn handle_data(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent
    ) {
        // let phase: MethodKeyPhase = state.phase.into();
        //
        // match phase {
        //     MethodKeyPhase::Init => {
        //         if let SborEvent::Start {
        //             type_id: TYPE_ENUM, ..
        //         } = event
        //         {
        //             state.phase = MethodKeyPhase::ModuleIdDiscrimitor.into();
        //         }
        //     }
        //     MethodKeyPhase::ModuleIdDiscrimitor => {
        //         if let SborEvent::Discriminator(byte) = event {
        //             state.discriminator = byte;
        //         }
        //         if let SborEvent::Start {
        //             type_id: TYPE_STRING,
        //             ..
        //         } = event
        //         {
        //             state.phase = MethodKeyPhase::Ident.into();
        //         }
        //     }
        //     MethodKeyPhase::Ident => {
        //         if let SborEvent::Data(byte) = event {
        //             //state.push_byte_for_string(byte);
        //         }
        //     }
        // };
    }
}

impl MethodKeyParameterPrinter {
    pub fn tty(&self, state: &mut ParameterPrinterState) {
        let text = match from_utf8(state.data.as_slice()) {
            Ok(text) => text,
            Err(_) => "<invalid string>",
        };

        // let message = arrform!(
        //     { ParameterPrinterState::PARAMETER_AREA_SIZE + 32 },
        //     "Key({} {})",
        //     module_id_to_name(state.discriminator),
        //     text
        // );
        //
        // state.tty.print_text(message.as_bytes());
    }
}
