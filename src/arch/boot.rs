use crate::pi::cpu::BOOT_CORE_ID;

global_asm!(include_str!("boot.s"));


#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    use crate::runtime_init;
    runtime_init::runtime_init();
}
