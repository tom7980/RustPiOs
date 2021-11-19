use crate::pi::memory;
use super::common::StaticRef;
use tock_registers::{register_bitfields, register_structs};
use tock_registers::registers::*;
use core::marker::PhantomData;
use tock_registers::interfaces::*;


pub struct GpioPin<State> {
    /// Far to lazy to create an enum to represent the pin number so it's a u32, probably too big :)
    pin: u8,
    registers: StaticRef<GpioRegisters>,
    _state: PhantomData<State>,
}

/// GPIO States available
pub enum Uninitialized {}
pub enum Input {}
pub enum Output {}
pub enum Alt {}

#[repr(u8)]
pub enum Function {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010
}

/// Complier will infer type of S via context of calling functions return type
impl<T> GpioPin<T>{
    pub fn transition<S>(self) -> GpioPin<S> {
        GpioPin{
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData
        }
    }
}

impl GpioPin<Uninitialized> {
    pub fn new(pin: u8) -> GpioPin<Uninitialized> {
        GpioPin {
            pin,
            registers: unsafe { StaticRef::new(memory::map::GPIO_START) },
            _state: PhantomData
        }
    }

    pub fn into_alt(self, function: Function) -> GpioPin<Alt> {
        let register = self.pin/10;
        let pin = self.pin % 10;
        let value = self.registers.gpfselx[register as usize].get();
        self.registers.gpfselx[register as usize].set(value | ((function as u32) << (3 * pin)));
        self.transition()
    }

    pub fn into_output(self) -> GpioPin<Output> {
        self.into_alt(Function::Output).transition()
    }

    pub fn into_input(self) -> GpioPin<Input> {
        self.into_alt(Function::Input).transition()
    }

}

impl GpioPin<Output> {
    pub fn set(&mut self) {
        let register = self.pin/32;
        let pin = self.pin % 32;
        self.registers.gpsetx[register as usize].set(1 << pin);
    }
    pub fn clear(&mut self) {
        let register = self.pin/32;
        let pin = self.pin % 32;
        self.registers.gpclrx[register as usize].set(1 << pin);
    }
}

impl GpioPin<Input> {
    pub fn level(&mut self) -> bool {
        let register = self.pin/32;
        let pin = self.pin % 32;
        (self.registers.gplevx[register as usize].get() & 1 << pin) == 1 << pin
    }
}

impl GpioPin<Alt> {
    pub fn set_no_pud(&mut self) {
        let register = self.pin/16;
        let pin = self.pin % 16;
        match pin {
            0 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD00::NoResistor),
            1 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD01::NoResistor),
            2 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD02::NoResistor),
            3 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD03::NoResistor),
            4 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD04::NoResistor),
            5 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD05::NoResistor),
            6 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD06::NoResistor),
            7 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD07::NoResistor),
            8 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD08::NoResistor),
            9 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD09::NoResistor),
            10 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD10::NoResistor),
            11 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD11::NoResistor),
            12 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD12::NoResistor),
            13 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD13::NoResistor),
            14 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD14::NoResistor),
            15 => self.registers.gppupdx[register as usize].write(GPPUPDX::GPPUPD15::NoResistor),
            _ => panic!("No more pins")
        }
    }
}

register_structs!{
    GpioRegisters{
        (0x000 => gpfselx:  [ReadWrite<u32, GPFSELX::Register>; 6]),
        (0x018 => _r1),
        (0x01c => gpsetx:   [ReadWrite<u32, GPSETX::Register>; 2]),
        (0x024 => _r2),
        (0x028 => gpclrx:   [ReadWrite<u32, GPCLRX::Register>; 2]),
        (0x030 => _r3),
        (0x034 => gplevx:   [ReadWrite<u32, GPLEVX::Register>; 2]),
        (0x03c => _r4),        
        (0x040 => gpedsx:   [ReadWrite<u32, GPEDSX::Register>; 2]),
        (0x048 => _r5),
        (0x04c => gprenx:   [ReadWrite<u32, GPRENX::Register>; 2]),
        (0x054 => _r6),
        (0x058 => gpfenx:   [ReadWrite<u32, GPFENX::Register>; 2]),
        (0x060 => _r7),
        (0x064 => gphenx:   [ReadWrite<u32, GPHENX::Register>; 2]),
        (0x06c => _r8),
        (0x070 => gplenx:   [ReadWrite<u32, GPLENX::Register>; 2]),
        (0x078 => _r9),
        (0x07c => gparenx:  [ReadWrite<u32, GPARENX::Register>; 2]),
        (0x084 => _r10),
        (0x088 => gpafenx:  [ReadWrite<u32, GPAFENX::Register>; 2]),
        (0x090 => _r11),
        (0x0e4 => gppupdx:  [ReadWrite<u32, GPPUPDX::Register>; 4]),
        (0x164 => @END),
    }
}

register_bitfields!{
    u32,

    GPFSELX [
        FSEL OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Pin Set Register
    GPSETX [
        SET OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Pin Clear Register
    GPCLRX [
        CLR OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Pin Level Register
    GPLEVX [
        LEV OFFSET(0) NUMBITS(32) []
    ],
    /// W1C Registers
    /// Event Detect Status Register
    /// Relevant bit is set when relevant programmed event is detected
    /// Better info at [RPI Peripherals Doc](https://datasheets.raspberrypi.org/bcm2711/bcm2711-peripherals.pdf)
    /// Page 71
    GPEDSX [
        EDS OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Rising Edge Detect enable register
    ///
    /// Register defines which pins will set a bit in the event detect register
    /// on a rising edge event
    GPRENX [
        REN OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Falling Edge Detect enable register
    ///
    /// Same as the last register but for falling edges
    GPFENX [
        FEN OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO High Level Detect enable register
    GPHENX [
        HEN OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Low Level Detect enable register
    GPLENX [
        LEN OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Async rising edge detect enable register
    GPARENX [
        AREN OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Async falling edge detect enable register
    GPAFENX [
        AFEN OFFSET(0) NUMBITS(32) []
    ],
    /// GPIO Pull up Pull down Control Registers
    GPPUPDX [
        GPPUPD00 OFFSET(0) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD01 OFFSET(2) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD02 OFFSET(4) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD03 OFFSET(6) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD04 OFFSET(8) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD05 OFFSET(10) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD06 OFFSET(12) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD07 OFFSET(14) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD08 OFFSET(16) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD09 OFFSET(18) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD10 OFFSET(20) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD11 OFFSET(22) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD12 OFFSET(24) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD13 OFFSET(26) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD14 OFFSET(28) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ],
        GPPUPD15 OFFSET(30) NUMBITS(2) [
            NoResistor  = 0b00,
            PullDown    = 0b01,
            PullUp      = 0b10,
            Reserved    = 0b11
        ]
    ]
}