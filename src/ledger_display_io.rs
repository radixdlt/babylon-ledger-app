use core::str::from_utf8;
use nanos_ui::ui;
use sbor::debug::debug_print;
use sbor::print::tty::TTY;

#[derive(Copy, Clone, Debug)]
pub struct LedgerTTY;

impl LedgerTTY {
    pub const fn new() -> TTY {
        TTY {
            show_message: Self::show_message,
        }
    }
    fn show_message(message: &[u8]) {
        match from_utf8(message) {
            Ok(str) => {
                ui::MessageScroller::new(str).event_loop();
            }
            // TODO: handle this error
            Err(_) => {
                ui::MessageScroller::new("Invalid content").event_loop();
            }
        }
    }
}
