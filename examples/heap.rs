#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use a800xl_utils::heap;
use alloc::{string::ToString, vec, vec::Vec};
use ufmt_stdio::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!(
        "heap bytes free: {} / {}",
        heap::bytes_free(),
        heap::get_limit(),
    );

    {
        let text = "foo".to_string();
        println!(
            "allocated string {}, free: {}",
            &text[..],
            heap::bytes_free()
        );
        {
            let data = (0..50u16).collect::<Vec<_>>();
            println!(
                "allocated vec: {:?}, free: {}",
                &data[..],
                heap::bytes_free()
            );
        }
    }
    println!("deallocated, free: {}", heap::bytes_free());
    loop {}
}
