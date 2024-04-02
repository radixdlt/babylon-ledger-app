use sbor::print::tty::TTY;
use crate::xui::instruction;

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
        instruction::display_message_with_title(title, message);
    }
}
