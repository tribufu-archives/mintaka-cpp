// Copyright (c) Tribufu. All Rights Reserved.

use mintaka::ffi::{c_char, from_string};
use mintaka::hal::cpu;

/// Returns the number of logical cores of the current CPU.
#[no_mangle]
pub extern "C" fn mintaka_get_logical_cores() -> u64 {
    cpu::get_logical_cores() as u64
}

/// Returns the number of physical cores of the current CPU.
#[no_mangle]
pub extern "C" fn mintaka_get_physical_cores() -> u64 {
    cpu::get_physical_cores() as u64
}

/// Returns the model of the current CPU.
#[no_mangle]
pub extern "C" fn mintaka_get_cpu_model() -> *const c_char {
    from_string(cpu::get_processor_model().to_string())
}

/// Returns the vendor of the current CPU.
#[no_mangle]
pub extern "C" fn mintaka_get_cpu_vendor() -> *const c_char {
    from_string(cpu::get_processor_vendor().to_string())
}
