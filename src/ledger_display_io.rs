use core::str::from_utf8;
use nanos_ui::ui;
use sbor::print::tty::TTY;
use staticvec::StaticVec;

pub struct LedgerTTY {
    text: StaticVec<u8, { LedgerTTY::DISPLAY_BUFFER_SIZE }>,
}

impl LedgerTTY {
    pub const DISPLAY_BUFFER_SIZE: usize = 512;

    pub fn new() -> Self {
        Self {
            text: StaticVec::<u8, { LedgerTTY::DISPLAY_BUFFER_SIZE }>::new(),
        }
    }
}

impl TTY for LedgerTTY {
    fn start(&mut self) {
        self.text.clear();
    }

    fn end(&mut self) {
        ui::MessageScroller::new(from_utf8(self.text.as_slice()).unwrap()).event_loop();
    }

    fn print_byte(&mut self, byte: u8) {
        self.text.push(byte);
    }

    // fn ask(&self, question: &str) -> bool {
    //     ui::Validator::new(question).ask()
    // }
}
