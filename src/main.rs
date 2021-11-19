#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(asm)]

use crate::pi::{drivers::timer::spin_sleep_ms};
use core::fmt::Write;

mod pi;
mod panic_wait;
mod arch;
// mod runtime_init;
mod memory;
// mod shell;
mod xmodem;
mod syncro;

#[macro_use]
mod console;
// mod xmodem;


fn kernel_init() -> !{
    
    use pi::UART_CONSOLE;
    use console::{Read, Write};
    use xmodem::{ModemError, ErrorKind};
    
    let dtb_pointer: u64;
    unsafe { asm!("mov {0}, x4", out(reg) dtb_pointer) }

    // Must initialize the UART device before we can print to the console

    unsafe { UART_CONSOLE.init(); }

    kernel_main(dtb_pointer);
}

fn kernel_main(dtb_pointer: u64) -> ! {

    loop{
        spin_sleep_ms(10000);

        kprintln!("DTB Pointer is at: {:?}", dtb_pointer);
    }
}
