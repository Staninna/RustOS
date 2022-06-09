// Exclude standard things for Rust
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::run_tests)]
#![reexport_test_harness_main = "test_main"]

// Imports
use core::panic::PanicInfo;

// Import own files
mod serial;
mod vga_buffer;

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
fn panic(panic_info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", panic_info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// Exit codes for qemu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// Exit from qemu using exit codes
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Main test function
#[cfg(test)]
pub fn run_tests(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// Test functions

// Add automatically printing for test functions
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

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
