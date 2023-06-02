use core::arch::asm;
use core::mem::size_of_val;
use core::ptr::addr_of_mut;
use crate::interrupt::IDT;

#[repr(C,packed)]
pub struct IDTRegister {
    limit: u16,
    base: u16
}

pub fn load_idt() {
    let mut idt = IDTRegister {
        limit: &IDT as *const IDT as u16,
        base: (252 * size_of_val(&IDT) - 1) as u16,
    };

    unsafe {
        asm!("lidt {0}",in(reg) addr_of_mut!(idt));
    }
}