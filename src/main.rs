// Exclude standard things for Rust
#![no_std]
#![no_main]

// Imports
use core::panic::PanicInfo;

// Import own files
mod vga_buffer;

// Entry point, since the linker looks for a function
// named `_start` by default
#[no_mangle] // Make function name not scrambled at compile time
pub extern "C" fn _start() -> ! {
    loop {
        println!("We're no strangers to love");
        println!("You know the rules and so do I (do I)");
        println!("A full commitment's what I'm thinking of");
        println!("You wouldn't get this from any other guy");
        println!("I just wanna tell you how I'm feeling");
        println!("Gotta make you understand");
        println!("Never gonna give you up");
        println!("Never gonna let you down");
        println!("Never gonna run around and desert you");
        println!("Never gonna make you cry");
        println!("Never gonna say goodbye");
        println!("Never gonna tell a lie and hurt you");
        println!("We've known each other for so long");
        println!("Your heart's been aching, but you're too shy to say it (say it)");
        println!("Inside, we both know what's been going on (going on)");
        println!("We know the game and we're gonna play it");
        println!("And if you ask me how I'm feeling");
        println!("Don't tell me you're too blind to see");
        println!("Never gonna give you up");
        println!("Never gonna let you down");
        println!("Never gonna run around and desert you");
        println!("Never gonna make you cry");
        println!("Never gonna say goodbye");
        println!("Never gonna tell a lie and hurt you");
        println!("Never gonna give you up");
        println!("Never gonna let you down");
        println!("Never gonna run around and desert you");
        println!("Never gonna make you cry");
        println!("Never gonna say goodbye");
        println!("Never gonna tell a lie and hurt you");
        println!("We've known each other for so long");
        println!("Your heart's been aching, but you're too shy to say it (to say it)");
        println!("Inside, we both know what's been going on (going on)");
        println!("We know the game and we're gonna play it");
        println!("I just wanna tell you how I'm feeling");
        println!("Gotta make you understand");
        println!("Never gonna give you up");
        println!("Never gonna let you down");
        println!("Never gonna run around and desert you");
        println!("Never gonna make you cry");
        println!("Never gonna say goodbye");
        println!("Never gonna tell a lie and hurt you");
        println!("Never gonna give you up");
        println!("Never gonna let you down");
        println!("Never gonna run around and desert you");
        println!("Never gonna make you cry");
        println!("Never gonna say goodbye");
        println!("Never gonna tell a lie and hurt you");
        println!("Never gonna give you up");
        println!("Never gonna let you down");
        println!("Never gonna run around and desert you");
        println!("Never gonna make you cry");
        println!("Never gonna say goodbye");
        println!("Never gonna tell a lie and hurt you");
    }
}

//  Ued to panic the OS
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
