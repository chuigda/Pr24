#![feature(panic_info_message)]
#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use crate::vga::Color;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vprintln!("Zdravstvuyte, mir!");
    vga::set_color2(Color::Yellow, Color::Blue);
    vprintln!("Hello, world!");

    panic!("We have {} senpais and {} cup of teas", 114, 514);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::set_color2(Color::White, Color::Red);

    vprint!("KABOOM! Program panicked");
    if let Some(location) = info.location() {
        vprint!(" at {}:{}:{}", location.file(), location.line(), location.column());
    }
    if let Some(args) = info.message() {
        vprint!(": \"{}\"", args);
    }

    loop {}
}
