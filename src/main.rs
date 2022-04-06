#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use crate::vga::Color;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vprintln!("Zdravstvuyte, mir!");
    vga::set_color2(Color::Yellow, Color::Blue);
    vprint!("Hello, world!");

    loop {}
}
