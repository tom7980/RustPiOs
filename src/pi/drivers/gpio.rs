use crate::pi::memory;
use super::common::StaticRef;
use tock_registers::{register_bitfields, register_structs};
use tock_registers::registers::*;

register_bitfields!{
    u32,

    GPFSELX [
        FSEL0 OFFSET(0) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL1 OFFSET(3) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL2 OFFSET(6) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL3 OFFSET(9) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL4 OFFSET(12) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL5 OFFSET(15) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL6 OFFSET(18) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL7 OFFSET(21) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL8 OFFSET(24) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ],
        FSEL9 OFFSET(27) NUMBITS(3) [
            Input   = 0b000,
            Output  = 0b001,
            Alt0    = 0b100,
            Alt1    = 0b101,
            Alt2    = 0b110,
            Alt3    = 0b111,
            Alt4    = 0b011,
            Alt5    = 0b010
        ]
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
    ]
}