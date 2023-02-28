#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(result_option_inspect)]
#![feature(const_cmp)]

use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io::{Comm, Event};
use nanos_ui::ui::SingleMessage;

use handler::dispatcher;

use crate::app_error::AppError;
use crate::command::Command;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::tx_sign_state::TxSignState;
use crate::utilities::version::{MODEL_DATA, VERSION_DATA};

mod app_error;
mod command;
mod command_class;
mod crypto;
mod handler;
mod tx_sign_state;
mod utilities;
mod ledger_display_io;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

// Application Name
const APPLICATION: &str = env!("CARGO_PKG_DESCRIPTION");

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = Comm::new();
    let mut state = TxSignState::new();

    loop {
        SingleMessage::new(APPLICATION).show();

        match comm.next_event() {
            // Press both buttons to exit app
            Event::Button(ButtonEvent::BothButtonsPress) => nanos_sdk::exit_app(0),

            Event::Command(ins) => match dispatcher::dispatcher(&mut comm, ins, &mut state) {
                Ok(()) => comm.reply_ok(),
                Err(app_error) => comm.reply(app_error),
            },
            _ => (),
        }
    }
}
