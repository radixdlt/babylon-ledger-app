use crate::instruction_extractor::{ExtractorEvent, InstructionHandler};
use crate::print::instruction_printer::InstructionPrinter;
use crate::print::tx_summary_detector::TxSummaryDetector;

pub struct Fanout<'a, T: Copy> {
    printer: &'a mut InstructionPrinter<T>,
    detector: &'a mut TxSummaryDetector,
}

impl<'a, T: Copy> Fanout<'a, T> {
    pub fn new(
        printer: &'a mut InstructionPrinter<T>,
        detector: &'a mut TxSummaryDetector,
    ) -> Self {
        Self { printer, detector }
    }
}

impl<'a, T: Copy> InstructionHandler for Fanout<'a, T> {
    fn handle(&mut self, evt: ExtractorEvent) {
        self.printer.handle(evt);
        self.detector.handle(evt);
    }
}
