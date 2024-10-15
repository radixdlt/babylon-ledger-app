use crate::xui::titled_message;
use sbor::print::tty::TTY;

#[derive(Copy, Clone, Debug)]
pub struct LedgerTTY;

impl LedgerTTY {
    pub const fn new_tty() -> TTY<()> {
        TTY {
            data: (),
            show_message: Self::show_message,
        }
    }
    fn show_message(_: &mut (), title: &[u8], message: &[u8]) {
        titled_message::display(title, message);
    }
}
