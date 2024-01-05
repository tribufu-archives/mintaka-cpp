// Copyright (c) Tribufu. All Rights Reserved.

use mintaka::ffi::{c_char, to_string};
use mintaka::prelude::*;

#[no_mangle]
pub extern "C" fn mintaka_log_init() {
    log::init();
}

#[no_mangle]
pub extern "C" fn mintaka_log_error(message: *const c_char) {
    log::error!("{}", to_string(message));
}

#[no_mangle]
pub extern "C" fn mintaka_log_warn(message: *const c_char) {
    log::warn!("{}", to_string(message));
}

#[no_mangle]
pub extern "C" fn mintaka_log_info(message: *const c_char) {
    log::info!("{}", to_string(message));
}

#[no_mangle]
pub extern "C" fn mintaka_log_debug(message: *const c_char) {
    log::debug!("{}", to_string(message));
}

#[no_mangle]
pub extern "C" fn mintaka_log_trace(message: *const c_char) {
    log::trace!("{}", to_string(message));
}
