mod as_mut;
pub mod clone;
pub mod conversion;
pub mod version;

use nanos_ui::ui;

pub fn debug(message: &str) {
    ui::MessageScroller::new(message).event_loop();
}
