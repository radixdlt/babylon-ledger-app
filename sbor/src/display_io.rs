pub trait DisplayIO {
    fn scroll(&self, message: &[u8]);
    fn ask(&self, question: &str) -> bool;
}
