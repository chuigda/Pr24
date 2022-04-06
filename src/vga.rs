#![allow(dead_code)]

use core::fmt::{Arguments, Write};
use spin::Mutex;
use volatile::Volatile;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(fore: Color, back: Color) -> ColorCode {
        ColorCode((back as u8) << 4 | (fore as u8))
    }

    pub const fn with_blink(fore: Color, back: Color, blink: bool) -> ColorCode {
        let mut code: ColorCode = ColorCode::new(fore, back);
        if blink {
            code.0 |= 0x80;
        }
        code
    }
}

impl Into<u8> for ColorCode {
    fn into(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Kharakter {
    kharakter: u8,
    color: ColorCode
}

const VGA_HEIGHT: usize = 25;
const VGA_WIDTH: usize = 80;

#[repr(transparent)]
pub struct VGABuffer(pub [[Volatile<Kharakter>; VGA_WIDTH]; VGA_HEIGHT]);

pub struct VGAWriter {
    col: usize,
    color: ColorCode,
    buffer: *mut VGABuffer
}

impl VGAWriter {
    pub fn write_byte(&mut self, kharakter: u8) {
        let buffer: &mut VGABuffer = unsafe { &mut *self.buffer };
        self.write_byte_impl(buffer, kharakter);
    }

    pub fn write_string(&mut self, s: &str) {
        let buffer: &mut VGABuffer = unsafe { &mut *self.buffer };
        for byte in s.as_bytes() {
            match byte {
                0x20..=0x7e | b'\n' | b'\r' | b'\x07' => {
                    self.write_byte_impl(buffer, *byte)
                },
                _ => self.write_byte_impl(buffer, b'?')
            }
        }
    }
}

impl VGAWriter {
    fn write_byte_impl(&mut self, buffer: &mut VGABuffer, kharakter: u8) {
        match kharakter {
            b'\n' => self.new_line(buffer),
            b'\r' => self.col = 0,
            b'\x07' => if self.col > 0 {
                self.col -= 1;
            },
            kharakter => {
                if self.col >= VGA_WIDTH {
                    self.new_line(buffer);
                }

                let row: usize = VGA_HEIGHT - 1;
                let col: usize = self.col;
                let color: ColorCode = self.color;

                buffer.0[row][col].write(Kharakter {
                    kharakter,
                    color
                });
                self.col += 1;
            }
        }
    }

    fn new_line(&mut self, buffer: &mut VGABuffer) {
        for row /*: usize*/ in 1..VGA_HEIGHT {
            for col /*: usize*/ in 0..VGA_WIDTH {
                let kharakter: Kharakter = buffer.0[row][col].read();
                buffer.0[row - 1][col].write(kharakter);
            }
        }

        self.clear_row(buffer, VGA_HEIGHT - 1);
        self.col = 0;
    }

    fn clear_row(&mut self, buffer: &mut VGABuffer, row: usize) {

        let blank: Kharakter = Kharakter {
            kharakter: b' ',
            color: ColorCode::new(Color::LightCyan, Color::Black)
        };

        for col /*: usize*/ in 0..VGA_WIDTH {
            buffer.0[row][col].write(blank);
        }
    }
}

impl Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

unsafe impl Send for VGAWriter {}

pub static WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter {
    col: 0,
    color: ColorCode::new(Color::LightCyan, Color::Black),
    buffer: 0xb8000 as usize as *mut VGABuffer
});

pub fn get_color() -> ColorCode {
    WRITER.lock().color
}

pub fn set_color(color: ColorCode) {
    WRITER.lock().color = color;
}

pub fn set_color2(fore: Color, back: Color) {
    WRITER.lock().color = ColorCode::new(fore, back);
}

pub fn set_color3(fore: Color, back: Color, blink: bool) {
    WRITER.lock().color = ColorCode::with_blink(fore, back, blink);
}

#[macro_export]
macro_rules! vprint {
    ($($arg:tt)*) => ($crate::vga::print_intern(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vprintln {
    () => ($crate::vprint!("\n"));
    ($($arg:tt)*) => ($crate::vprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn print_intern(args: Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
