// Copyright (c) Tribufu. All Rights Reserved.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LoggerOptions {
    pub colored: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for LoggerOptions {
    fn default() -> Self {
        Self { colored: true }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for LoggerOptions {
    fn default() -> Self {
        Self { colored: false }
    }
}
