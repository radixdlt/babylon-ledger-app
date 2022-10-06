#![no_std]
#![no_main]

use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_ui::ui;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

// Application Version Data
const VERSION: &[u8] = env!("CARGO_PKG_VERSION").as_bytes();
const APPLICATION: &str = env!("CARGO_PKG_DESCRIPTION");

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    loop {
        // Draw some 'welcome' screen
        ui::SingleMessage::new(APPLICATION).show();

        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
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
    GetAppVersion,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            0x10 => Ins::GetAppVersion,
            0xff => Ins::Exit,
            _ => panic!(),
        }
    }
}

use nanos_sdk::io::Reply;

fn handle_apdu(comm: &mut io::Comm, ins: Ins) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::GetAppVersion => {
            comm.append(VERSION);
        }
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}
