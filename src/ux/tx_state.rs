use crate::ui::multiline_scroller::MultilineMessageScroller;
use crate::ui::single_message::SingleMessage;

#[cfg(target_os = "stax")]
pub fn display_max_fee(text: &[u8]) {}

#[cfg(not(target_os = "stax"))]
pub fn display_max_fee(text: &[u8]) {
    non_stax::display_max_fee(text)
}

#[cfg(target_os = "stax")]
pub fn display_introductory_screen(text: &str) {}

#[cfg(not(target_os = "stax"))]
pub fn display_introductory_screen(text: &str) {
    non_stax::display_introductory_screen(text)
}

#[cfg(target_os = "stax")]
pub fn display_sign_auth(address: &[u8], origin: &[u8], nonce_hex: &mut [u8; 64]) -> bool {
    false
}
#[cfg(not(target_os = "stax"))]
pub fn display_sign_auth(address: &[u8], origin: &[u8], nonce_hex: &mut [u8; 64]) -> bool {
    non_stax::display_sign_auth(address, origin, nonce_hex)
}


mod stax {}
mod non_stax {
    use crate::ui::multiline_scroller::MultilineMessageScroller;
    use crate::ui::multipage_validator::MultipageValidator;
    use crate::ui::single_message::SingleMessage;
    use crate::ui::utils;

    pub fn display_max_fee(text: &[u8]) {
        MultilineMessageScroller::with_title(
            "Max TX Fee:",
            core::str::from_utf8(text).unwrap(),
            true,
        )
        .event_loop();
    }

    pub fn display_introductory_screen(text: &str) {
        SingleMessage::with_right_arrow(text).show_and_wait();
    }

    pub fn display_sign_auth(address: &[u8], origin: &[u8], nonce_hex: &mut [u8; 64]) -> bool {
        utils::info_message(b"Origin:", origin);
        utils::info_message(b"dApp Address:", address);
        utils::info_message(b"Nonce:", &nonce_hex);

        MultipageValidator::new(&["Sign Proof?"], &["Sign"], &["Reject"]).ask()
    }
}
