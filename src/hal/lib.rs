// Copyright (c) Tribufu. All Rights Reserved.

#[cfg(target_arch = "wasm32")]
compile_error!("This crate does not support wasm32");

pub mod cpu;
pub mod mem;
pub mod sys;
