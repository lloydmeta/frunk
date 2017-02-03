#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk: generic functional programming toolbelt for Rust
//!
//! Aims to be a collection of functional programming abstractions implemented in Rust
//! in effective, useful, and idiomatic ways. Examples of things that are included in rust are:
//!
//!   1. HLists (heterogeneously-typed lists)
//!   2. Validated (accumulator for Result)
//!   3. Semigroup
//!   4. Monoid
//!
//! Here is a small taste of what Frunk has to offer:
//!
//! ```
//! # #[macro_use] extern crate frunk;
//! # #[macro_use] extern crate frunk_core;
//! # use frunk_core::hlist::*; fn main() {
//! use frunk_core::hlist::*;
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
//! let hlist_pat!(a, b) = h;
//! assert_eq!(a, 1);
//! assert_eq!(b, "hi");
//! # }
//! ```
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/frunk)
//!   2. [Crates.io page](https://crates.io/crates/frunk)

#[macro_use]
extern crate frunk_core;
#[macro_use]
extern crate frunk_derives;

pub mod semigroup;
pub mod monoid;
pub mod validated;

pub use frunk_core::hlist::*;
// this needs to be globally imported in order for custom derives to work w/o fuss
pub use frunk_core::generic::*;

pub use frunk_derives::*;