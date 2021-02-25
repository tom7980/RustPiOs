#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]

mod pi;
mod panic_wait;
mod arch;
mod runtime_init;
mod memory;

fn kernel_init() -> ! {

    let mut pin_16 = pi::drivers::gpio::GpioPin::new(16);
    pin_16 = pin_16.into_output();


    loop {
        pin_16.set();
        pi::drivers::timer::spin_sleep_ms(1000);
        pin_16.clear();
        pi::drivers::timer::spin_sleep_ms(1000);
    };
}
