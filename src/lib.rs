#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use crate::vga::{clear_screen, print_str};

pub mod vga;
pub mod port;

static MES: &[u8] = b"FoundOS (version 0.1.0-0.0)";

#[no_mangle]
pub extern "C" fn main() -> ! {
    clear_screen();

    loop {
        unsafe {
            asm!("hlt")
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
