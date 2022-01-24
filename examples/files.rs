#![no_std]
#![feature(start)]
#![feature(core_intrinsics)]

#[allow(unused_imports)]
use a800xl_utils;
use a800xl_utils::{
    cio::{Cmd, Mode, IOCB},
    fs::File,
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

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut file = match File::create(b"D:TEST.TXT\x9b") {
        Err(err) => {
            println!("can't open: {}", err);
            return 1;
        }
        Ok(file) => file,
    };
    println!("opened channel {} to write", file.channel);
    for i in 0..3 {
        if let Err(err) = file.write(b"ok!\x9b") {
            println!("write error: {}", err);
            return 1;
        }
    }
    file.close();
    println!("written");

    if let Ok(mut file) = File::open(b"D:TEST.TXT\x9b") {
        println!("opened channel {} to read", file.channel);
        let mut buf: [u8; 32] = [0; 32];
        match file.read(&mut buf) {
            Err(err) => {
                println!("read error: {}", err);
                return 1;
            }
            Ok(len) => {
                println!("read {} bytes", len);
                // write buffer to 0 channel (E:)
                IOCB::new(0)
                    .cmd(Cmd::PutBytes as u8)
                    .buffer(&buf[0..len])
                    .call();
            }
        }
    }
    0
}
