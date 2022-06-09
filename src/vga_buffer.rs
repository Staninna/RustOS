// Imports
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// Constant variables

// Dimensions of the text buffer size
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Data structures

// Color codes
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Colors for character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

// Characters on the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Character {
    ascii_character: u8,
    color_code: ColorCode,
}

// The text buffer matrix we write into
#[repr(transparent)]
struct Buffer {
    // Just a matrix of Characters
    chars: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Add function to the text buffer

// The writer struct
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
// The function that writes to the text buffer
impl Writer {
    // Write a ASCII byte to the text buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(Character {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    // Write a whole string to the text buffer using write_byte
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII text or \n
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // print `■` if it is non printable
                _ => self.write_byte(0xfe),
            }
        }
    }

    // Move every line up by one
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // Overwrite given row with whitespace
    fn clear(&mut self, row: usize) {
        let blank = Character {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}
// Used to format integers and other types for the Writer
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Add global writer for OS to use
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Marcos

// Add print!() macro globally
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// Add println!() macro globally
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Used in macros to actually write to the text buffer
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// Test functions

// Test if print!() works
#[test_case]
fn test_print_once() {
    print!("Morbi semper iaculis massa, et tempus tellus nunc");
}

// Test if println!() works
#[test_case]
fn test_println_once() {
    println!("Nunc ut sapien varius, facilisis justo at egestas");
}

// Test if print!() works more than BUFFER_WIDTH length
#[test_case]
fn test_print_many_width() {
    print!("Praesent pellentesque, purus ac pharetra eleifend, nunc massa dignissim turpis, consequat tincidunt");
}

// Test if println!() works more than BUFFER_WIDTH length
#[test_case]
fn test_println_many_width() {
    println!("Aliquam eget justo eget nisl porttitor vehicula sed vitae massa. Sed at ex nulla. Morbi lobortis ac");
}

// Test if println!() works more than BUFFER_HEIGHT times
#[test_case]
fn test_print_many_height() {
    for _ in 0..100 {
        println!(" Morbi nec eros nunc. Pellentesque at blandit quis");
    }
}

// Test if print!() works more than BUFFER_HEIGHT times
#[test_case]
fn test_println_many_height() {
    for _ in 0..100 {
        println!("Donec eu turpis elit. Nulla convallis risus morbi");
    }
}

// Test if printed lines with print!() really appear on the screen
#[test_case]
fn test_print_output() {
    let s = "Mauris iaculis, nisi ullamcorper tempus porttitor";
    print!("\n{}", s); // add newline for next test
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 1][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

// Test if printed lines with println!() really appear on the screen
#[test_case]
fn test_println_output() {
    let s = "Donec eget elit non lacus mattis lobortis vivamus";
    println!("\n{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
