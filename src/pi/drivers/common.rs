use core::{marker::PhantomData, ops};

///Module creates a wrapper around a set of registers that implicitly dereferences to a constant containing the wrapped block of registers
///Can be used to create a single static reference to an MMIO device
///Requires device to implement a const unsafe fn creating a new instance of this type i.e
/// ```
/// pub const unsafe fn new (mmio_start_address: usize) -> Self {
///    Self {
///        registers: MMIODerefWrapper<RegisterBlock>::new(mmio_start_address),
///    }
/// }
/// ```
/// This can then be initialized as a static reference by creating a static variable and running this function
pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>
}

impl<T> MMIODerefWrapper<T> {
    pub const unsafe fn new(start_addr: usize) -> Self {
        Self{
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
    }
}