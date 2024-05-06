#![cfg_attr(not(feature = "std"), no_std)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk Core
//!
//! This library forms the core of Frunk. It should ideally be minimalistic,
//! containing only the fundamental building blocks of generic programming.
//!
//! # Examples
//!
//! ```
//! # use frunk_core::hlist::*;
//! # use frunk_core::{hlist, HList};
//! # fn main() {
//!
//! let h = hlist![1, false, 42f32];
//! let folded = h.foldr(hlist![|acc, i| i + acc,
//!     |acc, _| if acc > 42f32 { 9000 } else { 0 },
//!     |acc, f| f + acc],
//!     1f32);
//! assert_eq!(folded, 9001);
//!
//! // Reverse
//! let h1 = hlist![true, "hi"];
//! assert_eq!(h1.into_reverse(), hlist!["hi", true]);
//!
//! // foldr (foldl also available)
//! let h2 = hlist![1, false, 42f32];
//! let folded = h2.foldr(
//!             hlist![|acc, i| i + acc,
//!                    |acc, _| if acc > 42f32 { 9000 } else { 0 },
//!                    |acc, f| f + acc],
//!             1f32
//!     );
//! assert_eq!(folded, 9001);
//!
//! // Mapping over an HList
//! let h3 = hlist![9000, "joe", 41f32];
//! let mapped = h3.to_ref().map(hlist![|&n| n + 1,
//!                               |&s| s,
//!                               |&f| f + 1f32]);
//! assert_eq!(mapped, hlist![9001, "joe", 42f32]);
//!
//! // Plucking a value out by type
//! let h4 = hlist![1, "hello", true, 42f32];
//! let (t, remainder): (bool, _) = h4.pluck();
//! assert!(t);
//! assert_eq!(remainder, hlist![1, "hello", 42f32]);
//!
//! // Resculpting an HList
//! let h5 = hlist![9000, "joe", 41f32, true];
//! let (reshaped, remainder2): (HList![f32, i32, &str], _) = h5.sculpt();
//! assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
//! assert_eq!(remainder2, hlist![true]);
//! # }
//! ```
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/frunk)
//!   2. [Crates.io page](https://crates.io/crates/frunk)

#[cfg(not(feature = "std"))]
extern crate core as std;

#[macro_use]
mod macros;

pub mod coproduct;
pub mod generic;
pub mod hlist;
pub mod indices;
pub mod labelled;
pub mod path;
pub mod traits;
mod tuples;

#[cfg(test)]
mod test_structs;
