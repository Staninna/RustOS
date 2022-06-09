// A library mainly used for unit testing and exposing other things from rust files

// File rules
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::run_tests)]
#![reexport_test_harness_main = "test_main"]

// Imports
use core::panic::PanicInfo;

// Import own files
pub mod serial;
pub mod vga_buffer;

// Add the function to print to the serial for test functions
pub trait Testable {
    fn run(&self) -> ();
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// The exit codes we could give qemu to exit the OS while running tests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// Will exit qemu with the given exit code
// Used while testing to automatically exit on failure or success
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// The function that runs all the tests for the OS to check if everything is still working
pub fn run_tests(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// The function that actually handles the panics
// It needs to be here because we are using it in other files to so it needs to be public
pub fn test_panic_handler(panic_info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", panic_info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// One of the starting points of the OS when whe are running unit tests
#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

// The panic handler when we are testing the OS with unit tests
// It runs the actual panic handler
#[cfg(test)]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    test_panic_handler(panic_info)
}
