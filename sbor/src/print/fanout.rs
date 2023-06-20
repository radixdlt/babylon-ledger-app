use crate::instruction_extractor::{ExtractorEvent, InstructionHandler};
use crate::print::instruction_printer::InstructionPrinter;
use crate::print::tx_summary_detector::TxSummaryDetector;

pub struct Fanout<'a, T: Copy> {
    ins_printer: &'a mut InstructionPrinter<T>,
    tx_printer: &'a mut TxSummaryDetector,
}

impl<'a, T: Copy> Fanout<'a, T> {
    pub fn new(
        ins_printer: &'a mut InstructionPrinter<T>,
        tx_printer: &'a mut TxSummaryDetector,
    ) -> Self {
        Self {
            ins_printer,
            tx_printer,
        }
    }
}

impl<'a, T: Copy> InstructionHandler for Fanout<'a, T> {
    fn handle(&mut self, evt: ExtractorEvent) {
        self.ins_printer.handle(evt);
        self.tx_printer.handle(evt);
    }
}
