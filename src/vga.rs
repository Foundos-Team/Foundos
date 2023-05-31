use core::arch::asm;
use core::ffi::CStr;
use crate::port::{port_byte_in, port_byte_out};

const VGA_CTRL_REGISTER:u16 = 0x3d4;
const VGA_DATA_REGISTER:u16 = 0x3d5;
const VGA_OFFSET_LOW:u16 = 0x0f;
const VGA_OFFSET_HIGH:u16 = 0x0e;

const VIDEO_ADDRESS:u32 = 0xb8000;
const MAX_ROWS:i32 = 25;
const MAX_COLS:i32 = 80;
const WHITE_ON_BLACK:u8 = 0x0f;

fn set_cursor(offset: i32) {
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH as u8);
    port_byte_out(VGA_CTRL_REGISTER, ((offset.clone() / 2) >> 8) as u8);
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW as u8);
    port_byte_out(VGA_CTRL_REGISTER, ((offset / 2) & 0xff) as u8);
}

fn get_cursor() -> i32 {
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH as u8);
    #[allow(arithmetic_overflow)]
    let mut offset = port_byte_in(VGA_DATA_REGISTER) << 8;
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW as u8);
    offset += port_byte_in(VGA_DATA_REGISTER);
    (offset * 2) as i32
}

fn get_row_from_offset(offset: i32) -> i32 {
    offset / (2 * MAX_COLS)
}

fn get_offset(col: i32,row: i32) -> i32 {
    2 * (row * MAX_COLS * col)
}

fn move_offset_to_new_line(offset: i32) -> i32 {
    get_offset(0,get_row_from_offset(offset) + 1)
}

fn set_char_at_video_memory(character: char, offset: &i32) {
    let vidmem = 0xb8000 as *mut u8;
    unsafe {
        *vidmem.offset(offset.clone() as isize) = character as u8;
        *vidmem.offset((offset + 1) as isize) = WHITE_ON_BLACK;
    }
}

pub fn print_str(text: &str) {
    let mut offset = get_cursor();
    let bytes = text.as_bytes();
    for i in bytes {
        match *i as char {
            '\n' => {
                offset = move_offset_to_new_line(offset);
            }
            _ => {
                set_char_at_video_memory(*i as char,&offset);
                offset += 2;
            }
        }
    }
    set_cursor(offset);
}

pub fn clear_screen() {
    for i in 0..(MAX_COLS*MAX_ROWS) {
        set_char_at_video_memory(' ', &(i * 2));
    }
    set_cursor(get_offset(0,0));
}
