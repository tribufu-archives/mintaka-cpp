// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use mintaka::ffi::{c_char, from_string};
use mintaka::hal::sys;
use std::ptr;

/// Returns the uptime of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_system_uptime() -> u64 {
    sys::get_system_uptime()
}

/// Returns the OS version of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_system_version() -> *const c_char {
    if let Some(string) = sys::get_system_version() {
        from_string(string)
    } else {
        ptr::null()
    }
}

/// Returns the kernel version of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_kernel_version() -> *const c_char {
    if let Some(string) = sys::get_kernel_version() {
        from_string(string)
    } else {
        ptr::null()
    }
}

/// Returns the name of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_system_name() -> *const c_char {
    if let Some(string) = sys::get_system_name() {
        from_string(string)
    } else {
        ptr::null()
    }
}

/// Returns the edition of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_system_edition() -> *const c_char {
    if let Some(string) = sys::get_system_edition() {
        from_string(string)
    } else {
        ptr::null()
    }
}

/// Returns the host name of the current system.
#[no_mangle]
pub extern "C" fn mintaka_get_host_name() -> *const c_char {
    if let Some(string) = sys::get_host_name() {
        from_string(string)
    } else {
        ptr::null()
    }
}
