use crate::print::state::ParameterPrinterState;

#[derive(Copy, Clone)]
pub struct TTY {
    pub start: fn(&mut ParameterPrinterState),
    pub end: fn(&mut ParameterPrinterState),
    pub print_byte: fn(&mut ParameterPrinterState, byte: u8),
}
