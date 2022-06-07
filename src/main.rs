// Exclude standard things for Rust
#![no_std]
#![no_main]

// Imports
use core::panic::PanicInfo;

// Entry point
#[no_mangle] // Make function name not scrambled at compile time
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Used to panic the OS
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
