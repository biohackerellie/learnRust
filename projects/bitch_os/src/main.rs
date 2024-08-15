#![no_std]
#![no_main]

use core::panic::PanicInfo;

// called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle func name
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop {}
}

mod vga_buffer;
