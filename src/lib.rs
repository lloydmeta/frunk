#![cfg_attr(not(feature = "std"), no_std)]
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
#![cfg_attr(
    feature = "std",
    doc = r#"
Here is a small taste of what Frunk has to offer:

```
# fn main() {
use frunk::prelude::*;
use frunk::{self, hlist, hlist_pat, LabelledGeneric, monoid, Semigroup, Generic};

// Combining Monoids
let v = vec![Some(1), Some(3)];
assert_eq!(monoid::combine_all(&v), Some(4));

// HLists
let h = hlist![1, "hi"];
assert_eq!(h.len(), 2);
let hlist_pat!(a, b) = h;
assert_eq!(a, 1);
assert_eq!(b, "hi");

let h1 = hlist![Some(1), 3.3, 53i64, "hello".to_owned()];
let h2 = hlist![Some(2), 1.2, 1i64, " world".to_owned()];
let h3 = hlist![Some(3), 4.5, 54, "hello world".to_owned()];
assert_eq!(h1.combine(&h2), h3);

// Generic and LabelledGeneric-based programming
// Allows Structs to play well easily with HLists

#[derive(Generic, LabelledGeneric)]
struct ApiUser<'a> {
    FirstName: &'a str,
    LastName: &'a str,
    Age: usize,
}

#[derive(Generic, LabelledGeneric)]
struct NewUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(LabelledGeneric)]
struct SavedUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

// Instantiate a struct from an HList. Note that you can go the other way too.
let a_user: ApiUser = frunk::from_generic(hlist!["Joe", "Blow", 30]);

// Convert using Generic
let n_user: NewUser = Generic::convert_from(a_user); // done

// Convert using LabelledGeneric
//
// This will fail if the fields of the types converted to and from do not
// have the same names or do not line up properly :)
//
// Also note that we're using a helper method to avoid having to use universal
// function call syntax
let s_user: SavedUser = frunk::labelled_convert_from(n_user);

assert_eq!(s_user.first_name, "Joe");
assert_eq!(s_user.last_name, "Blow");
assert_eq!(s_user.age, 30);

// Uh-oh ! last_name and first_name have been flipped!
#[derive(LabelledGeneric)]
struct DeletedUser<'a> {
    last_name: &'a str,
    first_name: &'a str,
    age: usize,
}
// let d_user = <DeletedUser as LabelledGeneric>::convert_from(s_user); <-- this would fail at compile time :)

// This will, however, work, because we make use of the Sculptor type-class
// to type-safely reshape the representations to align/match each other.
let d_user: DeletedUser = frunk::transform_from(s_user);
assert_eq!(d_user.first_name, "Joe");
# }
```"#
)]
//!
//! ##### Transmogrifying
//!
//! Sometimes you need might have one data type that is "similar in shape" to another data type, but it
//! is similar _recursively_ (e.g. it has fields that are structs that have fields that are a superset of
//! the fields in the target type, so they are transformable recursively).  `.transform_from` can't
//! help you there because it doesn't deal with recursion, but the `Transmogrifier` can help if both
//! are `LabelledGeneric` by `transmogrify()`ing from one to the other.
//!
//! What is "transmogrifying"? In this context, it means to recursively transform some data of type A
//! into data of type B, in a typesafe way, as long as A and B are "similarly-shaped".  In other words,
//! as long as B's fields and their subfields are subsets of A's fields and their respective subfields,
//! then A can be turned into B.
//!
//! As usual, the goal with Frunk is to do this:
//! * Using stable (so no specialisation, which would have been helpful, methinks)
//! * Typesafe
//! * No usage of `unsafe`
//!
//! Here is an example:
//!
//! ```rust
//! # fn main() {
//! use frunk::LabelledGeneric;
//! use frunk::labelled::Transmogrifier;
//!
//! #[derive(LabelledGeneric)]
//! struct InternalPhoneNumber {
//!     emergency: Option<usize>,
//!     main: usize,
//!     secondary: Option<usize>,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct InternalAddress<'a> {
//!     is_whitelisted: bool,
//!     name: &'a str,
//!     phone: InternalPhoneNumber,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct InternalUser<'a> {
//!     name: &'a str,
//!     age: usize,
//!     address: InternalAddress<'a>,
//!     is_banned: bool,
//! }
//!
//! #[derive(LabelledGeneric, PartialEq, Debug)]
//! struct ExternalPhoneNumber {
//!     main: usize,
//! }
//!
//! #[derive(LabelledGeneric, PartialEq, Debug)]
//! struct ExternalAddress<'a> {
//!     name: &'a str,
//!     phone: ExternalPhoneNumber,
//! }
//!
//! #[derive(LabelledGeneric, PartialEq, Debug)]
//! struct ExternalUser<'a> {
//!     age: usize,
//!     address: ExternalAddress<'a>,
//!     name: &'a str,
//! }
//!
//! let internal_user = InternalUser {
//!     name: "John",
//!     age: 10,
//!     address: InternalAddress {
//!         is_whitelisted: true,
//!         name: "somewhere out there",
//!         phone: InternalPhoneNumber {
//!             main: 1234,
//!             secondary: None,
//!             emergency: Some(5678),
//!         },
//!     },
//!     is_banned: true,
//! };
//!
//! /// Boilerplate-free conversion of a top-level InternalUser into an
//! /// ExternalUser, taking care of subfield conversions as well.
//! let external_user: ExternalUser = internal_user.transmogrify();
//!
//! let expected_external_user = ExternalUser {
//!     name: "John",
//!     age: 10,
//!     address: ExternalAddress {
//!         name: "somewhere out there",
//!         phone: ExternalPhoneNumber {
//!             main: 1234,
//!         },
//!     }
//! };
//!
//! assert_eq!(external_user, expected_external_user);
//! # }
//! ```
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/frunk)
//!   2. [Crates.io page](https://crates.io/crates/frunk)

