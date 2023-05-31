use core::arch::asm;

pub fn port_byte_in(port: u16) -> u8 {
    let mut result: u8 = 0;
    unsafe {
        asm!(
        "in al, dx",
        lateout("al") result,
        in("dx") port
        );
    }
    result
}

pub fn port_byte_out(port: u16, data: u8) {
    unsafe {
        asm!(
        "out dx, al",
        in("dx") port,
        in("al") data
        );
    }
}