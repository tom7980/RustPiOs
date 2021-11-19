use core::ops::{RangeInclusive, Range};
use core::fmt;

use crate::console;

pub unsafe fn zero_volatile<T>(range: RangeInclusive<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = *range.start();
    let end_inclusive = *range.end();

    while ptr <= end_inclusive {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}


/// This wraps a range of pointers into memory for use with our console::Write implementation
///
/// This is inherantly unsafe as it allows the user to write arbitrary data into memory at any location 
/// however I need it to be able to write a new kernal binary to the Pi over UART
///
/// I'm unsure if this is the best way to do this but it's the only way I could come up with to wrap memory for use with
/// the console::Write trait
pub struct MemCursorWriteOnly<T>
    where 
        T: From<u8>
{
    range: Range<*mut T>,
    index: isize
}

impl<T> MemCursorWriteOnly<T> where T: From<u8> {
    pub fn new(input: Range<*mut T>) -> MemCursorWriteOnly<T>{
        MemCursorWriteOnly {
            range: input,
            index: 0
        }
    }

    fn _write_byte_inner(&mut self, byte: u8) {
        unsafe { 
            let mut ptr = self.range.start.offset(self.index);
            let end = self.range.end;

            
            if ptr > end {
                //Just don't do anything
            }
            else{
                core::ptr::write_volatile(ptr, T::from(byte));
                self.index += 1;
            }
        }
    }
}


// We're reusing the console write trait as that is what I'm using on my xmodem implementation
//
// I should really write a separate IO::Write trait that has more useful methods and use that for my xmodem 
// but I'm in too deep now and I'd just like to get it to work first - note for the future, write better abstracted traits
impl<T> console::Write for MemCursorWriteOnly<T> where T:From<u8> {
    fn write_byte(&mut self, byte: u8) {
        self._write_byte_inner(byte);
    }

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        Ok(())
    }
}