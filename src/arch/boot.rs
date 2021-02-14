use crate::arch::{cpu, smp};
use cortex_a::regs::*;
use crate::pi;


#[no_mangle]
pub unsafe fn _start() -> ! {
    use crate::runtime_init;

    if pi::cpu::BOOT_CORE_ID == smp::core_id() {
        SP.set(pi::memory::boot_core_stack_end() as u64);
        runtime_init::runtime_init()
    } else {
        cpu::wait_forever()
    }
}
