#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(format_args_nl)]

use crate::pi::UART_CONSOLE;
use core::ops::Range;

mod pi;
mod panic_wait;
mod arch;
mod runtime_init;
mod memory;
mod shell;
mod xmodem;

#[macro_use]
mod console;
// mod xmodem;


fn kernel_init() -> ! {

    use xmodem::{ModemError, ErrorKind};

    // Must initialize the UART device before we can print to the console
    pi::UART_CONSOLE.init();

    //kprintln available past this point

    kprintln!("Waiting for new kernel");

    let kernel_addr: *mut u8 = pi::memory::map::KERNEL_LOAD_ADDRESS as *mut u8;
    // Boot Core Stack End is same address as bootloader start address & end of memory space for Kernel
    let kernel_end: *mut u8 = pi::memory::map::BOOT_CORE_STACK_END as *mut u8;
    let kernel_range = kernel_addr..kernel_end;
    let wrapped_kernel_memory = memory::MemCursorWriteOnly::new(kernel_range);

    loop {
        match xmodem::Xmodem::receive(pi::UART_CONSOLE, wrapped_kernel_memory) {
            Ok(_) => {
                // This interprets a memory address as a function I can call from rust to jump to the new kernel
                // TODO: Check the rust calling convention to ensure this doesn't introduce any weird errors with CPU registers
                // I think perhaps I can call it as a naked function? 
                let kernel: fn() -> ! = unsafe { core::mem::transmute(kernel_addr) };
                kernel()
            }
            Err(err) => match err.kind() {
                _ => kprintln!("Error: {:?}", err)
            }
        }                
    }

}
