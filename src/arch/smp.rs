use cortex_a::regs::*;

#[inline(always)]
pub fn core_id<T>() -> T
where
    T: From<u8>,
{
    const CORE_MASK: u64 = 0b11;

    T::from((MPIDR_EL1.get() & CORE_MASK) as u8)
}