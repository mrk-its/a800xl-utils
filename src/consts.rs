pub const RTCLOK: *mut u8 = 0x12 as *mut u8;
pub const ATRACT: *mut u8 = 0x4d as *mut u8;

pub const LMARGN: *mut u8 = 0x52 as *mut u8;
pub const RMARGN: *mut u8 = 0x53 as *mut u8;
pub const ROWCRS: *mut u8 = 0x54 as *mut u8;
pub const COLCRS: *mut u16 = 0x55 as *mut u16;
pub const SAVMSC: *mut *mut u8 = 0x58 as *mut *mut u8;
pub const OLDCHR: *mut u8 = 0x5d as *mut u8;
pub const OLDADR: *mut *mut u8 = 0x5e as *mut *mut u8;

pub const SDMCTL: *mut u8 = 0x22f as *mut u8;
pub const SDLST: *mut *mut u8 = 0x230 as *mut *mut u8;
pub const SDLSTL: *mut u8 = 0x230 as *mut u8;
pub const SDLSTH: *mut u8 = 0x231 as *mut u8;

pub const CRSINH: *mut u8 = 0x2f0 as *mut u8;

pub const PCOLR0: *mut u8 = 0x2c0 as *mut u8;
pub const PCOLR1: *mut u8 = 0x2c1 as *mut u8;
pub const PCOLR2: *mut u8 = 0x2c2 as *mut u8;
pub const PCOLR3: *mut u8 = 0x2c3 as *mut u8;

pub const COLOR0: *mut u8 = 0x2c4 as *mut u8;
pub const COLOR1: *mut u8 = 0x2c5 as *mut u8;
pub const COLOR2: *mut u8 = 0x2c6 as *mut u8;
pub const COLOR3: *mut u8 = 0x2c7 as *mut u8;
pub const COLOR4: *mut u8 = 0x2c8 as *mut u8;

pub const KEYDEL: *mut u8 = 0x2f1 as *mut u8;
pub const CH: *mut u8 = 0x2fc as *mut u8;
pub const CH1: *mut u8 = 0x2f2 as *mut u8;
pub const SHFLOK: *mut u8 = 0x2be as *mut u8;
pub const NOCLIK: *mut u8 = 0x2db as *mut u8;

pub const CONSOL: *mut u8 = 0xd01f as *mut u8;
pub const SKSTAT: *mut u8 = 0xd20f as *mut u8;