#[cfg(not(feature = "std"))]
extern crate core as std;

pub mod monoid;
pub mod semigroup;
#[cfg(feature = "validated")]
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
pub use crate::hlist::lift_from;
#[doc(no_inline)]
pub use crate::hlist::HCons;
#[doc(no_inline)]
pub use crate::hlist::HNil;
#[doc(no_inline)]
pub use crate::traits::Func;
#[doc(no_inline)]
pub use crate::traits::Poly;
#[doc(no_inline)]
pub use crate::traits::{ToMut, ToRef}; // useful for where bounds

#[doc(no_inline)]
pub use crate::coproduct::Coproduct;

#[doc(no_inline)]
pub use crate::generic::convert_from;
#[doc(no_inline)]
pub use crate::generic::from_generic;
#[doc(no_inline)]
pub use crate::generic::into_generic;
#[doc(no_inline)]
pub use crate::generic::map_inter;
#[doc(no_inline)]
pub use crate::generic::map_repr;
#[doc(no_inline)]
pub use crate::generic::Generic;

#[doc(no_inline)]
pub use crate::labelled::from_labelled_generic;
#[doc(no_inline)]
pub use crate::labelled::into_labelled_generic;
#[doc(no_inline)]
pub use crate::labelled::labelled_convert_from;
#[doc(no_inline)]
pub use crate::labelled::transform_from;
#[doc(no_inline)]
pub use crate::labelled::LabelledGeneric;

#[doc(no_inline)]
pub use crate::semigroup::Semigroup;

#[doc(no_inline)]
pub use crate::monoid::Monoid;

#[doc(no_inline)]
#[cfg(feature = "validated")]
pub use crate::validated::Validated;

pub mod prelude {
    //! Traits that need to be imported for the complete `frunk` experience.
    //!
    //! The intent here is that `use frunk::prelude::*` is enough to provide
    //! access to any missing methods advertised in frunk's documentation.

    #[doc(no_inline)]
    pub use crate::hlist::HList; // for LEN
    #[doc(no_inline)]
    pub use crate::hlist::LiftFrom;
    #[doc(no_inline)]
    pub use crate::hlist::LiftInto;

    #[doc(no_inline)]
    #[cfg(feature = "validated")]
    pub use crate::validated::IntoValidated;
}
