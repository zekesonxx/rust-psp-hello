#![no_std]
#![no_main]
#![no_core]
#![feature(intrinsics, lang_items, linkage, macro_rules, no_core, improper_ctypes)]
#![allow(dead_code)]

use ctrl::Button;

mod libc;
mod lang;
#[macro_use]
mod raw;
mod utils;
mod display;
mod ctrl;

const VERSION_MAJOR: u8 = 0;
const VERSION_MINOR: u8 = 1;

PSP_MODULE_INFO!(raw::Mode::USER, VERSION_MAJOR, VERSION_MINOR);

#[no_mangle]
pub extern "C" fn main() {
    utils::debug_init();
    utils::debug_print("Hello World! from Rust with love <3\n");

    let mut pad_data = ctrl::Input::new();

    loop {
        //display::wait_vblank_start();

        if pad_data.read_changed() {
            match pad_data.pressed_key() {
                Button::Cross => utils::debug_print("X pressed\n"),
                Button::Circle => utils::debug_print("O pressed\n"),
                Button::Triangle => utils::debug_print("TRIANGLE pressed\n"),
                Button::Square => utils::debug_print("SQUARE pressed\n"),
                Button::Up => utils::debug_print("UP pressed\n"),
                Button::Down => utils::debug_print("DOWN pressed\n"),
                Button::Left => utils::debug_print("LEFT pressed\n"),
                Button::Right => utils::debug_print("RIGHT pressed\n"),
                Button::None => (),
                _ => utils::debug_print("unhandled keypress\n"),
            }
        }
        // ...
    }
}
