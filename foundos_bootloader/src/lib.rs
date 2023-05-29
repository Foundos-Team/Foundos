#![no_std]
#![no_main]

use crate::display::Display;

mod display;

#[no_mangle]
pub extern "C" fn BootMain() {
    Display::clear();
}