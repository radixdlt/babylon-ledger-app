use crate::ui::multiline_scroller::MultilineMessageScroller;
use crate::ui::single_message::SingleMessage;
use core::str::from_utf8;
use ledger_device_sdk::ui::bagls::CROSSMARK_ICON;

#[cfg(not(target_os = "stax"))]
pub fn display(title: &[u8], message: &[u8]) {
    MultilineMessageScroller::with_title(
        from_utf8(title).unwrap(),
        from_utf8(message).unwrap(),
        true,
    )
    .event_loop();
}

#[cfg(not(target_os = "stax"))]
pub fn display_error(message: &str) {
    SingleMessage::with_icon(message, CROSSMARK_ICON).show_and_wait();
}

#[cfg(target_os = "stax")]
pub fn display(title: &[u8], message: &[u8]) {}
#[cfg(target_os = "stax")]
pub fn display_error(message: &str) {}
