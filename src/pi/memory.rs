use core::{cell::UnsafeCell, ops::RangeInclusive};

/// Grab the BSS section memory locations from the linker
extern "Rust" {
    static __bss_start: UnsafeCell<u64>;
    static __bss_end_inclusive: UnsafeCell<u64>;
}

pub mod map {
    use core::usize;

    pub const KERNEL_LOAD_ADDRESS: usize    = 0x0008_0000;

    pub const BOOT_CORE_STACK_END: usize    = 0x0200_0000;
    pub const IO_BASE: usize                = 0xFE00_0000;

    pub const GPIO_OFFSET: usize            = 0x0020_0000;
    pub const TIMER_OFFSET: usize           = 0x0000_3000;

    /// Auxiliary peripherals: Mini UART, SPI1 & SPI2
    pub const AUX_OFFSET: usize             = 0x0021_5000;

    pub const GPIO_START: usize             = IO_BASE + GPIO_OFFSET;
    pub const TIMER_START: usize            = IO_BASE + TIMER_OFFSET;
    pub const AUX_START: usize              = IO_BASE + AUX_OFFSET;
}

#[inline(always)]
pub fn boot_core_stack_end() -> usize {
    map::BOOT_CORE_STACK_END
}

pub fn bss_range_inclusive() -> RangeInclusive<*mut u64> {
    let range;
    unsafe{
        range = RangeInclusive::new(__bss_start.get(), __bss_end_inclusive.get());
    }
    assert!(!range.is_empty());

    range
}