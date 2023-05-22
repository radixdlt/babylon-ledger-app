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
    fn show_message(title: &[u8], message: &[u8]) {
        MultilineMessageScroller::with_title(
            from_utf8(title).unwrap(),
            from_utf8(message).unwrap(),
            true,
        )
        .event_loop();
    }
}
