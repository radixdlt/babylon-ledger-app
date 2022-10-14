#![no_std]
#![no_main]

use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_sdk::io::{Comm, Event};
use nanos_ui::ui::SingleMessage;

use crate::app_errors::AppErrors;
use crate::pubkey::{derive_curve25519, double_sha256, to_bip32_path, Bip32Path};
use utils::MODEL_DATA;
use utils::VERSION_DATA;

mod app_errors;
mod pubkey;
mod utils;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

// Application Version Data
const APPLICATION: &str = env!("CARGO_PKG_DESCRIPTION");
// Device ID Derivation Path
const DEVICE_ID_DERIVATION_PATH: Bip32Path = to_bip32_path(b"m/44'/1022'/365'");

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = Comm::new();

    loop {
        // Draw some 'welcome' screen
        SingleMessage::new(APPLICATION).show();

        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            //TODO: remove handling here
            Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),
            Event::Command(ins) => match handle_apdu(&mut comm, ins) {
                Ok(()) => comm.reply_ok(),
                Err(app_error) => comm.reply(app_error),
            },
            _ => (),
        }
    }
}

#[repr(u8)]
enum Ins {
    GetApplicationVersion,
    GetDeviceModel,
    GetDeviceId,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            0x10 => Ins::GetApplicationVersion,
            0x11 => Ins::GetDeviceModel,
            0x12 => Ins::GetDeviceId,
            0xff => Ins::Exit,
            _ => panic!(),
        }
    }
}

fn handle_apdu(comm: &mut Comm, ins: Ins) -> Result<(), AppErrors> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::GetApplicationVersion => comm.append(&VERSION_DATA),
        Ins::GetDeviceModel => comm.append(&MODEL_DATA),
        Ins::GetDeviceId => handle_get_device_id(comm)?,

        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}

fn handle_get_device_id(comm: &mut Comm) -> Result<(), AppErrors> {
    let pub_key = derive_curve25519(&DEVICE_ID_DERIVATION_PATH)?;
    let hash = double_sha256(&pub_key.key[0..32]);

    comm.append(&hash.buffer);

    Ok(())
}
