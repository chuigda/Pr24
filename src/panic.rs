use core::panic::PanicInfo;

use crate::vprint;
use crate::vga::Color;
use crate::vga::set_color2;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_color2(Color::White, Color::Red);

    vprint!("KABOOM! Program panicked");
    if let Some(location) = info.location() {
        vprint!(" at {}:{}:{}", location.file(), location.line(), location.column());
    }
    if let Some(args) = info.message() {
        vprint!(": \"{}\"", args);
    }

    loop {}
}
