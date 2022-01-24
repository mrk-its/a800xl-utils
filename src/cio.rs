pub const IOCB_PTR: *mut _IOCB = 0x340 as *mut _IOCB;

/// Possible IOCB commands
#[repr(u8)]
pub enum Cmd {
    Open = 0x3,
    GetRecord = 0x5,
    GetBytes = 0x7,
    PutRecord = 0x9,
    PutBytes = 0xb,
    Close = 0xc,
    Status = 0xd,
}

/// Possible channel opening modes
#[repr(u8)]
pub enum Mode {
    Read = 4,
    Write = 8,
    ReadWrite = 4 + 8,
    Append = 9,
}

extern "C" {
    fn __cio(a: u8, x: u8) -> u8;
}

#[repr(C)]
pub struct _IOCB {
    icchid: u8,
    icdno: u8,
    iccmd: u8,
    icstat: u8,
    icbufa: *const u8,
    icputb: u16,
    icbufl: usize,
    icax: [u8; 6],
}

/// helper for constructing single IOCB request using builder pattern
pub struct IOCB {
    channel: u8,
}

impl IOCB {
    #[inline(always)]
    pub fn _iocb(&self) -> &'static mut _IOCB {
        unsafe { &mut *IOCB_PTR.add(self.channel as usize) }
    }

    pub fn new(channel: u8) -> Self {
        Self { channel: channel }
    }

    pub fn cmd(&mut self, cmd: u8) -> &mut Self {
        self._iocb().iccmd = cmd;
        self
    }

    pub fn buf_ptr(&mut self, buf: *const u8) -> &mut Self {
        self._iocb().icbufa = buf;
        self
    }

    pub fn buf_len(&mut self, len: usize) -> &mut Self {
        self._iocb().icbufl = len;
        self
    }
    pub fn get_buf_len(&mut self) -> usize {
        self._iocb().icbufl
    }

    pub fn buffer(&mut self, buf: &[u8]) -> &mut Self {
        self.buf_ptr(buf.as_ptr()).buf_len(buf.len())
    }

    pub fn icax(&mut self, n: usize, val: u8) -> &mut Self {
        self._iocb().icax[n] = val;
        self
    }

    pub fn icax1(&mut self, val: u8) -> &mut Self {
        self.icax(0, val)
    }

    pub fn icax2(&mut self, val: u8) -> &mut Self {
        self.icax(1, val)
    }

    pub fn icax3(&mut self, val: u8) -> &mut Self {
        self.icax(2, val)
    }

    pub fn icax4(&mut self, val: u8) -> &mut Self {
        self.icax(3, val)
    }

    pub fn icax5(&mut self, val: u8) -> &mut Self {
        self.icax(4, val)
    }

    pub fn icax6(&mut self, val: u8) -> &mut Self {
        self.icax(5, val)
    }

    pub fn call_with_a(&mut self, a: u8) -> u8 {
        unsafe { __cio(a, self.channel << 4) }
    }

    pub fn call(&mut self) -> u8 {
        self.call_with_a(0)
    }
}

/// returns first available (closed) CIO channel
pub fn find_available_channel() -> Result<u8, u8> {
    for channel in 0..8 {
        let iocb = unsafe { &*IOCB_PTR.add(channel as usize) };
        if iocb.icchid == 0xff {
            return Ok(channel);
        }
    }
    Err(255)
}
