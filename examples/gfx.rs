#![no_std]
#![feature(start)]
#![feature(core_intrinsics)]

#[allow(unused_imports)]
use a800xl_utils;
use a800xl_utils::{
    cio::{Cmd, Mode, IOCB},
    consts,
    fs::File,
    get_soft_stack_ptr, random,
    screen::{close_graphics, draw_line, init_graphics, plot, set_color, set_pos, set_start_pos},
};

use core::panic::PanicInfo;
use ufmt_stdio::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    unsafe {
        core::intrinsics::abort();
    }
}

fn in_bounds(x: i16, y: i16) -> bool {
    x >= 0 && x < 320 && y > 0 && y < 192
}

// [int(math.sin(2 * math.pi * i / 256) * 128) for i in range(256)]
static sin_table: [u8; 256] = [
    0x00, 0x03, 0x06, 0x09, 0x0c, 0x0f, 0x12, 0x15, 0x18, 0x1b, 0x1e, 0x21, 0x24, 0x27, 0x2a, 0x2d,
    0x30, 0x33, 0x36, 0x39, 0x3b, 0x3e, 0x41, 0x43, 0x46, 0x49, 0x4b, 0x4e, 0x50, 0x52, 0x55, 0x57,
    0x59, 0x5b, 0x5e, 0x60, 0x62, 0x64, 0x66, 0x67, 0x69, 0x6b, 0x6c, 0x6e, 0x70, 0x71, 0x72, 0x74,
    0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7b, 0x7c, 0x7d, 0x7d, 0x7e, 0x7e, 0x7e, 0x7e, 0x7e,
    0x7f, 0x7e, 0x7e, 0x7e, 0x7e, 0x7e, 0x7d, 0x7d, 0x7c, 0x7b, 0x7b, 0x7a, 0x79, 0x78, 0x77, 0x76,
    0x75, 0x74, 0x72, 0x71, 0x70, 0x6e, 0x6c, 0x6b, 0x69, 0x67, 0x66, 0x64, 0x62, 0x60, 0x5e, 0x5b,
    0x59, 0x57, 0x55, 0x52, 0x50, 0x4e, 0x4b, 0x49, 0x46, 0x43, 0x41, 0x3e, 0x3b, 0x39, 0x36, 0x33,
    0x30, 0x2d, 0x2a, 0x27, 0x24, 0x21, 0x1e, 0x1b, 0x18, 0x15, 0x12, 0x0f, 0x0c, 0x09, 0x06, 0x03,
    0x00, 0xfd, 0xfa, 0xf7, 0xf4, 0xf1, 0xee, 0xeb, 0xe8, 0xe5, 0xe2, 0xdf, 0xdc, 0xd9, 0xd6, 0xd3,
    0xd0, 0xcd, 0xca, 0xc7, 0xc5, 0xc2, 0xbf, 0xbd, 0xba, 0xb7, 0xb5, 0xb2, 0xb0, 0xae, 0xab, 0xa9,
    0xa7, 0xa5, 0xa2, 0xa0, 0x9e, 0x9c, 0x9a, 0x99, 0x97, 0x95, 0x94, 0x92, 0x90, 0x8f, 0x8e, 0x8c,
    0x8b, 0x8a, 0x89, 0x88, 0x87, 0x86, 0x85, 0x85, 0x84, 0x83, 0x83, 0x82, 0x82, 0x82, 0x82, 0x82,
    0x81, 0x82, 0x82, 0x82, 0x82, 0x82, 0x83, 0x83, 0x84, 0x85, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a,
    0x8b, 0x8c, 0x8e, 0x8f, 0x90, 0x92, 0x94, 0x95, 0x97, 0x99, 0x9a, 0x9c, 0x9e, 0xa0, 0xa2, 0xa5,
    0xa7, 0xa9, 0xab, 0xae, 0xb0, 0xb2, 0xb5, 0xb7, 0xba, 0xbd, 0xbf, 0xc2, 0xc5, 0xc7, 0xca, 0xcd,
    0xd0, 0xd3, 0xd6, 0xd9, 0xdc, 0xdf, 0xe2, 0xe5, 0xe8, 0xeb, 0xee, 0xf1, 0xf4, 0xf7, 0xfa, 0xfd,
];

fn sin(angle: u8) -> i16 {
    sin_table[angle as usize] as i8 as i16
}

fn cos(angle: u8) -> i16 {
    sin(angle.wrapping_add(64))
}

fn draw_tree(x: i16, y: i16, height: i16, angle: u8, level: u8) {
    if level > 8 {
        return;
    }
    let dx = x + height * sin(angle) / 128;
    let dy = y + height * cos(angle) / 128;

    if in_bounds(x, 175 - y) && in_bounds(dx, 175 - dy) {
        draw_line(x, 175 - y, dx, 175 - dy);
    }

    draw_tree(
        dx,
        dy,
        height * (192 + (random() & 15)) as i16 / 256,
        angle.wrapping_add(10).wrapping_add(random() & 15),
        level + 1,
    );
    draw_tree(
        dx,
        dy,
        height * (192 + (random() & 15)) as i16 / 256,
        angle.wrapping_sub(10).wrapping_sub(random() & 15),
        level + 1,
    );
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // HACK: force screen memory below 0xa000
    let ram_top_saved = unsafe { core::mem::replace(&mut *consts::RAMTOP, 0xa0) };
    init_graphics(8, 4 + 8 + 16);

    set_color(1);
    draw_tree(160, 0, 45, 0, 0);

    close_graphics();

    // unsafe {
    //     *consts::RAMTOP = ram_top_saved;
    // }
    // init_graphics(0, 4 + 8 + 16);
    // close_graphics();
    0
}
