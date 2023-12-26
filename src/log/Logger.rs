// Copyright (c) Tribufu. All Rights Reserved.

use crate::colors::*;
use crate::{Level, FILTER_ENV};
use chrono::prelude::*;
use env_logger::{fmt::Color, Builder};
use std::io::Write;

pub struct Logger {
    builder: Builder,
    //options: LoggerOptions,
}

impl Logger {
    pub fn New(/*options: LoggerOptions*/) -> Self {
        let builder = Builder::from_env(FILTER_ENV);

        Self { builder }
    }

    /*
    pub fn GetOptions(&self) -> LoggerOptions {
        self.options
    }

    pub fn SetOptions(&mut self, options: LoggerOptions) {
        self.options = options;
    }
    */

    pub fn init(mut self) {
        self.builder
            .format(|fmt, record| {
                let mut style = fmt.style();

                match record.level() {
                    Level::Error => style.set_color(Color::Ansi256(RED)),
                    Level::Warn => style.set_color(Color::Ansi256(YELLOW)),
                    Level::Info => style.set_color(Color::Ansi256(GREEN)),
                    Level::Debug => style.set_color(Color::Ansi256(WHITE)),
                    Level::Trace => style.set_color(Color::Ansi256(BRIGHT_BLACK)),
                };

                let line = format!(
                    "[{}] [{}]: {}",
                    Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    record.level(),
                    record.args()
                );

                writeln!(fmt, "{}", style.value(line))
            })
            .init();
    }
}
