use core::str::from_utf8;

use arrform::{arrform, ArrForm};

use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::print::tty::TTY;
use crate::sbor_decoder::SborEvent;
use core::{concat, stringify};
use paste::paste;

// Parameter which we just skip, without printing anything
pub struct IgnoredParameter {}

pub const IGNORED_PARAMETER_PRINTER: IgnoredParameter = IgnoredParameter {};

impl ParameterPrinter for IgnoredParameter {
    fn handle_data(
        &self,
        _state: &mut ParameterPrinterState,
        _event: SborEvent
    ) {
    }
}

impl IgnoredParameter {
    pub fn tty(&self, state: &mut ParameterPrinterState) {
        state.tty.print_text("<not decoded>".as_bytes())
    }
}
// BOOL parameter printer
pub struct BoolParameterPrinter {}

pub const BOOL_PARAMETER_PRINTER: BoolParameterPrinter = BoolParameterPrinter {};

impl ParameterPrinter for BoolParameterPrinter {
    fn handle_data(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent
    ) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }
}

impl BoolParameterPrinter {
    pub fn tty(&self, state: &mut ParameterPrinterState) {
        if state.data.len() != 1 {
            state.tty.print_text(b"<Invalid bool encoding>");
            return;
        }

        let message: &[u8] = match state.data.as_slice()[0] {
            0 => b"false",
            1 => b"true",
            _ => b"(invalid bool)",
        };

        state.tty.print_text(message);
    }
}

macro_rules! printer_for_type {
    ($type:ty) => {
        paste! {
            pub struct [<$type:upper ParameterPrinter>] {}
            pub const [<$type:upper _PARAMETER_PRINTER>] : [<$type:upper ParameterPrinter>] = [<$type:upper ParameterPrinter>] {};

            impl ParameterPrinter for [<$type:upper ParameterPrinter>] {
                fn handle_data(
                    &self,
                    state: &mut ParameterPrinterState,
                    event: SborEvent
                ) {
                    if let SborEvent::Data(byte) = event {
                        state.push_byte(byte);
                    }
                }

                fn end(&self, state: &mut ParameterPrinterState) {
                    if state.data.len() != (($type::BITS / 8) as usize) {
                        state.tty.print_text(b"<Invalid encoding>");
                        return;
                    }
                    fn to_array(input: &[u8]) -> [u8; ($type::BITS / 8) as usize] {
                        input.try_into().expect("<should not happen>")
                    }

                    let value = $type::from_le_bytes(to_array(state.data.as_slice()));

                    state.tty.print_text(arrform!(40, concat!("{}", stringify!($type)), value).as_bytes());
                }
            }
        }
    };
}

printer_for_type!(u8);
printer_for_type!(u16);
printer_for_type!(u32);
printer_for_type!(u64);
printer_for_type!(u128);
printer_for_type!(i8);
printer_for_type!(i16);
printer_for_type!(i32);
printer_for_type!(i64);
printer_for_type!(i128);

// String parameter printer
pub struct StringParameterPrinter {}

pub const STRING_PARAMETER_PRINTER: StringParameterPrinter = StringParameterPrinter {};

impl ParameterPrinter for StringParameterPrinter {
    fn handle_data(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent
    ) {
        if let SborEvent::Data(byte) = event {
            //TODO: split longer strings into chunks; keep in mind utf8 boundaries
            state.push_byte(byte);
        }
    }

    fn end(&self, state: &mut ParameterPrinterState) {
        match from_utf8(state.data.as_slice()) {
            Ok(message) => state.tty.print_text(message.as_bytes()),
            Err(_) => state.tty.print_text(b"<String decoding error>"),
        }
    }
}
