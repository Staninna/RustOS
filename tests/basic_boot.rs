// A file that tests some things for basic booting
// For example if the println!() and print!() works after boot because the panic handler relies on it

// File rules
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::run_tests)]
#![reexport_test_harness_main = "test_main"]

// Imports
use core::panic::PanicInfo;
use rust_os::{print, println};

// One of the starting points of the OS when whe are running unit tests
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

// Used to panic the OS
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(panic_info)
}

// Tests

// Test if println!() works after booting
#[test_case]
fn test_println() {
    println!("Sed pharetra dictum faucibus. Nulla in erat curae");
}

// Test if print!() works after booting
#[test_case]
fn test_print() {
    print!("Sed sit amet feugiat lacus. Duis ex ex porta ante");
}
