#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(dead_code)]

use core::ptr::copy;
use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_sdk::io::{Comm, Event};
use nanos_ui::ui;
use nanos_ui::ui::SingleMessage;

use utils::MODEL_DATA;
use utils::VERSION_DATA;

use crate::app_error::AppError;
use crate::bip32::Bip32Path;
use crate::key25519::Key25519;
use crate::sha256::Sha256;
use crate::utils::debug;

mod app_error;
mod bip32;
mod key25519;
mod sha256;
mod utils;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

// APDU Class
const RADIX_CLASS: u8 = 0xAA;
// Application Version Data
const APPLICATION: &str = env!("CARGO_PKG_DESCRIPTION");
// Device ID Derivation Path
const DEVICE_ID_DERIVATION_PATH: Bip32Path = Bip32Path::from(b"m/44'/1022'/365'");

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
    GetPublicKeyCurve25519,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            0x10 => Ins::GetApplicationVersion,
            0x11 => Ins::GetDeviceModel,
            0x12 => Ins::GetDeviceId,
            0x21 => Ins::GetPublicKeyCurve25519,
            0xff => Ins::Exit,
            _ => panic!(),
        }
    }
}

fn handle_apdu(comm: &mut Comm, ins: Ins) -> Result<(), AppError> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    if comm.get_cla_ins().0 != RADIX_CLASS {
        return Err(AppError::BadCla);
    }

    match ins {
        Ins::GetApplicationVersion => comm.append(&VERSION_DATA),
        Ins::GetDeviceModel => comm.append(&MODEL_DATA),
        Ins::GetDeviceId => handle_get_device_id(comm)?,
        Ins::GetPublicKeyCurve25519 => handle_get_public_key_curve25519(comm)?,
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}

fn handle_get_device_id(comm: &mut Comm) -> Result<(), AppError> {
    Key25519::derive(&DEVICE_ID_DERIVATION_PATH)
        .map(|key| Sha256::double(key.public()))
        .map(|hash| {
            comm.append(hash.hash());
            ()
        })
}

fn handle_get_public_key_curve25519(comm: &mut Comm) -> Result<(), AppError> {
    Bip32Path::read(comm)
        .and_then(|path| path.validate().map(|_| path))
        .and_then(|path| Key25519::derive(&path))
        .map(|key| {
            comm.append(key.public());
            ()
        })
}
