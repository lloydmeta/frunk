#![recursion_limit = "128"]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
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
use derive_generic::impl_generic;

mod derive_labelled_generic;
use derive_labelled_generic::impl_labelled_generic;

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
