// I should probably make this the Auxillary peripherals module but I'm only going to do
// MiniUart for now so I can start chainloading my kernel

use crate::{
    console, console::{ConsoleError, ConsoleErrorKind, ConsoleResult}, pi::memory, syncro::{NoLock, Lockable},
};
use super::{common::StaticRef, timer::SYSTEM_TIMER};
use tock_registers::{register_bitfields, register_structs};
use tock_registers::registers::*;
use tock_registers::interfaces::*;
use super::gpio::*;

use core::fmt::{self, Write};

pub struct MiniUart {
    registers: StaticRef<MiniRegisters>,
    timeout: Option<u32>
}

pub use MiniUart as PanicOut;

impl MiniUart {
    pub const unsafe fn new() -> MiniUart {
        MiniUart{
            registers: unsafe { StaticRef::new(memory::map::AUX_START) },
            timeout: None
        }
    }

    pub fn init(&mut self) {
        // we might be in a panic so flush the buffer
        self.flush();
        
        self.registers.auxenable.write(AUXENABLE::UART::SET);

        let mut pin14 = GpioPin::new(14).into_alt(Function::Alt5);
        pin14.set_no_pud();

        let mut pin15 = GpioPin::new(15).into_alt(Function::Alt5);
        pin15.set_no_pud();

        self.registers.lcr.set(3);

        let divisor: u32 = (500000000/(115200*8)) - 1;
        self.registers.baud.write(BAUD::RATE.val(divisor));
        
        self.registers.cntl.modify(CNTL::RXENABLE::SET + CNTL::TXENABLE::SET);
    }

    pub fn timeout(&mut self, ms: u32) {
        self.timeout = Some(ms);
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.registers.lsr.matches_any(LSR::TXEMPTY::SET) {};
        self.registers.io.set(byte);
    }

    pub fn wait_for_byte(&self) -> Result<(), ()> {
        match self.timeout {
            None => {
                loop{
                    if self.has_byte(){
                        return Ok(())
                    }
                }
            },
            Some(timeout) => {
                let time = SYSTEM_TIMER.read();
                let deadline = time + (timeout as u64) * 1000;
                while deadline >= SYSTEM_TIMER.read() {
                    if self.has_byte(){
                        return Ok(())
                    }
                }
                Err(())
            }
        }
    }

    pub fn has_byte(&self) -> bool {
        self.registers.lsr.matches_any(LSR::DATA::SET)
    }

    pub fn read_byte(&mut self) -> u8 {
        while !self.has_byte() {};
        self.registers.io.read(IO::IO)
    }

    // Helper function to test UART debugging
    pub fn echo(&mut self) {
        while !self.has_byte() {};
        let temp = self.registers.io.read(IO::IO);
        match temp {
            b'\n' | b'\r' => {
                self.write_byte(b'\r');
                self.write_byte(b'\n');
            },
            _ => self.write_byte(temp)
        }
    }

    fn flush(&self) {
        while !self.registers.lsr.matches_any(LSR::TXEMPTY::SET) {};
    }
}

impl fmt::Write for MiniUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'\n' | b'\r' => {
                    self.write_byte(b'\r');
                    self.write_byte(b'\n');
                },
                _ => self.write_byte(byte)
            }
        }

        Ok(())
    }
}

pub struct LockedUart {
    inner: NoLock<MiniUart>,
}

impl LockedUart {
    /// Create a UART device encapsulated by a concurrent lock
    ///
    /// Doesn't actually do anything right now as the lock isn't a lock
    /// No need to pass in a memory address as it will always be the same on the 
    /// Rpi 4. 
    ///
    /// This could be made more generic for other microcontrollers if I allowed other register maps
    /// or memory addresses.
    pub const unsafe fn new() -> Self {
        Self {
            inner: NoLock::new(MiniUart::new()),
        }
    }

    pub unsafe fn init(&self) -> Result<(),()> {
        self.inner.lock(|inner| inner.init());

        Ok(())
    }

