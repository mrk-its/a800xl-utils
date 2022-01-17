#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use a800xl_utils::heap;
use alloc::{string::ToString, vec};
use ufmt_stdio::*;

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
            let data = vec![1u8, 2, 3];
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
