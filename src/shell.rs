use crate::pi::drivers::{timer::spin_sleep_ms};
use core::str;

use crate::kprintln;

use staticvec::{staticvec, StaticVec};

struct Command<'a> {
    args: StaticVec<&'a str, 65>,
}

impl<'a> Command<'a> {
    fn parse(s: &str) -> Result<Command, &'static str> {
        let mut args = StaticVec::<&str, 65>::new();

        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg);
        }

        if args.is_empty() {
            return Err("No args")
        }

        Ok(Command { args })
    }

    fn path(&self) -> &str {
        self.args[0]
    }
}

pub fn shell(prefix: &str) {
    spin_sleep_ms(200);
    kprintln!("Welcome to my shell");
    
    let mut input_buf = [0 as u8; 512];
    
}

fn recieve_line(buf:&[u8]) {
    
}