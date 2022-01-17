#![no_std]
#![feature(start)]

use a800xl_utils::{
    clock, consts,
    screen::{clrscr, gotoxy, show_cursor},
    wait_vbl,
};
use core::panic::PanicInfo;
use ufmt_stdio::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    println!("Hello World!");
    show_cursor(false);
    clrscr();
    loop {
        let ch = unsafe { *consts::CH };
        gotoxy(2, 2);
        print!("clock: {}", clock());
        gotoxy(2, 4);
        print!("CH: {}  ", ch);
        gotoxy(2, 5);
        print!("SKSTAT: {}  ", unsafe { *consts::SKSTAT });
        gotoxy(2, 6);
        print!("CONSOL: {}  ", unsafe { *consts::CONSOL });
        if ch == 28 {
            clrscr();
            gotoxy(2, 0);
            break;
        }
    }
    0
}
