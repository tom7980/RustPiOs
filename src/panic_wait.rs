use crate::arch::cpu;
use core::{fmt, panic::PanicInfo};
use crate::pi::console;

fn _panic_print(args: fmt::Arguments) {
    use fmt::Write;

    unsafe { console::panic_console().write_fmt(args).unwrap() };
}

#[macro_export]
macro_rules! panic_println {
    ($($arg:tt)*) => ({
        _panic_print(format_args_nl!($($arg)*));
    })
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_println!("\nKernel panicked: {}", info);

    loop{}
}

#[lang = "eh_personality"]
fn eh_personality() {}
