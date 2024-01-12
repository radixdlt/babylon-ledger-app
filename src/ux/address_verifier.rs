
#[cfg(not(target_os = "stax"))]
pub fn display_address(message: &[u8]) {
    non_stax::display_address(message)
}

#[cfg(target_os = "stax")]
pub fn display_address(message: &[u8]) {
    stax::display_address(message)
}

#[cfg(target_os = "stax")]
mod stax {
    pub fn display_address(message: &[u8]) {
        //TODO: STAX UI
    }
}
#[cfg(not(target_os = "stax"))]
mod non_stax {
    use crate::ui::single_message::SingleMessage;
    use crate::ui::utils::info_message;

    pub fn display_address(message: &[u8]) {
        info_message(b"Address:", message);
        SingleMessage::with_bold("\nDone\n").show_and_wait();
    }
}
