#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(format_args_nl)]
#![feature(global_asm)]

use crate::pi::{drivers::timer::spin_sleep_ms};
use core::fmt::Write;

mod pi;
mod panic_wait;
mod arch;
mod runtime_init;
mod memory;
mod shell;
mod xmodem;
mod syncro;

#[macro_use]
mod console;
// mod xmodem;


fn kernel_init() -> ! {
    
    use pi::UART_CONSOLE;
    use console::{Read, Write};
    use xmodem::{ModemError, ErrorKind};

    // unsafe { 
    //     let mut uart = pi::drivers::uart::MiniUart::new();

    //     uart.init();

    //     let s = "hello world";

    //     loop {
    //         for b in s.bytes() {
    //             uart.write_byte(b);
    //         }
    //     }
    // };

    




    // Must initialize the UART device before we can print to the console
    // Probably worth splitting this out into a separate init function that is unsafe
    unsafe { UART_CONSOLE.init(); }

    // let mut gpio_10 = pi::drivers::gpio::GpioPin::new(10).into_output();

    // loop {
    //     gpio_10.set();
    //     spin_sleep_ms(2000);
    //     gpio_10.clear();
    //     spin_sleep_ms(2000);
    // }

    //kprintln available past this point

    UART_CONSOLE.write_fmt(format_args!("{}", "Hello, World")).unwrap();
    
    kprintln!("Waiting for new kernel");

    let kernel_addr: *mut u8 = pi::memory::map::KERNEL_LOAD_ADDRESS as *mut u8;
    // Boot Core Stack End is same address as bootloader start address & end of memory space for Kernel
    let kernel_end: *mut u8 = pi::memory::map::BOOT_CORE_STACK_END as *mut u8;
    UART_CONSOLE.timeout(750);
    

    loop {
        let kernel_range = kernel_addr..kernel_end;
        let wrapped_kernel_memory = memory::MemCursorWriteOnly::new(kernel_range);
        match xmodem::Xmodem::receive(pi::UART_CONSOLE, wrapped_kernel_memory) {
            Ok(_) => {
                // I stole this but it basically turns the start adress of the new kernel into a function
                // which we can call to enter into it - bonus this means I don't have to move the stack pointer
                // or write assembly code
                let kernel: fn() -> ! = unsafe { core::mem::transmute(kernel_addr) };
                kernel()
            }
            Err(err) => match err.kind() {
                ErrorKind::ConsoleError => {
                    kprintln!("UART Timed Out");
                    continue
                },
                _ => kprintln!("Error: {:?}", err)
            }
        }                
    }
}
