use crate::pi::memory;
use super::common::StaticRef;
use tock_registers::{register_bitfields, register_structs};
use tock_registers::registers::*;

register_bitfields!{
    u32,

    ///Actually W1C registers, write 1 to clear, read to check timer comparisons.
    CS [
        M3 OFFSET(3) NUMBITS(1) [
            NoMatch = 0,
            Matched = 1
        ],

        M2 OFFSET(2) NUMBITS(1) [
            NoMatch = 0,
            Matched = 1
        ],

        M1 OFFSET(1) NUMBITS(1) [
            NoMatch = 0,
            Matched = 1
        ],

        M0 OFFSET(0) NUMBITS(1) [
            NoMatch = 0,
            Matched = 1
        ]

    ],

    ///System timer counter lower 32 bits
    CLO [
        CNT OFFSET(0) NUMBITS(32)[]
    ],

    ///System timer counter upper 32 bits
    CHI [
        CNT OFFSET(0) NUMBITS(32)[]
    ],

    ///Generic System timer compare register
    CX [
        CMP OFFSET(0) NUMBITS(32)[]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    TimerBlock {
        (0x000 => cs:  ReadWrite<u32, CS::Register>),
        (0x004 => clo: ReadOnly<u32, CLO::Register>),
        (0x008 => chi: ReadOnly<u32, CHI::Register>),
        (0x00c => cx: [ReadWrite<u32, CX::Register>; 4]),
        (0x010 => @END),
    }
}

pub struct Timer {
    registers: StaticRef<TimerBlock>
}

///Wrapper for concurrent access once I implement a Mutex or lock type
///currently useless but it's nice to plan ahead
pub struct TimerDevice {
    inner: Timer,
}

impl Timer {
    /// Constant function allows me to instantiate system timer a single time
    /// and use it as a reference
    pub const fn new() -> Timer {
        Timer{
            registers: unsafe { 
                StaticRef::new(memory::map::TIMER_START as *const TimerBlock)
            },
        }
    }

    ///Reads the system timer and returns the 64 bit counter value
    ///Number is elapsed microseconds

    pub fn read(&self) -> u64 {
        let low = self.registers.clo.read(CLO::CNT);
        let high = self.registers.chi.read(CHI::CNT);
        ((high as u64) << 32) | (low as u64)
    }
}

impl TimerDevice {
    pub fn read(&self) -> u64 {
        self.inner.read()
    }
}

///Initialization of system timer available outside of this crate

pub const SYSTEM_TIMER: Timer = Timer::new();

pub fn spin_sleep_us(delay: u64) {
    let old = SYSTEM_TIMER.read();
    loop{
        let new = SYSTEM_TIMER.read();
        if old + delay <= new {
            break;
        }
    }
}

pub fn spin_sleep_ms(ms: u64){
    spin_sleep_us(ms*1000);
}