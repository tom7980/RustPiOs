use super::drivers::uart;
use crate::console::Write;

pub unsafe fn panic_console() -> uart::PanicOut {
    let mut panic_uart = uart::PanicOut::new();
    
    panic_uart.init();
    panic_uart
}