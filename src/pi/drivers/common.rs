use core::ops::Deref;
use core::marker::PhantomData;


///Static reference initialization using a wrapper type

#[derive(Debug)]
pub struct StaticRef<T> {
    ptr: usize,
    phantom: PhantomData<fn() -> T>
}

impl<T> StaticRef<T> {
    /// New static reference
    ///
    /// ## Saftey
    ///
    /// Calls to this must ensure memory is static and does not overlap
    pub const unsafe fn new(ptr: usize) -> StaticRef<T> {
        StaticRef { 
            ptr: ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for StaticRef<T> {
    fn clone(&self) -> Self {
        StaticRef { 
            ptr: self.ptr,
            phantom: PhantomData
        }
    }
}

impl<T> Copy for StaticRef<T> {}

impl<T> Deref for StaticRef<T> {
    type Target = T;
    fn deref(&self) -> &'static T {
        unsafe { &*(self.ptr as *const _) }
    }
}