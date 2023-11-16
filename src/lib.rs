//! # Hash Consing Library
//!
//! This library provides implementations for hash consing in both
//! single-threaded and multi-threaded environments.
//!
//! ## Features
//! - `single-threaded`: For single-threaded environments.
//! - `thread-safe`: For thread-safe environments.

#[cfg(feature = "single-threaded")]
pub mod single_threaded;

#[cfg(feature = "thread-safe")]
pub mod thread_safe;

#[cfg(feature = "single-threaded")]
pub use single_threaded::*;

#[cfg(feature = "thread-safe")]
pub use thread_safe::*;
