extern crate proc_macro;
#[macro_use]
extern crate frunk_core;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

mod common;

mod derive_generic;
use derive_generic::impl_generic;

mod derive_labelled_generic;
use derive_labelled_generic::impl_labelled_generic;

/// Derives a Generic instance based on HList for
/// a given Struct or Tuple Struct
#[proc_macro_derive(Generic)]
pub fn generic(input: TokenStream) -> TokenStream {
    // Build the impl
    let gen = impl_generic(input);
    //    println!("{}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}

/// Derives a Generic instance based on Labelled + HList for
/// a given Struct (Tuple Structs not supported because they have
/// no labels)
///
/// There *may* be problems if you certain characters in your
/// field names. This can be solved by adding letter to the
/// create_enums_for! macro invocation in frunk_core::labelled via a PR :)
#[proc_macro_derive(LabelledGeneric)]
pub fn labelled_generic(input: TokenStream) -> TokenStream {
    // Build the impl
    let gen = impl_labelled_generic(input);
    //    println!("{}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}