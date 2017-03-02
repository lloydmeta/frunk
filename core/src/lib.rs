#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk Core
//!
//! This library forms the core of Frunk. It should ideally be minimalistic,
//! containing only the fundamental building blocks of generic programming.
//!
//! ```
//! # #[macro_use] extern crate frunk_core;
//! # use frunk_core::hlist::*;
//! # use frunk_core::hlist::*;
//! # fn main() {
//!
//! let h = hlist![1, false, 42f32];
//! let folded = h.foldr(hlist![|i, acc| i + acc,
//!     |_, acc| if acc > 42f32 { 9000 } else { 0 },
//!     |f, acc| f + acc],
//!     1f32);
//! assert_eq!(folded, 9001)
//! # }
//! ```
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/frunk)
//!   2. [Crates.io page](https://crates.io/crates/frunk)

#[macro_use]
pub mod hlist;
pub mod generic;
pub mod labelled;
