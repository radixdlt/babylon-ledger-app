use crate::ui::single_message::SingleMessage;
use crate::ui::utils::info_message;

pub fn display_address(message: &[u8]) {
    info_message(b"Address:", message);
    SingleMessage::with_bold("\nDone\n").show_and_wait();
}
