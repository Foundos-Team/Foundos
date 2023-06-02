use core::arch::asm;
use core::ffi::CStr;
use core::ptr;
use crate::port::{port_byte_in, port_byte_out};

const VGA_CTRL_REGISTER:u16 = 0x3d4;
const VGA_DATA_REGISTER:u16 = 0x3d5;
const VGA_OFFSET_LOW:u8 = 0x0f;
const VGA_OFFSET_HIGH:u8 = 0x0e;

const VIDEO_ADDRESS:i32 = 0xb8000;
const MAX_ROWS:i32 = 25;
const MAX_COLS:i32 = 80;
const WHITE_ON_BLACK:u8 = 0x0f;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone,Copy)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

fn set_cursor(offset: i32) {
    let offset = offset / 2;
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH);
    port_byte_out(VGA_DATA_REGISTER, (offset >> 8) as u8);
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW);
    port_byte_out(VGA_DATA_REGISTER, (offset & 0xff) as u8);
}

fn get_cursor() -> i32 {
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH);
    #[allow(arithmetic_overflow)]
    let mut offset = port_byte_in(VGA_DATA_REGISTER) << 8;
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW);
    offset += port_byte_in(VGA_DATA_REGISTER);
    (offset * 2) as i32
}

fn get_row_from_offset(offset: i32) -> i32 {
    offset / (2 * MAX_COLS)
}

fn get_offset(col: i32,row: i32) -> i32 {
    2 * (row * MAX_COLS + col)
}

fn move_offset_to_new_line(offset: i32) -> i32 {
    get_offset(0, get_row_from_offset(offset) + 1)
}

fn set_char_at_video_memory(character: char, offset: &i32,color_code: u8) {
    let vidmem = 0xb8000 as *mut u8;
    unsafe {
        *vidmem.offset(offset.clone() as isize) = character as u8;
        *vidmem.offset((offset + 1) as isize) = color_code;
    }
}

pub fn print_str(text: &str,code: ColorCode) {
    let mut offset = get_cursor();
    let bytes = text.as_bytes();
    for i in bytes {
        let character = *i as char;
        if offset >= MAX_ROWS * MAX_COLS * 2 {
           //offset = scroll_ln(offset);
        }
        if character == '\n' {
            offset = move_offset_to_new_line(offset);
        } else {
            set_char_at_video_memory(*i as char,&offset,code.0);
            offset += 2;
        }
    }
    set_cursor(offset);
}

pub fn _print_str(text: char,offset: &mut i32,code: ColorCode) {
    if *offset >= MAX_ROWS * MAX_COLS * 2 {
        *offset = scroll_ln(*offset);
    }
    if text == '\n' {
        *offset = move_offset_to_new_line(*offset);
    } else {
        set_char_at_video_memory(text,&offset,code.0);
        *offset += 2;
    }
}

pub fn clear_screen() {
    for i in 0..(MAX_COLS*MAX_ROWS) {
        set_char_at_video_memory(' ', &(i * 2),0x0f);
    }
    set_cursor(get_offset(0,0));
    set_cursor(get_offset(0,0));
}

fn memory_copy(source: *const u8, dest: *mut u8, nbytes: usize) {
    unsafe {
        for i in 0..nbytes {
            ptr::write_volatile(dest.offset(i as isize), ptr::read_volatile(source.offset(i as isize)));
        }
    }
}

fn scroll_ln(offset: i32) -> i32 {
    unsafe {
        memory_copy(
            (get_offset(0, 1) + VIDEO_ADDRESS) as *const u8,
            (get_offset(0, 0) + VIDEO_ADDRESS) as *mut u8,
            (MAX_COLS * (MAX_ROWS - 1) * 2) as usize
        );
    }

    let color = ColorCode::new(Color::Black,Color::Black);

    for col in 0..MAX_COLS {
        set_char_at_video_memory(' ', &get_offset(col, MAX_ROWS - 1),color.0);
    }

    return offset - 2 * MAX_COLS;
}