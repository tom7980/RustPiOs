use core::{slice, str};

use super::raw;
pub use super::raw::{Core, Mem};

#[derive(Debug, Copy, Clone)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None,
}

impl Atag {
    pub fn core(self) -> Option<Core> {
        if let Atag::Core(core) = self {
            return Some(core);
        }
        None
    }

    pub fn mem(self) -> Option<Mem> {
        if let Atag::Mem(mem) = self {
            return Some(mem);
        }
        None
    }

    pub fn cmd(self) -> Option<&'static str> {
        if let Atag::Cmd(cmd) = self {
            return Some(cmd);
        }
        None
    }
}

impl <'a> From<&'a raw::Atag> for Atag {
    fn from(atag: &raw::Atag) -> Atag {
        unsafe{
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => Atag::Core(core),
                (raw::Atag::MEM, &raw::Kind{ mem }) => Atag::Mem(mem),
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => {
                    let length = c_strlen((&cmd.cmd as *const u8) as *const Cchar);
                    let slice = slice::from_raw_parts(&cmd.cmd as *const u8, length);
                    let out = str::from_utf8_unchecked(slice);
                    Atag::Cmd(out)
                }
                (raw::Atag::NONE, _) => Atag::None,
                (id,_) => Atag::Unknown(id),
            }
        }
    }
}

// Hardcoded because the RPI c_char is a u8
type Cchar = u8;

// Search string for null terminator and count the length
unsafe fn c_strlen(p: *const Cchar) -> usize {
    let mut n = 0;
    while *p.offset(n as isize) != 0 {
        n += 1;
    }
    n
}

