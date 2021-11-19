use crate::pi::cpu::BOOT_CORE_ID;
use cortex_a::{asm, registers::*};
use tock_registers::interfaces::Writeable;

global_asm!(include_str!("boot.s"));


// Prepare to transition from EL2 to EL1 by creating a fake 
// EL2 program status with all interrupts masked, updating the link
// register to our kernel_init function pointer then eret out of EL2 
// into EL1

#[inline(always)]
unsafe fn prepare_el2_to_el1(boot_core_stack_end: u64){
    // Allow timer access for EL1
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading counters/timers
    CNTVOFF_EL2.set(0);

    // We are in AArch64 so EL1 also needs to be
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    //Simulate exception return with masked interrupts
    //so they are masked when we eret into EL1
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // Change link register pointer to kernel_init();
    ELR_EL2.set(crate::kernel_init() as *const () as u64);

    // Move stack pointer for EL1, we don't plan to come back to EL2 so we can use the same stack
    SP_EL1.set(boot_core_stack_end);
}

// x0 is loaded with the stack address when called from boot.s and passed into
// this function.
#[no_mangle]
pub unsafe fn _start_rust(physical_boot_core_stack_end: u64, dtb_pointer: u64) -> ! {
    //BSS is zeroed - prepare EL2 to EL1 change
    prepare_el2_to_el1(physical_boot_core_stack_end);
    // Use exception return to "return" to EL1. Because we put 'kernel_init()' in the link register
    // this will jump to there and continue setting up the kernel
    // Move the DTB Pointer into register x4 beforehand so we can use it in EL1 also
    asm!("mov x4, {0}", in(reg) dtb_pointer);
    asm::eret()
}
