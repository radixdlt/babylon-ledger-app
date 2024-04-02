use crate::ui::multiline_scroller::MultilineMessageScroller;
use core::str::from_utf8;

#[cfg(not(target_os = "stax"))]
pub fn display_message_with_title(title: &[u8], message: &[u8]) {
    MultilineMessageScroller::with_title(
        from_utf8(title).unwrap(),
        from_utf8(message).unwrap(),
        true,
    )
    .event_loop();
}

#[cfg(target_os = "stax")]
pub fn display_message_with_title(title: &[u8], message: &[u8]) {}