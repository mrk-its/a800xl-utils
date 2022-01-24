#![no_std]

use volatile_register::{RO, RW};

pub mod cio;
pub mod consts;
pub mod fs;
pub mod screen;

pub fn volatile_write<T: Copy>(addr: *mut T, value: T) {
    unsafe {
        (*(addr as *const RW<T>)).write(value);
    }
}

pub fn volatile_read<T: Copy>(addr: *const T) -> T {
    unsafe { (*(addr as *const RO<T>)).read() }
}

/// returns value of RTCLOCK
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

/// wait for vblank
pub fn wait_vbl() {
    unsafe {
        let clkaddr = consts::RTCLOK.add(2);
        let v = volatile_read::<u8>(clkaddr);
        while volatile_read::<u8>(clkaddr) == v {}
    }
}

/// returns value of llvm-mos soft stack pointer
pub fn get_soft_stack_ptr() -> *const u8 {
    unsafe { *consts::LLVM_MOS_SOFT_STACK_PTR }
}

/// returns random byte from POKEY's RANDOM register
#[inline(always)]
pub fn random() -> u8 {
    volatile_read::<u8>(consts::RANDOM)
}
