use core::str::from_utf8;
use nanos_ui::ui;
use sbor::print::tty::TTY;

use crate::utilities::debug_prepared_message;

#[derive(Copy, Clone, Debug)]
pub struct LedgerTTY;

impl LedgerTTY {
    pub const fn new() -> TTY {
        TTY {
            show_message: Self::show_message,
        }
    }
    fn show_message(message: &[u8]) {
        debug_prepared_message(message);
        match from_utf8(message) {
            Ok(str) => {ui::MessageScroller::new(str).event_loop();}
            // TODO: handle this error
            Err(_) => {ui::MessageScroller::new("Invalid content").event_loop();}
        }
    }
}
