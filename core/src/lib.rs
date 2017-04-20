#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk Core
//!
//! This library forms the core of Frunk. It should ideally be minimalistic,
//! containing only the fundamental building blocks of generic programming.
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate frunk_core;
//! # use frunk_core::hlist::*;
//! # fn main() {
//!
//! let h = hlist![1, false, 42f32];
//! let folded = h.foldr(hlist![|i, acc| i + acc,
//!     |_, acc| if acc > 42f32 { 9000 } else { 0 },
//!     |f, acc| f + acc],
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
//!             hlist![|i, acc| i + acc,
//!                    |_, acc| if acc > 42f32 { 9000 } else { 0 },
//!                    |f, acc| f + acc],
//!             1f32
//!     );
//! assert_eq!(folded, 9001);
//!
//! // Mapping over an HList
//! let h3 = hlist![9000, "joe", 41f32];
//! let mapped = (&h3).map(hlist![|&n| n + 1,
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
//! let (reshaped, remainder2): (Hlist![f32, i32, &str], _) = h5.sculpt();
//! assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
//! assert_eq!(remainder2, hlist![true]);
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
