use core::fmt;

use crate::pi::UART_CONSOLE;

pub type ConsoleResult<T> = core::result::Result<T, ConsoleError>;

pub struct ConsoleError{
    kind: ConsoleErrorKind
}

pub enum ConsoleErrorKind {
    TimedOut,
}

impl ConsoleError {
    pub fn new(kind: ConsoleErrorKind) -> ConsoleError {
        ConsoleError{
            kind: kind
        }
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) {
        for byte in buf {
            self.write_byte(*byte);
        }
    }

    fn write_byte(&mut self, byte: u8);

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

}

pub trait Read {
    fn read_byte(&self) -> ConsoleResult<u8>;
}


pub fn _print(args: fmt::Arguments) {
    UART_CONSOLE.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($($arg:tt)*) => ({
        $crate::console::_print(format_args_nl!($($arg)*));
    })
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        $crate::console::_print(format_args!($($arg)*));
    })
}