#![no_std]
#![feature(start)]

use a800xl_utils::{
    clock, consts,
    screen::{clrscr, gotoxy, show_cursor},
};
use ufmt_stdio::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    show_cursor(false);
    clrscr();
    gotoxy(2, 0);
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
