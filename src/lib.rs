//! # Hash Consing Library
//!
//! [![crates.io](https://img.shields.io/crates/v/hash_cons.svg)](https://crates.io/crates/hash_cons)
//! [![docs.rs](https://docs.rs/hash_cons/badge.svg)](https://docs.rs/hash_cons)
//!
//! The Hash Consing Library (`hash_cons`) is a Rust library that implements efficient hash consing techniques,
//! making it a robust choice for both multi-threaded and single-threaded Rust applications.
//! At its core, the library is designed to focus on type-safety, efficiency, and zero-cost abstractions.
//!
//! ## Efficiency and Idiomatic Rust
//!
//! This library is designed to be inherently idiomatic to Rust, ensuring efficient memory management and optimal performance. It utilizes Rust's unique ownership and borrowing rules to manage immutable data structures. The key feature is its tunable memory management: when a value is no longer referred to anywhere in your program, the library automatically deallocates it due to enabled `auto_cleanup` feature by default, preventing memory leaks and optimizing resource usage. This makes it an excellent tool for applications where duplicate data structures are common, ensuring memory efficiency.
//!
//! ## Features
//!
//! - **auto_cleanup**: Enabled by default, this feature allows the library to automatically clean up and
//!  manage memory efficiently by removing unused entries.
//! - **single-threaded**: Disabled by default, enable this feature if you are working in a single-threaded environment
//!   or if you want to manage synchronization yourself.
//!
//! ## Usage
//!
//! To integrate `hash_cons` into your project, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! hash_cons = "0.2.0"  # Replace with the actual version
//! ```
//!
//! By default, the library operates in a multi-threaded environment with auto_cleanup enabled. For single-threaded support, enable the `single-threaded` feature:
//!
//! ```toml
//! # Default multi-threaded with auto_cleanup enabled
//! hash_cons = "0.2.0"
//!
//! # For single-threaded environments with auto_cleanup enabled
//! hash_cons = { version = "0.2.0", features = ["single-threaded"] }
//!
//! # For multi-threaded environments with auto_cleanup disabled
//! hash_cons = { version = "0.2.0", default-features = false }
//!
//! # For single-threaded environments with auto_cleanup disabled
//! hash_cons = { version = "0.2.0", default-features = false, features = ["single-threaded"] }
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use hash_cons::{HcTable, Hc};
//!
//! /// A simple boolean expression
//! #[derive(Hash, PartialEq, Eq)]
//! enum BoolExpr {
//!     Const(bool),
//!     And(Hc<BoolExpr>, Hc<BoolExpr>),
//!     Or(Hc<BoolExpr>, Hc<BoolExpr>),
//!     Not(Hc<BoolExpr>),
//! }
//!
//!
//! /// Multi-threaded usage with auto_cleanup enabled by default
//!
//! #[cfg(all(feature = "auto-cleanup", not(feature = "single-threaded")))]
//! fn main() {
//!     use std::thread;
//!     let table: HcTable<BoolExpr> = HcTable::new();
//!     let table_clone = table.clone();
//!     let thread_handle_hc_false = thread::spawn(move || {
//!         table_clone.hashcons(BoolExpr::Const(false))
//!     });
//!     let hc_false: Hc<BoolExpr> = thread_handle_hc_false.join().unwrap(); // Safe for concurrent use across threads
//!     drop(hc_false);
//!     assert_eq!(table.len(), 0);
//! }
//!
//!
//! /// Multi-threaded usage with auto_cleanup disabled
//!
//! #[cfg(all(not(feature = "auto-cleanup"), not(feature = "single-threaded")))]
//! fn main() {
//!     use std::thread;
//!     let table: HcTable<BoolExpr> = HcTable::new();
//!     let table_clone = table.clone();
//!     let thread_handle_hc_true = thread::spawn(move || {
//!         table_clone.hashcons(BoolExpr::Const(true))
//!     });
//!     let hc_true: Hc<BoolExpr> = thread_handle_hc_true.join().unwrap(); // Safe for concurrent use across threads
//!     assert_eq!(table.len(), 1);
//!     drop(hc_true);
//!     table.cleanup(); //hc_true is removed from the table after it has been dropped and `cleanup()` is called on the table.
//!     assert_eq!(table.len(), 0);
//! }
//!
//!
//! /// Single-threaded usage with auto_cleanup enabled
//!
//! #[cfg(all(feature = "auto-cleanup", feature = "single-threaded"))]
//! fn main() {
//!     let table: HcTable<BoolExpr> = HcTable::new();
//!     let const_true = BoolExpr::Const(true);
//!     let hc_true: Hc<BoolExpr> = table.hashcons(const_true);
//!     drop(hc_true);// hc_true is automatically removed from the table when dropped from the memory
//!     assert_eq!(table.len(), 0);
//! }
//!
//!
//! /// Single-threaded usage with auto_cleanup disabled
//!
//! #[cfg(all(feature = "single-threaded", not(feature = "auto-cleanup")))]
//! fn main() {
//!     let table: HcTable<BoolExpr> = HcTable::new();
//!     let const_true = BoolExpr::Const(true);
//!     let hc_true: Hc<BoolExpr> = table.hashcons(const_true);
//!     drop(hc_true);
//!     assert_eq!(table.len(), 1);
//!     table.cleanup(); //hc_true is removed from the table after it has been dropped and `cleanup()` is called on the table.
//!     assert_eq!(table.len(), 0);
//! }
//! ```
//!

#[cfg(feature = "single-threaded")]
pub mod single_threaded;

#[cfg(feature = "single-threaded")]
pub use single_threaded::*;

#[cfg(not(feature = "single-threaded"))]
pub mod thread_safe;

#[cfg(not(feature = "single-threaded"))]
pub use thread_safe::*;