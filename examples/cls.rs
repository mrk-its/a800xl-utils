#![no_std]
#![feature(start)]
#![feature(core_intrinsics)]

use a800xl_utils::cio::{Cmd, IOCB};
use core::panic::PanicInfo;
use ufmt_stdio::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    #[allow(unused_unsafe)]
    unsafe {
        core::intrinsics::abort();
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    IOCB::new(0).cmd(Cmd::PutBytes as u8).buffer(b"\x7d").call();
    0
}
