use cortex_a::registers::*;
use tock_registers::interfaces::Readable;

/// The processing element's current privilege level.
pub fn current_privilege_level() -> &'static str {
    let el = CurrentEL.read_as_enum(CurrentEL::EL);
    match el {
        Some(CurrentEL::EL::Value::EL2) => "EL2",
        Some(CurrentEL::EL::Value::EL1) => "EL1",
        Some(CurrentEL::EL::Value::EL0) => "EL0",
        _ => "Unknown",
    }
}