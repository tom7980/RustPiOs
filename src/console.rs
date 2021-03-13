use core::fmt;

pub trait Write {
    fn write(&mut self, buf: &mut [u8]) -> Result<Err,usize>;

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        fmt::Write::write_fmt(&mut self, args)
    }

    fn flush(&self);
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}