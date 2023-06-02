use core::ptr::{null, null_mut};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::interrupt::{Registers, set_idt_gate};
use crate::interrupt::idt::load_idt;
use crate::port::port_byte_out;
use crate::vga::{Color, ColorCode, print_str};

type IsrT = *const Registers;

#[derive(Copy,Clone)]
pub struct Isr(IsrT);

unsafe impl Send for Isr {

}

unsafe impl Sync for Isr {

}

lazy_static! {
    pub static ref INTERRUPT_HANDLERS:Mutex<[Isr; 256]> = Mutex::new([Isr(core::ptr::null()); 256]);
}

extern {
    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31();

    fn irq0();
    fn irq1();
    fn irq2();
    fn irq3();
    fn irq4();
    fn irq5();
    fn irq6();
    fn irq7();
    fn irq8();
    fn irq9();
    fn irq10();
    fn irq11();
    fn irq12();
    fn irq13();
    fn irq14();
    fn irq15();
}

static EXCEPTION_MESSAGES:[&str; 32] = [
    "Division By Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint",
    "Into Detected Overflow",
    "Out of Bounds",
    "Invalid Opcode",
    "No Coprocessor",

    "Double Fault",
    "Coprocessor Segment Overrun",
    "Bad TSS",
    "Segment Not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault",
    "Unknown Interrupt",

    "Coprocessor Fault",
    "Alignment Check",
    "Machine Check",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",

    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved"
];

fn isr_handler(r: *const Registers) {
    let color = ColorCode::new(Color::White,Color::Black);
    print_str(EXCEPTION_MESSAGES[unsafe { (*r).int_no } as usize],color);
}

fn isr_install() {
    set_idt_gate(0, isr0 as u32);
    set_idt_gate(1, isr1 as u32);
    set_idt_gate(2, isr2 as u32);
    set_idt_gate(3, isr3 as u32);
    set_idt_gate(4, isr4 as u32);
    set_idt_gate(5, isr5 as u32);
    set_idt_gate(6, isr6 as u32);
    set_idt_gate(7, isr7 as u32);
    set_idt_gate(8, isr8 as u32);
    set_idt_gate(9, isr9 as u32);
    set_idt_gate(10, isr10 as u32);
    set_idt_gate(11, isr11 as u32);
    set_idt_gate(12, isr12 as u32);
    set_idt_gate(13, isr13 as u32);
    set_idt_gate(14, isr14 as u32);
    set_idt_gate(15, isr15 as u32);
    set_idt_gate(16, isr16 as u32);
    set_idt_gate(17, isr17 as u32);
    set_idt_gate(18, isr18 as u32);
    set_idt_gate(19, isr19 as u32);
    set_idt_gate(20, isr20 as u32);
    set_idt_gate(21, isr21 as u32);
    set_idt_gate(22, isr22 as u32);
    set_idt_gate(23, isr23 as u32);
    set_idt_gate(24, isr24 as u32);
    set_idt_gate(25, isr25 as u32);
    set_idt_gate(26, isr26 as u32);
    set_idt_gate(27, isr27 as u32);
    set_idt_gate(28, isr28 as u32);
    set_idt_gate(29, isr29 as u32);
    set_idt_gate(30, isr30 as u32);
    set_idt_gate(31, isr31 as u32);

    // ICW1
    port_byte_out(0x20, 0x11);
    port_byte_out(0xA0, 0x11);

    // ICW2
    port_byte_out(0x21, 0x20);
    port_byte_out(0xA1, 0x28);

    // ICW3
    port_byte_out(0x21, 0x04);
    port_byte_out(0xA1, 0x02);

    // ICW4
    port_byte_out(0x21, 0x01);
    port_byte_out(0xA1, 0x01);

    // OCW1
    port_byte_out(0x21, 0x0);
    port_byte_out(0xA1, 0x0);

    set_idt_gate(32, irq0 as u32);
    set_idt_gate(33, irq1 as u32);
    set_idt_gate(34, irq2 as u32);
    set_idt_gate(35, irq3 as u32);
    set_idt_gate(36, irq4 as u32);
    set_idt_gate(37, irq5 as u32);
    set_idt_gate(38, irq6 as u32);
    set_idt_gate(39, irq7 as u32);
    set_idt_gate(40, irq8 as u32);
    set_idt_gate(41, irq9 as u32);
    set_idt_gate(42, irq10 as u32);
    set_idt_gate(43, irq11 as u32);
    set_idt_gate(44, irq12 as u32);
    set_idt_gate(45, irq13 as u32);
    set_idt_gate(46, irq14 as u32);
    set_idt_gate(47, irq15 as u32);

    load_idt();
}

fn irq_handler(r: *mut Registers) {
    unsafe {
        if (*INTERRUPT_HANDLERS).lock()[(*r).int_no as usize].0 != null() {
            let handler = (*INTERRUPT_HANDLERS).lock()[(*r).int_no as usize];
            //handler(r)
        }
    }

    port_byte_out(0x20, 0x20);
    if unsafe { (*r).int_no } < 40 {
        port_byte_out(0xA0,0x20);
    }
}
