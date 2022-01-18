#![no_std]
#![feature(start)]
#[allow(unused_imports)]

use a800xl_utils;  // installs panic handler
use ufmt_stdio::println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert!(false);
    0
}
