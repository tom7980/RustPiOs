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

    

    loop {
        kprintln!("New Kernel loaded now");
        spin_sleep_ms(3000);
    }
}
