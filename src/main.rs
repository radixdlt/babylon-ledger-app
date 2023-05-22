#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(result_option_inspect)]
#![feature(asm_const)]
#![feature(cfg_version)]
#![feature(const_mut_refs)]

use nanos_sdk::io::{Comm, Event};
use nanos_ui::bagls::{CERTIFICATE_ICON, DASHBOARD_X_ICON, PROCESSING_ICON};
use nanos_ui::ui::clear_screen;

use handler::dispatcher;

use crate::app_error::AppError;
use crate::sign::tx_state::TxState;
use crate::ui::menu::{Menu, MenuItem};
use crate::ui::single_message::SingleMessage;
use crate::ui::utils::RADIX_LOGO_ICON;

mod app_error;
mod command;
mod command_class;
mod crypto;
mod handler;
mod ledger_display_io;
mod sign;
mod ui;
mod utilities;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

const APPLICATION: &str = env!("CARGO_PKG_DESCRIPTION");
const APPLICATION_ABOUT: &str = concat!(
    env!("CARGO_PKG_DESCRIPTION"),
    "\n(c) 2022-23\nRDX Works Ltd."
);
const APPLICATION_VERSION: &str = concat!("\n", env!("CARGO_PKG_VERSION"), "\n",);

fn app_menu_action() {}

fn version_menu_action() {
    clear_screen();
    SingleMessage::new(APPLICATION_VERSION).show_and_wait();
}

fn about_menu_action() {
    clear_screen();
    SingleMessage::new(APPLICATION_ABOUT).show_and_wait();
}

fn quit_menu_action() {
    clear_screen();
    nanos_sdk::exit_app(0);
}

#[no_mangle]
extern "C" fn sample_main() {
    let menu = [
        MenuItem::new(&RADIX_LOGO_ICON, "\nRadix Babylon", app_menu_action),
        MenuItem::new(&PROCESSING_ICON, "\nVersion", version_menu_action),
        MenuItem::new(&CERTIFICATE_ICON, "\nAbout", about_menu_action),
        MenuItem::new(&DASHBOARD_X_ICON, "\nQuit", quit_menu_action),
    ];
    let mut comm = Comm::new();
    let mut state = TxState::new();
    let mut main_menu = Menu::new(&menu);
    let mut ticker = 0;

    main_menu.display();

    loop {
        let event = comm.next_event();

        match event {
            Event::Button(button_event) => main_menu.handle(button_event),
            Event::Command(ins) => {
                match dispatcher::dispatcher(&mut comm, ins, &mut state) {
                    Ok(()) => comm.reply_ok(),
                    Err(app_error) => comm.reply(app_error),
                };
                ticker = 0;
            }
            Event::Ticker => {
                if ticker >= 5 {
                    ticker = 0;
                    main_menu.display();
                } else {
                    ticker += 1;
                }
                main_menu.display()
            }
        }
    }
}
