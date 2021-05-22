pub mod memory;
pub mod cpu;
pub mod drivers;

// Global instance of uart device

pub const UART_CONSOLE: drivers::uart::LockedUart = 
    unsafe { drivers::uart::LockedUart::new() };