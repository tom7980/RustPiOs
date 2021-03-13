#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]

use core::fmt::Write;

use pi::drivers::timer::spin_sleep_ms;

mod pi;
mod panic_wait;
mod arch;
mod runtime_init;
mod memory;
// mod console;

fn kernel_init() -> ! {

    let mut uart = pi::drivers::uart::MiniUart::new();
    uart.init();

    let mut pin16 = pi::drivers::gpio::GpioPin::new(16).into_output();

    loop {
        // let byte = 0x40;
        // uart.write_byte(byte);
        
        // if uart.has_byte() {
        //     let new = uart.read_byte();

        //     if new == byte {
        //         pin16.set();
        //         spin_sleep_ms(100);
        //         pin16.clear();
        //         spin_sleep_ms(100);
        //     }
        //     else {
        //         pin16.set();
        //         spin_sleep_ms(100);
        //         pin16.clear();
        //         spin_sleep_ms(100);
        //         pin16.set();
        //         spin_sleep_ms(100);
        //         pin16.clear();
        //     }
        // }        

        // spin_sleep_ms(2000);
        uart.write_byte(1);
        uart.write_byte(2);
        uart.write_byte(3);
        uart.write_byte(4);
        uart.write_byte(5);
    };
}
