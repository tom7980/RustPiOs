use crate::pi;
use crate::memory;

#[inline(always)]
unsafe fn zero_bss(){
    memory::zero_volatile(pi::memory::bss_range_inclusive());
}


pub unsafe fn runtime_init() -> ! {
    zero_bss();

    crate::kernel_init()
}