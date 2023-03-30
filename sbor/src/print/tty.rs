const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

pub trait TTY {
    fn start(&mut self);                //Begin new line
    fn end(&mut self);                  //End of line
    fn print_byte(&mut self, byte: u8);

    fn print_space(&mut self) {
        self.print_byte(b' ');
    }

    fn print_text(&mut self, text: &[u8]) {
        for &byte in text {
            self.print_byte(byte);
        }
    }

    fn print_hex_byte(&mut self, byte: u8) {
        self.print_byte(HEX_DIGITS[((byte >> 4) & 0x0F) as usize]);
        self.print_byte(HEX_DIGITS[(byte & 0x0F) as usize]);
    }

    fn print_hex_slice(&mut self, slice: &[u8]) {
        for &byte in slice {
            self.print_hex_byte(byte);
        }
    }
}
