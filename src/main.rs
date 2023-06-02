#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use crate::vga::{clear_screen, Color, ColorCode, print_str};

pub mod vga;
pub mod port;
pub mod interrupt;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let color = ColorCode::new(Color::White,Color::Black);
    clear_screen();
    print_str("Hello World!\n",color);
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
