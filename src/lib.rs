//! A collection of utilities for simple, uniform TUIs.
//!
//! # Examples
//!
//! ```rust,no_run
#![doc = include_str!("../examples/demo.rs")]
//! ```

// TODO: consider a scope- and/or guard-based approach to automating nesting

pub mod exit;
pub mod print;
pub mod query;

/// Imports all the sub-modules.
pub mod prelude {
    pub use crate::{exit, print, query};
}
