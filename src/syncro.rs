// Here I will implement a lock for my OS, will be very rudimentiary
// for now it's not a lock just a wrapper that "locks" things. I'll probably make it actually do
// some locking on a weekend when I have more time to learn about them.

use core::cell::UnsafeCell;

pub trait Lockable {
    type Data;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
}

//Not actually a lock, just a wrapper to build on later
pub struct NoLock<T>
where
    T: ?Sized {
        inner: UnsafeCell<T>,
}

enum Guard {
    Locked,
    Unlocked,
}

unsafe impl<T> Send for NoLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NoLock<T> where T: ?Sized + Send {}

impl<T> NoLock<T> {
    pub const fn new(inner: T) -> Self {
        NoLock {
            inner: UnsafeCell::new(inner),
        }
    }
}

impl<T> Lockable for NoLock<T> {
    type Data = T;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let inner = unsafe { &mut *self.inner.get() };

        f(inner)
    }
}


