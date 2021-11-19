mod raw;
mod atag;

pub use self::atag::*;
use crate::{kprintln, pi::memory::map::ATAG_LOAD_ADDRESS};

pub struct Atags {
    ptr: &'static raw::Atag,
}

impl Atags {
    pub fn get() -> Atags {
        kprintln!("Atags::get()");
        Atags{
            ptr: unsafe { &*(ATAG_LOAD_ADDRESS as *const raw::Atag) }
        }
    }
}

impl Iterator for Atags {
    type Item = Atag;

    fn next(&mut self) -> Option<Atag> {
        if let Some(tag) = self.ptr.next() {
            self.ptr = tag;
            return Some(tag.into());
        }
        None
    }
}