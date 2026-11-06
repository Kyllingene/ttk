use std::fmt;

pub fn ok() -> ! {
    drop(cod::guard::Reset);
    std::process::exit(0);
}

pub fn abort(msg: impl fmt::Display) -> ! {
    with(1, msg)
}

pub fn with(code: i32, msg: impl fmt::Display) -> ! {
    crate::print::err(1, msg);
    drop(cod::guard::Reset);
    std::process::exit(code);
}

#[must_use = "Hold onto this to ensure the terminal is reset when the program exits"]
pub fn guard() -> cod::guard::Reset {
    cod::guard::Reset
}

#[macro_export]
macro_rules! abort {
    ($($args:tt)*) => {
        $crate::exit::with!(1, $($args)*)
    };
}

#[macro_export]
macro_rules! exit {
    ($code:expr, $($args:tt)*) => {
        $crate::exit::with(format_args!($($args)*))
    };
}

pub use crate::abort;
pub use crate::exit as with;
