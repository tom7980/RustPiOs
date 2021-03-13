#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]

use core::fmt::Write;

mod pi;
mod panic_wait;
mod arch;
mod runtime_init;
mod memory;
// mod console;

fn kernel_init() -> ! {

    let mut uart = pi::drivers::uart::MiniUart::new();
    uart.init();


    loop {
        // uart.write_str("Hello").expect("No");
        uart.write_str("abcdefghijklmnopqrstuvwxyz").unwrap();
    };
}
