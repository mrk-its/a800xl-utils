#![no_std]
#![feature(start)]
#![feature(core_intrinsics)]

use a800xl_utils::cio::{Cmd, IOCB};
use a800xl_utils::{consts, volatile_read, random};
use a800xl_utils::screen::show_cursor;
use core::panic::PanicInfo;
use ufmt_stdio::*;

const HEADER: &[u8] = b"\x11\x12\x12\x12\x12\x17\x12\x12\x12\x12\x17\x12\x12\x12\x12\x17\x12\x12\x12\x12\x05\x9b";
const MID_LINE: &[u8] = b"\x01\x12\x12\x12\x12\x13\x12\x12\x12\x12\x13\x12\x12\x12\x12\x13\x12\x12\x12\x12\x04\x9b";
// const MID: &[u8] = b"|    |    |    |    |\x9b";
const FOOTER: &[u8] = b"\x1a\x12\x12\x12\x12\x18\x12\x12\x12\x12\x18\x12\x12\x12\x12\x18\x12\x12\x12\x12\x03\x9b";

const NRS: [[u8; 4]; 12] = [
    *b"    ",
    *b"   2",
    *b"   4",
    *b"   8",
    *b"  16",
    *b"  32",
    *b"  64",
    *b" 128",
    *b" 256",
    *b" 512",
    *b"1024",
    *b"2048",
];

struct Board {
    score: u32,
    occupied: u8,
    pub data: [[u8; 4]; 4],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            score: 0,
            occupied: 0,
            data:  Default::default(),
        }
    }
}

fn write_line(text: &[u8]) {
    IOCB::new(0).cmd(Cmd::PutBytes as u8).buffer(text).call();
}

fn getch() -> u8 {
    unsafe {
        while (volatile_read(consts::SKSTAT) & 4) > 0 {}
        while (volatile_read(consts::SKSTAT) & 4) == 0 {}
        return *consts::CH
    }
}


impl Board {
    fn show(&self, redraw: bool) {
        let mut line: [u8; 22] = [0; 22];
        if redraw {
            write_line(b"\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c\x1c");
        }
        if !redraw {
            write_line(HEADER);
        }
        for j in 0..4 {
            line[0] = '|' as u8;
            line[21] = 0x9b;
            for i in 0..4 {
                let mut v = self.data[j][i];
                let inv = v & 0x80;
                v = v & 0x7f;
                let v = NRS[v as usize];
                for k in 0..4 {
                    line[1 + i * 5 + k] = v[k] | inv;
                }
                line[1 + i * 5 + 4] = '|' as u8;
            }
            write_line(&line);
            if !redraw {
                write_line(if j < 3  {MID_LINE} else {FOOTER});
            } else {
                write_line(b"\x1d")
            }
        }
        println!("score: {}", self.score);
    }
    fn put_random(&mut self) {
        let (i, j) = loop {
            let i = random() as usize & 3;
            let j = random() as usize & 3;
            if self.data[j][i] == 0 {
                break (i, j)
            }
        };
        self.data[j][i] = 1;
        self.occupied += 1;
    }
    fn clean_fold_marks(&mut self) {
        for j in 0..4 {
            for i in 0..4 {
                self.data[j][i] &= 0x7f;
            }
        }
    }
    fn mv(&mut self, kx: i8, ky: i8) {
        let abs_kx = kx.abs() as usize;
        let abs_ky = ky.abs() as usize;

        loop {
            let mut moved = false;
            for j in abs_ky..4 {
                for i in abs_kx..4 {
                    let (i, prev_i) = if kx < 0 {
                        (i, i - abs_kx)
                    } else {
                        (3 - i, 3 - (i - abs_kx))
                    };

                    let (j, prev_j) = if ky < 0 {
                        (j, j - abs_ky)
                    } else {
                        (3 - j, 3 - (j - abs_ky))
                    };
                    let v = self.data[j][i];
                    if v == 0 {
                        continue
                    }

                    let prev_v = self.data[prev_j][prev_i];

                    if prev_v == 0 {
                        self.data[prev_j][prev_i] = v;
                        self.data[j][i] = 0;
                        moved = true;
                    } else if prev_v == v && ((v | prev_v) & 0x80) == 0 && v < 11 {
                        self.score += 2 << v;
                        self.data[prev_j][prev_i] += 1;
                        self.data[prev_j][prev_i] |= 0x80;
                        self.data[j][i] = 0;
                        self.occupied -= 1;
                        moved = true;
                    }
                }
            }
            if !moved {
                break
            }
        }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    #[allow(unused_unsafe)]
    unsafe {
        core::intrinsics::abort();
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut board = Board::default();

    board.put_random();
    board.put_random();

    show_cursor(false);
    board.show(false);
    loop {
        // board.put_random();
        match getch() {
            134 => board.mv(-1, 0),
            135 => board.mv(1, 0),
            142 => board.mv(0, -1),
            143 => board.mv(0, 1),
            28 => break,
            _ => (),
        }
        board.show(true);
        board.put_random();
        board.clean_fold_marks();
        board.show(true);
        if board.occupied == 16 {
            break;
        }
    }
    show_cursor(true);
    0
}
