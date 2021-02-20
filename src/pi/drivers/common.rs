use core::ops::Deref;

///Static reference initialization using a wrapper type

#[derive(Debug)]
pub struct StaticRef<T> {
    ptr: *const T,
}

impl<T> StaticRef<T> {
    /// New static reference
    ///
    /// ## Saftey
    ///
    /// Calls to this must ensure memory is static and does not overlap
    pub const unsafe fn new(ptr: *const T) -> StaticRef<T> {
        StaticRef { ptr }
    }
}

impl<T> Clone for StaticRef<T> {
    fn clone(&self) -> Self {
        StaticRef { ptr: self.ptr}
    }
}

impl<T> Copy for StaticRef<T> {}

impl<T> Deref for StaticRef<T> {
    type Target = T;
    fn deref(&self) -> &'static T {
        unsafe { &*self.ptr }
    }
}