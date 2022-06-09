// File rules
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::run_tests)]
#![reexport_test_harness_main = "test_main"]

// Imports
use core::panic::PanicInfo;
use rust_os::println;

// Entry point, since the linker looks for a function
// named `_start` by default
#[no_mangle] // Make function name not scrambled at compile time
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

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