    pub fn timeout(&self, ms: u32) {
        self.inner.lock(|inner| inner.timeout(ms));
    }

    pub fn flush(&self) {
        self.inner.lock(|inner| inner.flush());
    }
}

impl console::Write for LockedUart {
    fn write_byte(&mut self, byte: u8) {
        self.inner.lock(|inner| inner.write_byte(byte));
    }
    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}

impl console::Read for LockedUart {
    fn read_byte(&self) -> ConsoleResult<u8> {
        self.inner.lock(
            |inner| match inner.wait_for_byte() {
                Ok(()) => Ok(inner.read_byte()),
                Err(()) => Err(ConsoleError::new(ConsoleErrorKind::TimedOut))
        })
    }
}

register_structs!{
    pub MiniRegisters{
        (0x000 => _r1),
        (0x004 => auxenable: ReadWrite<u32, AUXENABLE::Register>),
        (0x008 => _r2),
        (0x040 => io: ReadWrite<u8, IO::Register>),
        (0x041 => _r3),
        (0x044 => ier: ReadWrite<u32, IER::Register>),
        (0x048 => iir: ReadWrite<u32, IIR::Register>),
        (0x04c => lcr: ReadWrite<u32, LCR::Register>),
        (0x050 => mcr: ReadWrite<u32, MCR::Register>),
        (0x054 => lsr: ReadWrite<u32, LSR::Register>),
        (0x058 => _r4),
        (0x060 => cntl: ReadWrite<u32, CNTL::Register>),
        (0x064 => status: ReadOnly<u32, STATUS::Register>),
        (0x068 => baud: ReadWrite<u32, BAUD::Register>),
        (0x06c => @END),
    }
}

register_bitfields!{
    u8,

    IO [
        IO OFFSET(0) NUMBITS(8) []
    ]
}

register_bitfields!{
    u32,

    AUXENABLE [
        UART OFFSET(0) NUMBITS(1) [],
        SPI1 OFFSET(1) NUMBITS(1) [],
        SPI2 OFFSET(2) NUMBITS(1) []
    ],

    // Interrupt enable register
    IER [
        // Same but for transmit FIFO
        TXIR OFFSET(0) NUMBITS(1) [],
        // If set interrupt line is asserted when reciever FIFO holds at least 1 byte
        RXIR OFFSET(1) NUMBITS(1) []        
    ],

    // interrupt status register
    IIR [
        // Clear if interrupt is pending
        PENDING OFFSET(0) NUMBITS(1) [],
        // Write to clear FIFO buffers
        CLEAR OFFSET(1) NUMBITS(2) [
            TxClr = 0b01,
            RxClr = 0b10
        ]
    ],

    LCR [
        // Set to 1 for 8bit mode
        DATASIZE OFFSET(0) NUMBITS(2) []
    ],

    MCR [
        RTS OFFSET(1) NUMBITS(1) []
    ],

    LSR [
        DATA OFFSET(0) NUMBITS(1) [],
        TXEMPTY OFFSET(5) NUMBITS(1) []
    ],

    CNTL [
        RXENABLE OFFSET(0) NUMBITS(1) [],
        TXENABLE OFFSET(1) NUMBITS(1) []
    ],

    STATUS [
        SYM_AVAILABLE OFFSET(0) NUMBITS(1) [],
        SPACE_AVAILABLE OFFSET(1) NUMBITS(1) [],
        RX_IDLE OFFSET(2) NUMBITS(1) [],
        TX_IDLE OFFSET(3) NUMBITS(1) [],
        RX_OVERRUN OFFSET(4) NUMBITS(1) [],
        TX_FULL OFFSET(5) NUMBITS(1) [],
        TX_EMPTY OFFSET(8) NUMBITS(1) [],
        TX_DONE OFFSET(9) NUMBITS(1) []
    ],

    BAUD [
        RATE OFFSET(0) NUMBITS(16) []
    ]
}