#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![no_std]
#![no_main]

mod vga;
mod panic;

use crate::vga::Color;
use crate::vga::set_color2;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)] test_main();

    vprintln!("Zdravstvuyte, mir!");
    set_color2(Color::Yellow, Color::Blue);
    vprintln!("Hello, world!");

    panic!("We have {} senpais and {} cup of teas", 114, 514);
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) -> ! {
    use crate::vga::ColorCode;
    use crate::vga::{get_color, set_color};

    let original_color: ColorCode = get_color();
    set_color2(Color::Yellow, Color::Blue);
    vprintln!("Running {} tests", tests.len());
    set_color(original_color);

    for (idx, test) in tests.iter().enumerate() {
        set_color2(Color::Yellow, Color::Blue);
        vprintln!("Running test {} of {}", idx + 1, tests.len());
        set_color(original_color);

        test();
    }

    vprint!("All tests passed!");

    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn trivial_assertion2() {
    assert_eq!(2 + 2, 3);
}
