mod as_mut;
pub mod clone;
pub mod conversion;
pub mod version;

use core::str::from_utf8;
use nanos_ui::ui;

pub fn debug_arr(message: &[u8]) {
    debug(from_utf8(message).unwrap());
}

pub fn debug(message: &str) {
    ui::MessageScroller::new(message).event_loop();
}
