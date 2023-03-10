pub trait ByteReceiver {
    fn push(&mut self, byte: u8);

    fn push_all(&mut self, data: &[u8]) {
        for &byte in data {
            self.push(byte);
        }
    }
}