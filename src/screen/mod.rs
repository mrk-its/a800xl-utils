use crate::consts;
use ufmt_write::uWrite;

/// shows / hides text cursor
pub fn show_cursor(visible: bool) {
    unsafe {
        *crate::consts::CRSINH = !visible as u8;
    }
}

/// resets ATRACT (screen saver counter)
pub fn clear_atract() {
    unsafe { *consts::ATRACT = 0 }
}

/// moves text cursor to specified position
pub fn gotoxy(x: usize, y: usize) {
    unsafe {
        **consts::OLDADR = *consts::OLDCHR;
        *consts::OLDADR = (*consts::SAVMSC).add(40 * y + x);
        let c = **consts::OLDADR;
        *consts::OLDCHR = c;
        if *consts::CRSINH == 0 {
            **consts::OLDADR |= 0x80;
        }
        *consts::ROWCRS = y as u8;
        *consts::COLCRS = x as u16;
    }
}

/// clears screen (for now only text gr0 mode)
pub fn clrscr() {
    let scr_slice = unsafe {
        let scr_addr = *consts::SAVMSC;
        core::slice::from_raw_parts_mut(scr_addr, 40 * 24)
    };
    scr_slice.fill(0);
}

/// converts atascii code to screen code
pub fn atascii_to_screen(b: u8) -> u8 {
    (match b & 0x7f {
        0..=31 => b + 64,
        32..=95 => b - 32,
        _ => b,
    }) | (b & 128)
}

/// [ufmt::uWrite] implementation for writing directly to screen memory
/// (it converts atascii text to screen codes)
pub struct ScreenMemoryWriter<'a> {
    buffer: &'a mut [u8],
    written: usize,
}

impl<'a> ScreenMemoryWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> ScreenMemoryWriter<'a> {
        ScreenMemoryWriter { buffer, written: 0 }
    }
}

impl<'a> uWrite for ScreenMemoryWriter<'a> {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for b in s.bytes().map(atascii_to_screen) {
            if self.written >= self.buffer.len() {
                return Err(());
            }
            self.buffer[self.written] = b;
            self.written += 1;
        }
        Ok(())
    }
}
