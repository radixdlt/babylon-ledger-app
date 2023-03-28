use arrform::{arrform, ArrForm};
use staticvec::StaticVec;

use crate::print::tty::TTY;
use crate::print::hex::to_hex;
use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::sbor_decoder::SborEvent;

// NonFungibleLocalId parameter printer
pub struct NonFungibleLocalIdParameterPrinter {}

pub const NON_FUNGIBLE_LOCAL_ID_PARAMETER_PRINTER: NonFungibleLocalIdParameterPrinter =
    NonFungibleLocalIdParameterPrinter {};

impl NonFungibleLocalIdParameterPrinter {
    const MAX_DISPLAY_LEN: usize = 80;
}

impl ParameterPrinter for NonFungibleLocalIdParameterPrinter {
    fn handle_data(
        &self,
        state: &mut ParameterPrinterState,
        event: SborEvent
    ) {
        match event {
            SborEvent::Data(byte) => state.push_byte(byte),
            SborEvent::Discriminator(discriminator) => {
                // state.inner_discriminator = discriminator;
                //state.flip_flop = true;
            }
            _ => {}
        }
    }
}

impl NonFungibleLocalIdParameterPrinter {
    pub fn tty(&self, state: &mut ParameterPrinterState) {
        // if !state.flip_flop {
        //     state.tty.print_text(b"Id(<unable to recognize non-fungible local id>)");
        //     return;
        // }

        let mut message =
            StaticVec::<u8, { NonFungibleLocalIdParameterPrinter::MAX_DISPLAY_LEN }>::new();
        // See radix-engine-common/src/data/scrypto/model/non_fungible_local_id.rs
        // match state.inner_discriminator {
        //     // String
        //     0 => {
        //         if state.data.len() == 0 || state.data.len() > 64 {
        //             state.tty.print_text(b"<invalid non-fungible local id string>");
        //             return;
        //         }
        //         message.push(b'<');
        //         message.extend_from_slice(state.data.as_slice());
        //         message.push(b'>');
        //     }
        //     // Integer
        //     1 => {
        //         if state.data.len() != 8 {
        //             state.tty.print_text(b"<invalid non-fungible local id integer>");
        //             return;
        //         }
        //
        //         fn to_array(input: &[u8]) -> [u8; 8] {
        //             input.try_into().expect("<should not happen>")
        //         }
        //
        //         let value = u64::from_be_bytes(to_array(state.data.as_slice()));
        //         message.extend_from_slice(arrform!(20, "#{}#", value).as_bytes());
        //     }
        //     // Bytes
        //     2 => {
        //         if state.data.len() == 0 || state.data.len() > 64 {
        //             state.tty.print_text(b"<invalid non-fungible local id bytes>");
        //             return;
        //         }
        //         message.push(b'[');
        //         to_hex(state.data.as_slice(), &mut message);
        //         message.push(b']');
        //     }
        //     // UUID
        //     3 => {
        //         if state.data.len() != 16 {
        //             state.tty.print_text(b"<invalid non-fungible local id UUID>");
        //             return;
        //         }
        //         message.push(b'{');
        //         to_hex(&state.data.as_slice()[0..4], &mut message);
        //         message.push(b'-');
        //         to_hex(&state.data.as_slice()[4..6], &mut message);
        //         message.push(b'-');
        //         to_hex(&state.data.as_slice()[6..8], &mut message);
        //         message.push(b'-');
        //         to_hex(&state.data.as_slice()[8..10], &mut message);
        //         message.push(b'-');
        //         to_hex(&state.data.as_slice()[10..16], &mut message);
        //         message.push(b'}');
        //     }
        //     _ => {
        //         state.tty.print_text(b"Id(<unknown type of non-fungible local id>)");
        //         return;
        //     }
        // };

        state.tty.print_text(message.as_slice());
    }
}
