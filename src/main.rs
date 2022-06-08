// Exclude standard things for Rust
#![no_std]
#![no_main]

// Imports
use core::panic::PanicInfo;

// Put I love mom in the memory at 0xb8000
static MOM: &[u8] = b"Never gonna give you up                                                         Never gonna let you down                                                        Never gonna run around and desert you                                           Never gonna make you cry                                                        Never gonna say goodbye                                                         Never gonna tell a lie and hurt you";

// Entry point, since the linker looks for a function
// named `_start` by default
#[no_mangle] // Make function name not scrambled at compile time
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in MOM.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// Used to panic the OS
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
