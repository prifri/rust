#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]

use core::panic::PanicInfo;
use core::fmt;
use core::fmt::Write;

use x86_64::instructions::hlt;

#[allow(unused)]
#[derive(Clone,Copy)]
#[repr(u8)]
enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    White = 0xf,
    Brightblue = 0x9,
    BrightGreen = 0xa,
    BrightCyan = 0xb,
    BrightRed = 0xc,
    BrightMagenta = 0xd,
    Yello = 0xe,
    DarkGray = 0x8,
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    fn print(&mut self, text: &[u8]) {
        let color = self.color();

        let framebuffer = 0xb8000 as *mut u8;

        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1 ).write_volatile(color);
            }
            self.position += 2;
        }
    }
}

impl fmt::Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
/*
 * prifri, 2022.11.04:
 * - utf8로 인코딩된 &str을 &[u8]로 변환
 */
        self.print(s.as_bytes());
        Ok(())
    }
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    let mut cursor = Cursor {
        position: 0,
        foreground: Color::White,
        background: Color::Red,
    };

    for _ in 0..(80 * 25) {
        cursor.print(b" ");
    }

    cursor.position = 0;
    write!(cursor, "{}", info).unwrap();

    loop {
        hlt();
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("help!");
}