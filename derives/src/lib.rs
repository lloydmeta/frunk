#![recursion_limit = "128"]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![cfg_attr(feature = "nightly", feature(proc_macro_diagnostic))]
//! Frunk Derives
//!
//! This library holds logic for the nice custom derives in Frunk.
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/frunk)
//!   2. [Crates.io page](https://crates.io/crates/frunk)

extern crate frunk_proc_macro_helpers;
extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

mod derive_generic;
use crate::derive_generic::impl_generic;

mod derive_labelled_generic;
use crate::derive_labelled_generic::impl_labelled_generic;

mod list_builder;

use quote::ToTokens;

/// Derives a Generic instance based on HList for
/// a given Struct or Tuple Struct
#[proc_macro_derive(Generic)]
pub fn generic(input: TokenStream) -> TokenStream {
    // Build the impl
    let gen = impl_generic(input);
    //    println!("{}", gen);
    // Return the generated impl
    gen.into_token_stream().into()
}

/// Derives a Generic instance based on Field + HList for
/// a given Struct (Tuple Structs not supported because they have
/// no labels)
///
/// There *may* be problems if your field names contain certain characters.
/// This can be solved by adding letters to the create_enums_for! macro invocation
/// in frunk_core::labelled via a PR :)
#[proc_macro_derive(LabelledGeneric)]
pub fn labelled_generic(input: TokenStream) -> TokenStream {
    // Build the impl
    let gen = impl_labelled_generic(input);
    //    println!("{}", gen);
    // Return the generated impl
    gen.into_token_stream().into()
}

/// Constructs a struct using an `HList`.
///
/// This trait allows you to create an instance of a struct from an HList. You keep the
/// remaining items in the `HList` after `DerivedListBuild::hl_new(...)`
///
/// # Examples
///
/// ```ignore
/// use frunk::hlist::HList;
/// use frunk::hlist;
///
/// #[derive(Debug, Eq, PartialEq, frunk::hlist::ListBuild)]
/// struct ListConstructed {
///     field0: bool,
///     field1: u8,
///     #[list_build_ignore]
///     fn_built: &'static str,
/// }
///
/// // Create a struct manually for comparison
/// let manually_made = ListConstructed {
///     field0: true,
///     field1: 3,
///     fn_built: "passed_in",
/// };
///
/// // Use `hl_new` to construct a struct and remaining HList
/// let (built, list): (ListConstructed, HList!(u32)) =
///     ListConstructed::hl_new(hlist![true, 3u8, 42u32], "passed_in");
///
/// assert_eq!(built, manually_made);
/// assert_eq!(list, hlist![42u32]);
/// ```
#[proc_macro_derive(ListBuild, attributes(list_build_ignore, plucker))]
pub fn list_build(item: TokenStream) -> TokenStream {
    list_builder::list_build_inner(item)
}
