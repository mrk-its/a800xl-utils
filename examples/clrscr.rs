#![no_std]
#![feature(start)]

use core::panic::PanicInfo;
use ufmt_stdio::*;
use a800xl_utils::{screen::{clrscr, hide_cursor, gotoxy}, clock};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    println!("hello world!");
    hide_cursor(true);
    clrscr();
    loop {
        gotoxy(2, 2);
        println!("clock: {}", clock());
    }
    0
}
