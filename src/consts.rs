// Copyright (c) Tribufu. All Rights Reserved.

use mintaka::ffi::{c_char, from_string};
use mintaka::prelude::*;

/// Get the current version of this library.
#[no_mangle]
pub extern "C" fn mintaka_get_version() -> *const c_char {
    from_string(MINTAKA_VERSION.to_string())
}

/// The target triple that is being compiled for.
#[no_mangle]
pub extern "C" fn mintaka_get_target_triple() -> *const c_char {
    from_string(TARGET_TRIPLE.to_string())
}

/// The LLVM version of the Rust compiler.
#[no_mangle]
pub extern "C" fn mintaka_get_llvm_version() -> *const c_char {
    from_string(LLVM_VERSION.to_string())
}

/// The version of the Rust compiler.
#[no_mangle]
pub extern "C" fn mintaka_get_rust_version() -> *const c_char {
    from_string(RUSTC_VERSION.to_string())
}

/// The release channel of the Rust compiler.
#[no_mangle]
pub extern "C" fn mintaka_get_rust_channel() -> *const c_char {
    from_string(RUSTC_CHANNEL.to_string())
}

/// The current cargo build profile.
#[no_mangle]
pub extern "C" fn mintaka_get_cargo_profile() -> *const c_char {
    from_string(CARGO_PROFILE.to_string())
}

/// The timestamp that has been compiled.
#[no_mangle]
pub extern "C" fn mintaka_get_build_timestamp() -> *const c_char {
    from_string(BUILD_TIMESTAMP.to_string())
}
