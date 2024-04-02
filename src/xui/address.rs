use crate::ui::single_message::SingleMessage;
use crate::ui::utils::info_message;

#[cfg(not(target_os = "stax"))]
pub fn display(message: &[u8]) {
    info_message(b"Address:", message);
    SingleMessage::with_bold("\nDone\n").show_and_wait();
}

#[cfg(target_os = "stax")]
pub fn display(message: &[u8]) {}
