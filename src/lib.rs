#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk: generic functional programming toolbelt for Rust
//!
//! Aims to be a collection of functional programming abstractions implemented in Rust
//! in effective, useful, and idiomatic ways. Examples of things that are included in rust are:
//!
//!   1. HLists (heterogeneously-typed lists)
//!   2. LabelledGeneric, and Generic
//!   3. Coproduct
//!   4. Validated (accumulator for Result)
//!   5. Semigroup
//!   6. Monoid
//!
//! Here is a small taste of what Frunk has to offer:
//!
//! ```
//! # #[macro_use] extern crate frunk;
//! # #[macro_use] extern crate frunk_core;
//! # fn main() {
//! use frunk::prelude::*;
//! use frunk::{self, monoid, Semigroup, Generic};
//!
//! // Combining Monoids
//! let v = vec![Some(1), Some(3)];
//! assert_eq!(monoid::combine_all(&v), Some(4));
//!
//! // HLists
//! let h = hlist![1, "hi"];
//! assert_eq!(h.len(), 2);
//! let hlist_pat!(a, b) = h;
//! assert_eq!(a, 1);
//! assert_eq!(b, "hi");
//!
//! let h1 = hlist![Some(1), 3.3, 53i64, "hello".to_owned()];
//! let h2 = hlist![Some(2), 1.2, 1i64, " world".to_owned()];
//! let h3 = hlist![Some(3), 4.5, 54, "hello world".to_owned()];
//! assert_eq!(h1.combine(&h2), h3);
//!
//! // Generic and LabelledGeneric-based programming
//! // Allows Structs to play well easily with HLists
//!
//! #[derive(Generic, LabelledGeneric)]
//! struct ApiUser<'a> {
//!     FirstName: &'a str,
//!     LastName: &'a str,
//!     Age: usize,
//! }
//!
//! #[derive(Generic, LabelledGeneric)]
//! struct NewUser<'a> {
//!     first_name: &'a str,
//!     last_name: &'a str,
//!     age: usize,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct SavedUser<'a> {
//!     first_name: &'a str,
//!     last_name: &'a str,
//!     age: usize,
//! }
//!
//! // Instantiate a struct from an HList. Note that you can go the other way too.
//! let a_user: ApiUser = frunk::from_generic(hlist!["Joe", "Blow", 30]);
//!
//! // Convert using Generic
//! let n_user: NewUser = Generic::convert_from(a_user); // done
//!
//! // Convert using LabelledGeneric
//! //
//! // This will fail if the fields of the types converted to and from do not
//! // have the same names or do not line up properly :)
//! //
//! // Also note that we're using a helper method to avoid having to use universal
//! // function call syntax
//! let s_user: SavedUser = frunk::labelled_convert_from(n_user);
//!
//! assert_eq!(s_user.first_name, "Joe");
//! assert_eq!(s_user.last_name, "Blow");
//! assert_eq!(s_user.age, 30);
//!
//! // Uh-oh ! last_name and first_name have been flipped!
//! #[derive(LabelledGeneric)]
//! struct DeletedUser<'a> {
//!     last_name: &'a str,
//!     first_name: &'a str,
//!     age: usize,
//! }
//! // let d_user = <DeletedUser as LabelledGeneric>::convert_from(s_user); <-- this would fail at compile time :)
//!
//! // This will, however, work, because we make use of the Sculptor type-class
//! // to type-safely reshape the representations to align/match each other.
//! let d_user: DeletedUser = frunk::transform_from(s_user);
//! assert_eq!(d_user.first_name, "Joe");
//! # }
//! ```
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/frunk)
//!   2. [Crates.io page](https://crates.io/crates/frunk)

#[allow(unused_imports)]
#[macro_use]
extern crate frunk_core;
#[allow(unused_imports)]
#[macro_use]
extern crate frunk_derives;

pub mod semigroup;
pub mod monoid;
pub mod validated;

pub use frunk_core::*;
pub use frunk_derives::*;

// Root-level reexports so that users don't need to guess where things are located.
//
// Things to re-export:
//
// * Datatypes and free functions intended for human consumption.
//   * **Exception:** things that benefit from being namespaced,
//     like `frunk::semigroup::Any`
//
// * Traits that users ought to care enough about to `use` it **by name:**
//   * ...because users might want to `impl` it for their own types
//   * ...because it shows up in lots of `where` bounds
//   * NOT simply because it extends existing types with useful methods
//     (that's what the prelude is for!)

// NOTE: without `#[doc(no_inline)]`, rustdoc will generate two separate pages for
//       each item (one in `frunk::` and another in `frunk_core::module::`).
//       Hyperlinks will be broken for the ones in `frunk::`, so we need to prevent it.

#[doc(no_inline)]
pub use hlist::HNil;
#[doc(no_inline)]
pub use hlist::HCons;
#[doc(no_inline)]
pub use hlist::lift_from;

#[doc(no_inline)]
pub use coproduct::Coproduct;

#[doc(no_inline)]
pub use generic::Generic;
#[doc(no_inline)]
pub use generic::from_generic;
#[doc(no_inline)]
pub use generic::into_generic;
#[doc(no_inline)]
pub use generic::convert_from;

#[doc(no_inline)]
pub use labelled::LabelledGeneric;
#[doc(no_inline)]
pub use labelled::from_labelled_generic;
#[doc(no_inline)]
pub use labelled::into_labelled_generic;
#[doc(no_inline)]
pub use labelled::labelled_convert_from;
#[doc(no_inline)]
pub use labelled::transform_from;

#[doc(no_inline)]
pub use semigroup::Semigroup;

#[doc(no_inline)]
pub use monoid::Monoid;

#[doc(no_inline)]
pub use validated::Validated;

pub mod prelude {
    //! Traits that need to be imported for the complete `frunk` experience.
    //!
    //! The intent here is that `use frunk::prelude::*` is enough to provide
    //! access to any missing methods advertised in frunk's documentation.

    #[doc(no_inline)]
    pub use hlist::HList; // for LEN
    #[doc(no_inline)]
    pub use hlist::LiftFrom;
    #[doc(no_inline)]
    pub use hlist::LiftInto;

    #[doc(no_inline)]
    pub use validated::IntoValidated;
}
