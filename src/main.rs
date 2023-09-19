#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(result_option_inspect)]
#![feature(asm_const)]
#![feature(cfg_version)]
#![feature(const_mut_refs)]

use nanos_sdk::io::{Comm, Event};
use nanos_ui::bagls::{CERTIFICATE_ICON, COGGLE_ICON, DASHBOARD_X_ICON, PROCESSING_ICON};
use nanos_ui::ui::clear_screen;

use handler::dispatcher;

use crate::app_error::AppError;
use crate::ledger_display_io::LedgerTTY;
use crate::settings::Settings;
use crate::sign::tx_state::TxState;
use crate::ui::menu::{Menu, MenuFeature, MenuItem};
use crate::ui::multipage_validator::MultipageValidator;
use crate::ui::single_message::SingleMessage;
use crate::ui::utils::{BACK_ICON, RADIX_LOGO_ICON};

mod app_error;
mod command;
mod command_class;
mod crypto;
mod handler;
mod ledger_display_io;
mod settings;
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

fn app_menu_action() -> bool {
    false
}

fn version_menu_action() -> bool {
    clear_screen();
    SingleMessage::new(APPLICATION_VERSION).show_and_wait();
    false
}

fn get_verbose_mode_state() -> bool {
    Settings::get().verbose_mode
}

fn get_blind_signing_state() -> bool {
    Settings::get().blind_signing
}

fn settings_menu_action() -> bool {
    clear_screen();

    let menu = [
        MenuItem::new(
            MenuFeature::OnOffState(get_verbose_mode_state),
            "\nVerbose Mode",
            verbose_mode_setting_action,
        ),
        MenuItem::new(
            MenuFeature::OnOffState(get_blind_signing_state),
            "\nBlind Signing",
            blind_signing_setting_action,
        ),
        MenuItem::new(
            MenuFeature::Icon(&BACK_ICON),
            "\nBack",
            back_from_setting_action,
        ),
    ];

    Menu::new(&menu).event_loop();

    false
}

fn verbose_mode_setting_action() -> bool {
    clear_screen();

    Settings {
        verbose_mode: MultipageValidator::new(
            &[&"Set Verbose", &"Mode"],
            &[&"Enable"],
            &[&"Disable"],
        )
        .ask(),
        blind_signing: get_blind_signing_state(),
    }
    .update();

    true
}

fn blind_signing_setting_action() -> bool {
    clear_screen();

    Settings {
        verbose_mode: get_verbose_mode_state(),
        blind_signing: MultipageValidator::new(
            &[&"Set Blind", &"Signing"],
            &[&"Enable"],
            &[&"Disable"],
        )
        .ask(),
    }
    .update();

    true
}

fn back_from_setting_action() -> bool {
    true
}

fn about_menu_action() -> bool {
    clear_screen();
    SingleMessage::new(APPLICATION_ABOUT).show_and_wait();
    false
}

fn quit_menu_action() -> bool {
    clear_screen();
    nanos_sdk::exit_app(0);
}

#[no_mangle]
extern "C" fn sample_main() {
    let menu = [
        MenuItem::new(
            MenuFeature::Icon(&RADIX_LOGO_ICON),
            "\nRadix Babylon",
            app_menu_action,
        ),
        MenuItem::new(
            MenuFeature::Icon(&PROCESSING_ICON),
            "\nVersion",
            version_menu_action,
        ),
        MenuItem::new(
            MenuFeature::Icon(&COGGLE_ICON),
            "\nSettings",
            settings_menu_action,
        ),
        MenuItem::new(
            MenuFeature::Icon(&CERTIFICATE_ICON),
            "\nAbout",
            about_menu_action,
        ),
        MenuItem::new(
            MenuFeature::Icon(&DASHBOARD_X_ICON),
            "\nQuit",
            quit_menu_action,
        ),
    ];
    let mut comm = Comm::new();
    let mut state = TxState::new(LedgerTTY::new_tty());
    let mut main_menu = Menu::new(&menu);
    let mut ticker = 0i8;

    nanos_ui::ui::popup("Pending Review");

    main_menu.display();

    loop {
        let event = comm.next_event();

        match event {
            Event::Button(button_event) => _ = main_menu.handle(button_event),
            Event::Command(ins) => {
                match dispatcher::dispatcher(&mut comm, ins, &mut state) {
                    Ok(()) => comm.reply_ok(),
                    Err(app_error) => comm.reply(app_error),
                };
                ticker = 5;
            }
            Event::Ticker => {
                if ticker >= 0 {
                    ticker -= 1;

                    if ticker == 0 {
                        main_menu.display();
                    }
                }
            }
        }
    }
}
