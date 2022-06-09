// A file the tests i thing that should panic
// It is for now mostly just a example
// This kind of test can only have one test per file because it panics

// File rules
#![no_std]
#![no_main]

// Imports
use core::panic::PanicInfo;
use rust_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

// One of the starting points of the OS when whe are running unit tests
#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// Test if something will fail
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

// Used to panic the OS (in this case it is good)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
