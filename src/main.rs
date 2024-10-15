#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(result_option_inspect)]
#![feature(asm_const)]
#![feature(cfg_version)]
#![feature(const_mut_refs)]
#![feature(core_intrinsics)]

use handler::dispatcher;

use crate::app_error::AppError;
use crate::io::{Comm, Event, UxEvent};
use crate::ledger_display_io::LedgerTTY;
use crate::sign::tx_state::TxState;

mod app_error;
mod command;
mod command_class;
mod crypto;
mod handler;
mod io;
mod ledger_display_io;
mod settings;
mod sign;
mod ui;
mod utilities;
mod xui;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = Comm::new();
    let mut state = TxState::new(LedgerTTY::new_tty());
    let mut main_menu = xui::menu::main_menu::create();
    let mut ticker = 0i8;

    core::intrinsics::black_box(&mut comm);

    #[cfg(not(target_os = "stax"))]
    main_menu.display();

    loop {
        let event = comm.next_event();

        match event {
            Event::Command(ins) => {
                // Prevent excessive optimization which causes stack overflow on Nano S
                core::intrinsics::black_box(ins);

                UxEvent::wakeup();
                // Prevent excessive optimization which causes stack overflow on Nano S
                match core::intrinsics::black_box(dispatcher::dispatcher(
                    &mut comm, ins, &mut state,
                )) {
                    Ok(()) => comm.reply_ok(),
                    Err(app_error) => comm.reply(app_error),
                };
                ticker = 5;

                // Prevent excessive optimization which causes stack overflow on Nano S
                core::intrinsics::black_box(ins);
            }
            Event::Ticker => {
                if ticker >= 0 {
                    ticker -= 1;

                    if ticker == 0 {
                        main_menu.display();
                    }
                }

                if UxEvent::enter_screen_lock(&mut comm) {
                    main_menu.display();
                }
            }
            #[cfg(not(target_os = "stax"))]
            Event::Button(button_event) => {
                UxEvent::wakeup();
                _ = main_menu.handle(button_event);
            }
            #[cfg(target_os = "stax")]
            _ => {}
        }
    }
}
