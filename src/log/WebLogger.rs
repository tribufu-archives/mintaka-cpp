// Copyright (c) Tribufu. All Rights Reserved.

use crate::Level;

pub struct Logger {}

impl Logger {
    pub fn New() -> Self {
        Self {}
    }

    #[allow(unused_mut)]
    pub fn Init(mut self) -> () {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(Level::Trace).unwrap();
    }
}
