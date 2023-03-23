use staticvec::StaticVec;

use crate::display_io::DisplayIO;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

// Printer for various parameters formatted as hex string
pub struct HexParameterPrinter {
    fixed_len: u32,
}

const OBJECT_ID_LEN: u32 = 1 + 26 + 4; // ENTITY_BYTES_LENGTH + OBJECT_HASH_LENGTH + OBJECT_INDEX_LENGTH
pub const OBJECT_ID_PARAMETER_PRINTER: HexParameterPrinter = HexParameterPrinter {
    fixed_len: OBJECT_ID_LEN,
};
pub const MANIFEST_BLOB_REF_PARAMETER_PRINTER: HexParameterPrinter =
    HexParameterPrinter { fixed_len: 32 };
pub const HEX_PARAMETER_PRINTER: HexParameterPrinter = HexParameterPrinter { fixed_len: 0 };

impl HexParameterPrinter {
    const USER_INFO_SPACE_LEN: usize = 20; // "###/###" - show part of part
    const PRINTABLE_SIZE: usize =
        ParameterPrinterState::PARAMETER_AREA_SIZE * 2 + HexParameterPrinter::USER_INFO_SPACE_LEN;
}

pub fn to_hex<const N: usize>(data: &[u8], message: &mut StaticVec<u8, N>) {
    for &c in data.iter() {
        message.push(HEX_DIGITS[((c >> 4) & 0x0F) as usize]);
        message.push(HEX_DIGITS[(c & 0x0F) as usize]);
    }
}

const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

impl ParameterPrinter for HexParameterPrinter {
    fn handle_data_event(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent,
        display: &'static dyn DisplayIO,
    ) {
        // TODO: show to user that this is 'piece # of ##'
        if let SborEvent::Len(len) = event {
            if self.fixed_len > 0 && self.fixed_len != len {
                display.scroll(b"<payload size mismatch>");
                state.flip_flop = true;
            }
            state.expected_len = len;
        }

        // If error is triggered, ignore remaining data
        if state.flip_flop {
            return;
        }

        if let SborEvent::Data(byte) = event {
            if state.data_counter as usize == Self::PRINTABLE_SIZE {
                self.display(state, display);
                state.data_counter = 0;
                return;
            }

            state.push_byte(byte);
        }
    }

    fn display(&self, state: &ParameterPrinterState, display: &'static dyn DisplayIO) {
        if state.flip_flop {
            return;
        }

        let mut message = StaticVec::<u8, { HexParameterPrinter::PRINTABLE_SIZE }>::new();

        message.insert_from_slice(0, b"Hex(");
        to_hex(state.data(), &mut message);
        message.push(b')');

        display.scroll(message.as_slice());
    }
}
