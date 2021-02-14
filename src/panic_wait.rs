use crate::arch::cpu;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cpu::wait_forever()
}

#[lang = "eh_personality"]
fn eh_personality() {}
