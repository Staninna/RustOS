// The starting point of the OS `_start()` lives here
// It is getting loaded by the bootloader we use

// File rules
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::run_tests)]
#![reexport_test_harness_main = "test_main"]

// Imports
use core::panic::PanicInfo;
use rust_os::println;

// Starting point of the OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    #[cfg(test)]
    test_main();

    loop {}
}

// Used to panic the OS
#[cfg(not(test))]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    println!("{}", panic_info);
    loop {}
}

// Unit tests

// The panic handler for testing when `cargo test`
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
