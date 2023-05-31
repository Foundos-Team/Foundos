#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

static MES: &[u8] = b"X";

#[no_mangle]
pub extern "C" fn main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in MES.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

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
