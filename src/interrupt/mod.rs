mod isr;
mod idt;

use lazy_static::lazy_static;
use spin::Mutex;
use crate::vga::{Color, ColorCode, print_str};

lazy_static! {
    pub static ref IDT:Mutex<[IDTGate; 256]> = Mutex::new([IDTGate::null(); 256]);
}

#[derive(Clone,Copy)]
#[repr(C)]
pub struct Registers {
    ds: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    int_no: u32,
    err_code: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    useresp: u32,
    ss: u32,
}

#[repr(C,packed)]
#[derive(Clone,Copy)]
pub struct IDTGate {
    low_offset: u16,
    selector: u16,
    always: u8,
    flags: u8,
    high_offset: u16
}

impl IDTGate {
    pub fn null() -> Self {
        Self {
            low_offset: 0,
            selector: 0,
            always: 0,
            flags: 0,
            high_offset: 0
        }
    }
}

fn set_idt_gate(n: i32,handler: u32) {
    IDT.lock()[n as usize].low_offset = (handler & 0xFFFF) as u16;
    IDT.lock()[n as usize].selector = 0x08;
    IDT.lock()[n as usize].always = 0;
    IDT.lock()[n as usize].flags = 0x8E;
    IDT.lock()[n as usize].high_offset = ((handler >> 16) & 0xFFFF) as u16;
}