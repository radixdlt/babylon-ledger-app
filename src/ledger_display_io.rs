use sbor::display_io::DisplayIO;

pub struct LedgerDisplayIO {}

impl DisplayIO for LedgerDisplayIO {
    fn scroll(&self, message: &[u8]) {
        todo!()
    }

    fn ask(&self, question: &str) -> bool {
        todo!()
    }
}
