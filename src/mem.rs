// Copyright (c) Tribufu. All Rights Reserved.

use mintaka::hal::mem;

/// Returns the total memory of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_total_memory() -> u64 {
    mem::get_total_memory()
}

/// Returns the free memory of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_free_memory() -> u64 {
    mem::get_free_memory()
}

/// Returns the used memory of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_used_memory() -> u64 {
    mem::get_used_memory()
}

/// Returns the total swap of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_total_swap() -> u64 {
    mem::get_total_swap()
}

/// Returns the free swap of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_free_swap() -> u64 {
    mem::get_free_swap()
}

/// Returns the used swap of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_used_swap() -> u64 {
    mem::get_used_swap()
}
