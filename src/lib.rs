//! A collection of utilities for simple, uniform TUIs.
//!
//! # Examples
//!
//! ```rust,no_run
#![doc = include_str!("../examples/demo.rs")]
//! ```

// TODO: consider a scope- and/or guard-based approach to automating nesting

pub mod cmd;
pub mod display;
pub mod exit;
pub mod print;
pub mod query;
pub mod logfile;

/// Imports all the sub-modules.
pub mod prelude {
    pub use crate::{cmd, display, exit, print, query, logfile};
}
