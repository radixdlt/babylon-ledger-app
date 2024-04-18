use crate::ui::single_message::SingleMessage;
use crate::xui::titled_message;

#[cfg(not(target_os = "stax"))]
pub fn display(message: &[u8]) {
    titled_message::display(b"Address:", message);
    SingleMessage::with_bold("\nDone\n").show_and_wait_both_click();
}

#[cfg(target_os = "stax")]
pub fn display(message: &[u8]) {}
