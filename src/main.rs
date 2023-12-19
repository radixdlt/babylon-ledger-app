#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(result_option_inspect)]
#![feature(asm_const)]
#![feature(cfg_version)]
#![feature(const_mut_refs)]

use crate::app_error::AppError;

#[cfg(not(target_os = "stax"))]
use crate::other_main::app_main;

#[cfg(target_os = "stax")]
use crate::stax_main::app_main;

mod app_error;
mod command;
mod command_class;
mod crypto;
mod handler;
#[cfg(not(target_os = "stax"))]
mod io;
#[cfg(not(target_os = "stax"))]
mod ledger_display_io;
mod settings;
mod sign;
#[cfg(not(target_os = "stax"))]
mod ui;
mod utilities;

#[cfg(target_os = "stax")]
mod stax_main;
#[cfg(not(target_os = "stax"))]
mod other_main;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_main() {
    app_main();
}
