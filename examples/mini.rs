#![no_std]
#![feature(start)]
#[allow(unused_imports)]

use a800xl_utils;  // installs panic handler

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert!(false);
    0
}
