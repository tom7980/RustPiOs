use crate::kprintln;
use core::fmt;

#[repr(C)]
pub struct Atag {
    pub dwords: u32,
    pub tag: u32,
    pub kind: Kind,
}

impl Atag {
    pub const NONE: u32 = 0x00000000;
    pub const CORE: u32 = 0x54410001;
    pub const MEM: u32 = 0x54410002;
    pub const VIDEOTEXT: u32 = 0x54410003;
    pub const RAMDISK: u32 = 0x54410004;
    pub const INITRD2: u32 = 0x54410005;
    pub const SERIAL: u32 = 0x54410006;
    pub const REVISION: u32 = 0x54410007;
    pub const VIDEOLFB: u32 = 0x54410008;
    pub const CMDLINE: u32 = 0x54410009;
    
    pub fn next(&self) -> Option<&Atag> {
        unsafe {
            let addr = (self as *const Atag) as *const u32;
            kprintln!("{:?}", addr);
            let next_atag = addr.offset(self.dwords as isize) as *mut Atag;
            kprintln!("{:?}", next_atag);

            match (*next_atag).tag {
                Atag::NONE => None,
                _ => next_atag.as_ref(),
            }
        }
    }
}

#[repr(C)]
pub union Kind {
    pub core: Core,
    pub mem: Mem,
    pub cmd: Cmd,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Core {
    pub flags: u32,
    pub page_size: u32,
    pub root_dev: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mem {
    pub size: u32,
    pub start: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Cmd {
    pub cmd: u8,
}
