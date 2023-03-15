use core::str::from_utf8;
use sbor::display_io::DisplayIO;
use nanos_ui::ui;

pub struct LedgerDisplayIO {}

impl DisplayIO for LedgerDisplayIO {
    fn scroll(&self, message: &[u8]) {
        ui::MessageScroller::new(from_utf8(message).unwrap()).event_loop();
    }

    fn ask(&self, question: &str) -> bool {
        ui::Validator::new(question).ask()
    }
}
