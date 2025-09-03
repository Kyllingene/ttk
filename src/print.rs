use cod::prelude::*;

use std::fmt;

pub const INFO_HEADER: Header = Header { ch: '=', color: 6 };

pub const DBG_HEADER: Header = Header { ch: '-', color: 4 };

pub const WARN_HEADER: Header = Header { ch: '*', color: 3 };

pub const ERR_HEADER: Header = Header { ch: '!', color: 1 };

pub const PROMPT_HEADER: Header = Header { ch: '#', color: 5 };

pub struct Header {
    pub ch: char,
    pub color: u8,
    // pub bold: bool,
}

impl Header {
    pub fn print(&self, level: usize, msg: impl fmt::Display) {
        print!(" ");
        color::with::fg(self.color, || {
            (0..level).for_each(|_| print!("{}", self.ch));
        });
        print!(" {msg}");
    }

    pub fn println(&self, level: usize, msg: impl fmt::Display) {
        print!(" ");
        color::with::fg(self.color, || {
            (0..level).for_each(|_| print!("{}", self.ch));
        });
        println!(" {msg}");
    }

    pub fn print_full(&self, level: usize, msg: impl fmt::Display) {
        print!(" ");
        color::with::fg(self.color, || {
            (0..level).for_each(|_| print!("{}", self.ch));
            print!(" {msg}");
        });
    }

    pub fn println_full(&self, level: usize, msg: impl fmt::Display) {
        print!(" ");
        color::with::fg(self.color, || {
            (0..level).for_each(|_| print!("{}", self.ch));
            println!(" {msg}");
        });
    }
}

pub fn info(level: usize, msg: impl fmt::Display) {
    INFO_HEADER.println(level, msg)
}

pub fn dbg(level: usize, msg: impl fmt::Display) {
    DBG_HEADER.println_full(level, msg)
}

pub fn warn(level: usize, msg: impl fmt::Display) {
    WARN_HEADER.println_full(level, msg)
}

pub fn err(level: usize, msg: impl fmt::Display) {
    ERR_HEADER.println_full(level, msg)
}

pub fn prompt(level: usize, msg: impl fmt::Display) {
    PROMPT_HEADER.println_full(level, msg)
}

pub fn ln() {
    println!();
}

#[macro_export]
macro_rules! info {
    ($level:expr, $($args:tt)*) => {
        $crate::print::info($level, format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! dbg {
    ($level:expr, $($args:tt)*) => {
        $crate::print::dbg($level, format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($level:expr, $($args:tt)*) => {
        $crate::print::warn($level, format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! err {
    ($level:expr, $($args:tt)*) => {
        $crate::print::err($level, format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! prompt {
    ($level:expr, $($args:tt)*) => {
        $crate::print::prompt($level, format_args!($($args)*))
    };
}

pub use crate::{dbg, err, info, prompt, warn};
