use nanos_sdk::testing::debug_print;
use nanos_ui::ui;
use sbor::print::state::ParameterPrinterState;
use sbor::print::tty::TTY;

use crate::utilities::debug_print_byte;

#[derive(Copy, Clone, Debug)]
pub struct LedgerTTY;

impl LedgerTTY {
    pub const fn new() -> TTY {
        TTY {
            start: Self::start,
            end: Self::end,
            print_byte: Self::print_byte,
        }
    }

    fn start(_state: &mut ParameterPrinterState) {
        debug_print("tty start\n");
    }

    fn end(_state: &mut ParameterPrinterState) {
        debug_print("tty end\n");
    }

    fn print_byte(state: &mut ParameterPrinterState, byte: u8) {
        debug_print("tty print_byte: ");
        debug_print_byte(byte.clone());
    }
}
