#![no_std]
#![no_main]
#![feature(intrinsics, lang_items, linkage, macro_rules)]
#![allow(dead_code)]

mod libc;
mod lang;
#[macro_escape]
mod raw;
mod utils;
mod display;
mod ctrl;

const VERSION_MAJOR: u8 = 0;
const VERSION_MINOR: u8 = 1;

PSP_MODULE_INFO!(raw::Mode::USER, VERSION_MAJOR, VERSION_MINOR)

#[no_mangle]
pub extern "C" fn main() {
    utils::debug_init();
    utils::debug_print("Hello World! from Rust with love <3\n");

    let mut pad_data = ctrl::SceCtrlData::new();
    let mut last_pad : u32 = 0;

    loop {
        //display::wait_vblank_start();

        ctrl::read(&mut pad_data);

        if last_pad != pad_data.buttons {
            last_pad = pad_data.buttons;

            if ctrl::button_pressed(&pad_data, ctrl::Button::PSP_CTRL_CROSS) {
                utils::debug_print("X pressed\n");
            } else if ctrl::button_pressed(&pad_data, ctrl::Button::PSP_CTRL_CIRCLE) {
                utils::debug_print("O pressed\n");
            } else if ctrl::button_pressed(&pad_data, ctrl::Button::PSP_CTRL_TRIANGLE) {
                utils::debug_print("TRIANGLE pressed\n");
            } else if ctrl::button_pressed(&pad_data, ctrl::Button::PSP_CTRL_SQUARE) {
                utils::debug_print("SQUARE pressed\n");
            } else if ctrl::button_pressed(&pad_data, ctrl::Button::PSP_CTRL_UP) {
                utils::debug_print("UP pressed\n");
            } else if ctrl::button_pressed(&pad_data, ctrl::Button::PSP_CTRL_DOWN) {
                utils::debug_print("DOWN pressed\n");
            } else if ctrl::button_pressed(&pad_data, ctrl::Button::PSP_CTRL_LEFT) {
                utils::debug_print("LEFT pressed\n");
            } else if ctrl::button_pressed(&pad_data, ctrl::Button::PSP_CTRL_RIGHT) {
                utils::debug_print("RIGHT pressed\n");
            }
        }
        // ...
    }
}
