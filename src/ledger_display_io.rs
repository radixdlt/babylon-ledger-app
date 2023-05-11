use crate::ui::multiline_scroller::MultilineMessageScroller;
use core::str::from_utf8;
use sbor::print::tty::TTY;

#[derive(Copy, Clone, Debug)]
pub struct LedgerTTY;

impl LedgerTTY {
    pub const fn new_tty() -> TTY {
        TTY {
            show_message: Self::show_message,
        }
    }
    fn show_message(message: &[u8]) {
        match from_utf8(message) {
            Ok(str) => {
                MultilineMessageScroller::new(str).event_loop();
            }
            // TODO: handle this error?
            Err(_) => {
                MultilineMessageScroller::new("Invalid content").event_loop();
            }
        }
    }
}
