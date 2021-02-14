#![no_main]
#![no_std]
#![feature(lang_items)]

mod pi;
mod panic_wait;
mod arch;
mod runtime_init;
mod memory;

fn kernel_init() -> ! {

    const DELAY: usize = 500000;
    const GPIO_BASE: usize = 0xFE20_0000;

    let GPFSEL1 = 0xFE20_0004 as *mut u32;
    let GPSET0 = 0xFE20_001C as *mut u32;
    let GPCLR0 = 0xFE20_0028 as *mut u32;

    unsafe { GPFSEL1.write_volatile(0b001 << 18)};

    loop {
        unsafe {GPSET0.write_volatile(1 << 16)};
        arch::cpu::spin_for_cycles(DELAY);
        unsafe {GPCLR0.write_volatile(1 << 16)};
        arch::cpu::spin_for_cycles(DELAY);
    };
}
