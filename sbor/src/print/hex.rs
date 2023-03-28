use staticvec::StaticVec;

use crate::print::tty::TTY;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::{ParameterPrinterState, PARAMETER_AREA_SIZE};
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
    const PRINTABLE_SIZE: usize = PARAMETER_AREA_SIZE * 2 + HexParameterPrinter::USER_INFO_SPACE_LEN;
}

pub fn to_hex<const N: usize>(data: &[u8], message: &mut StaticVec<u8, N>) {
    for &c in data.iter() {
        message.push(HEX_DIGITS[((c >> 4) & 0x0F) as usize]);
        message.push(HEX_DIGITS[(c & 0x0F) as usize]);
    }
}

const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

impl ParameterPrinter for HexParameterPrinter {
    fn handle_data(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent
    ) {
        // TODO: show to user that this is 'piece # of ##'
        // if let SborEvent::Len(len) = event {
        //     if self.fixed_len > 0 && self.fixed_len != len {
        //         state.tty.print_text(b"<payload size mismatch>");
        //         //state.flip_flop = true;
        //     }
        //     state.expected_len = len;
        // }

        // If error is triggered, ignore remaining data
        // if state.flip_flop {
        //     return;
        // }

        if let SborEvent::Data(byte) = event {
            state.push_byte(byte);
        }
    }
}

impl HexParameterPrinter {
    pub fn tty(&self, state: &mut ParameterPrinterState) {
        // if state.flip_flop {
        //     return;
        // }

        let mut message = StaticVec::<u8, { Self::PRINTABLE_SIZE }>::new();

        message.extend_from_slice(b"Bytes(");
        to_hex(state.data.as_slice(), &mut message);
        message.push(b')');

        state.tty.print_text(message.as_slice());
    }
}
