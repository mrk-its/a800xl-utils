#![no_std]

use volatile_register::{RO, RW};
pub mod consts;
pub mod screen;

pub fn io_write<T: Copy>(addr: usize, value: T) {
    unsafe {
        (*(addr as *const RW<T>)).write(value);
    }
}

pub fn io_read<T: Copy>(addr: usize) -> T {
    unsafe { (*(addr as *const RO<T>)).read() }
}

pub fn clock() -> u32 {
    unsafe {
        let x = [
            *consts::RTCLOK.add(2),
            *consts::RTCLOK.add(1),
            *consts::RTCLOK,
            0,
        ];
        *(&x as *const u8 as *const u32)
    }
}

pub fn wait_vbl() {
    unsafe {
        let clkaddr = consts::RTCLOK.add(2) as usize;
        let v = io_read::<u8>(clkaddr);
        while io_read::<u8>(clkaddr) == v {}
    }
}
