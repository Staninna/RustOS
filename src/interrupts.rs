// This file will handle exceptions and interrupts

// Imports
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// Define a Interrupt Descriptor Table
// aka the data structure where all handlers are stored as far i know
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

// Load the interrupt descriptor table
pub fn init_idt() {
    IDT.load();
}

// Add a breakpoint interrupt handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// Tests

// Test if the breakpoint interrupt handler will continue afterwards
#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
