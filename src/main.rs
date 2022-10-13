#![no_std]
#![no_main]

mod app_errors;
mod pubkey;

//use core::convert::AsRef;
use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_sdk::io::Reply;
use nanos_ui::ui;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

// Application Version Data
const VERSION: &[u8] = env!("CARGO_PKG_VERSION").as_bytes();
//const APPLICATION: &str = env!("CARGO_PKG_DESCRIPTION");

#[cfg(target_os = "nanos")]
const APPLICATION: &str = "Nano S";
#[cfg(target_os = "nanosplus")]
const APPLICATION: &str = "Nano S+";
#[cfg(target_os = "nanox")]
const APPLICATION: &str = "Nano X";

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    loop {
        // Draw some 'welcome' screen
        ui::SingleMessage::new(APPLICATION).show();

        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            //TODO: remove handling here
            io::Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),
            io::Event::Command(ins) => match handle_apdu(&mut comm, ins) {
                Ok(()) => comm.reply_ok(),
                Err(sw) => comm.reply(sw),
            },
            _ => (),
        }
    }
}

#[repr(u8)]
enum Ins {
    GetDeviceInformation,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            0x10 => Ins::GetDeviceInformation,
            0xff => Ins::Exit,
            _ => panic!(),
        }
    }
}

fn handle_apdu(comm: &mut io::Comm, ins: Ins) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::GetDeviceInformation => {
            comm.append(VERSION);
        }
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}
