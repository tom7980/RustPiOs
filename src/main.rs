#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(format_args_nl)]
#![feature(global_asm)]

use crate::{console::Write, pi::{UART_CONSOLE, drivers::timer::spin_sleep_ms}};

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

    // let mut gpio_10 = pi::drivers::gpio::GpioPin::new(10).into_output();

    // loop {
    //     gpio_10.set();
    //     spin_sleep_ms(2000);
    //     gpio_10.clear();
    //     spin_sleep_ms(2000);
    // }

    //kprintln available past this point
    for i in 1..10 {UART_CONSOLE.write_byte(b'a');}
    
    UART_CONSOLE.write_fmt(format_args!("{}", "Hi"));
    kprint!("Waiting for new kernel");

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
                    kprint!("UART Timed Out");
                    continue
                },
                _ => kprintln!("Error: {:?}", err)
            }
        }                
    }
}
