use core::str::from_utf8;

use arrform::{arrform, ArrForm};

use crate::display_io::DisplayIO;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

// Parameter which we just skip, without printing anything
pub struct IgnoredParameter {}

pub const IGNORED_PARAMETER_PRINTER: IgnoredParameter = IgnoredParameter {};

impl ParameterPrinter for IgnoredParameter {
    fn handle_data_event(
        &self,
        _state: &mut ParameterPrinterState,
        _event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
    }
    fn display(&self, _state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        display.scroll("<not decoded>".as_bytes())
    }
}

// U8 parameter printer
pub struct U8ParameterPrinter {}

pub const U8_PARAMETER_PRINTER: U8ParameterPrinter = U8ParameterPrinter {};

impl ParameterPrinter for U8ParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            state.data[0] = byte;
            state.data_counter = 1;
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.data_counter != 1 {
            //TODO: an error condition, should we handle it somehow?
            return;
        }

        display.scroll(arrform!(8, "{}u8", state.data[0]).as_bytes());
    }
}

// U32 parameter printer
pub struct U32ParameterPrinter {}

pub const U32_PARAMETER_PRINTER: U32ParameterPrinter = U32ParameterPrinter {};

impl ParameterPrinter for U32ParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.data_counter != 4 {
            display.scroll(b"<Invalid u32 encoding>");
            return;
        }

        fn to_array(input: &[u8]) -> [u8; 4] {
            input.try_into().expect("<should not happen>")
        }

        let value = u32::from_le_bytes(to_array(state.data()));

        display.scroll(arrform!(20, "{}u32", value).as_bytes());
    }
}

// String parameter printer
pub struct StringParameterPrinter {}

pub const STRING_PARAMETER_PRINTER: StringParameterPrinter = StringParameterPrinter {};

impl ParameterPrinter for StringParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        _display: &'static dyn DisplayIO,
    ) {
        if let SborEvent::Data(byte) = event {
            //TODO: split longer strings into chunks; keep in mind utf8 boundaries
            state.push_byte_for_string(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        match from_utf8(state.data()) {
            Ok(message) => display.scroll(message.as_bytes()),
            Err(_) => display.scroll(b"<String decoding error>"),
        }
    }
}

