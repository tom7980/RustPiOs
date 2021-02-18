use core::{cell::UnsafeCell, ops::RangeInclusive};

extern "Rust" {
    static __bss_start: UnsafeCell<u64>;
    static __bss_end_inclusive: UnsafeCell<u64>;
}

pub(super) mod map {
    pub const BOOT_CORE_STACK_END: usize    = 0x0008_0000;
    pub const IO_BASE: usize                = 0xFE00_0000;

    pub const GPIO_OFFSET: usize            = 0x0020_0000;
    pub const TIMER_OFFSET: usize           = 0x0000_3000;

    pub const GPIO_START: usize             = IO_BASE + GPIO_OFFSET;
    pub const TIMER_START: usize            = IO_BASE + TIMER_OFFSET;
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