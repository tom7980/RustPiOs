pub mod memory;
pub mod cpu;
pub mod drivers;
pub mod console;

// Turns out the RPI4 doesn't like ATAGs & uses a device tree.
// I'll come back to memory allocators later - first I'll initialize the MMU/interrupts
// pub mod atags;

pub static UART_CONSOLE: drivers::uart::LockedUart = unsafe { drivers::uart::LockedUart::new() };