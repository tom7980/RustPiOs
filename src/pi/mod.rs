pub mod memory;
pub mod cpu;
pub mod drivers;

pub static UART_CONSOLE: drivers::uart::LockedUart = unsafe { drivers::uart::LockedUart::new() };

