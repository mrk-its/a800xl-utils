use ufmt_write::uWrite;
use crate::consts;

pub fn hide_cursor(hide: bool) {
    unsafe {
        *crate::consts::CRSINH = hide as u8;
    }
}

pub fn clear_atract() {
    unsafe {*consts::ATRACT = 0}
}

pub fn gotoxy(x: usize, y: usize) {
    // save x y in COLCRS (2 bytes), ROWCRS (1 byte)
    // call setcursor
    // restore char under old cursor *OLDADR = OLDCHR
    // OLDADDR = 40 * ROWCRS + COLCRS
    // store char under a cursor OLDCHR = *OLDADDR
    // update inverse bit of char under cursor according to CRSINH

    unsafe {
        *crate::consts::ROWCRS = y as u8;
        *crate::consts::COLCRS = x as u16;
    }

    // *COLCRS = x as u8;
    // *ROWCRS = y;

    // write_io(*OLDADDR, *OLDCHR);

    // *OLDADDR = 40 * y + x;


}

pub fn clrscr() {
    let scr_slice = unsafe {
        let scr_addr: u16 = *consts::SAVMSC;
        core::slice::from_raw_parts_mut(scr_addr as *mut u8, 40 * 24)
    };
    scr_slice.fill(0);
}

pub fn atascii_to_screen(b: u8) -> u8 {
    (match b & 0x7f {
        0..=31 => b + 64,
        32..=95 => b - 32,
        _ => b,
    }) | (b & 128)
}

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
                return Err(())
            }
            self.buffer[self.written] = b;
            self.written += 1;
        }
        Ok(())
    }
}
