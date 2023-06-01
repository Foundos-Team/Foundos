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

fn set_cursor(offset: i32) {
    let offset = offset / 2;
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH);
    port_byte_out(VGA_DATA_REGISTER, (offset >> 8) as u8);
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW);
    port_byte_out(VGA_DATA_REGISTER, (offset & 0xff) as u8);
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
    2 * (row * MAX_COLS + col)
}

fn move_offset_to_new_line(offset: i32) -> i32 {
    get_offset(0, get_row_from_offset(offset) + 1)
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
    set_cursor(get_offset(0,0));
}

fn memory_copy(source: *const i8, dest: *mut i8, nbytes: usize) {
    let source_slice = unsafe { core::slice::from_raw_parts(source, nbytes) };
    let dest_slice = unsafe { core::slice::from_raw_parts_mut(dest, nbytes) };

    for i in 0..nbytes {
        dest_slice[i] = source_slice[i];
    }
}

pub fn scroll_ln(offset: i32) -> i32 {
    memory_copy((get_offset(0,1)+VIDEO_ADDRESS) as *const i8, (get_offset(0,0) + VIDEO_ADDRESS) as *mut i8, (MAX_COLS * (MAX_ROWS - 1) * 2) as usize);

    for i in 0..MAX_COLS {
        let offset = get_offset(i, MAX_ROWS - 1);
        set_char_at_video_memory(' ',&offset);
    }

    offset - 2 * MAX_COLS
}

// int scroll_ln(int offset) {
// memory_copy(
// (char *) (get_offset(0, 1) + VIDEO_ADDRESS),
// (char *) (get_offset(0, 0) + VIDEO_ADDRESS),
// MAX_COLS * (MAX_ROWS - 1) * 2
// );
//
// for (int col = 0; col < MAX_COLS; col++) {
// set_char_at_video_memory(' ', get_offset(col, MAX_ROWS - 1));
// }
//
// return offset - 2 * MAX_COLS;
// }
