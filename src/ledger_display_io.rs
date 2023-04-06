use core::str::from_utf8;
use nanos_sdk::testing::debug_print;
use nanos_ui::ui;
use sbor::print::tty::TTY;
//use staticvec::StaticVec;
use crate::utilities::debug_print_byte;

#[derive(Copy, Clone, Debug)]
pub struct LedgerTTY {
   //text: StaticVec<u8, { LedgerTTY::DISPLAY_BUFFER_SIZE }>,
   //  text:[u8;LedgerTTY::DISPLAY_BUFFER_SIZE],
   //  line: usize,
}

impl LedgerTTY {
    pub const DISPLAY_BUFFER_SIZE: usize = 16;

    pub const fn new() -> Self {
        Self {
           //text: StaticVec::<u8, { LedgerTTY::DISPLAY_BUFFER_SIZE }>::new(),
           //  text:[0;LedgerTTY::DISPLAY_BUFFER_SIZE],
           //  line: 0,
        }
    }
}

impl TTY for LedgerTTY {
    fn start(&mut self) {
        debug_print("\n<TTY START>\n");
        //self.text.clear();
    }

    fn end(&mut self) {
        debug_print("\n<TTY END>\n");
//        ui::MessageScroller::new(from_utf8(self.text.as_slice()).unwrap()).event_loop();
    }

    fn print_byte(&mut self, byte: u8) {
        //debug_print_byte(byte.clone());
        //self.text.push(byte);
    }
}
