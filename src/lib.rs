//! Frunk: generic functional programming toolbelt for Rust
//!
//! Aims to be a collection of functional programming abstractions implemented in Rust
//! in effective, useful, and idiomatic ways.
//!
//! Here is a small taste of what Frunk has to offer:
//!
//! ```
//! # #[macro_use] extern crate frunk; use frunk::hlist::*; fn main() {
//! use frunk::hlist::*;
//! use frunk::monoid::*;
//! use frunk::validated::*;
//!
//! // Combining Monoids
//! let v = vec![Some(1), Some(3)];
//! assert_eq!(combine_all(&v), Some(4));
//!
//! // HLists
//! let h = hlist![1, "hi"];
//! assert_eq!(h.length(), 2);
//! let (a, b) = h.into_tuple2();
//! assert_eq!(a, 1);
//! assert_eq!(b, "hi");
//! # }
//! ```

pub mod semigroup;
pub mod monoid;
#[macro_use]pub mod hlist;
pub mod validated;
