extern crate proc_macro;
#[macro_use]
extern crate frunk_core;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

mod common;
use common::to_ast;

mod derive_generic;
use derive_generic::impl_generic;

mod derive_labelled_generic;
use derive_labelled_generic::impl_labelled_generic;

#[proc_macro_derive(Generic)]
pub fn generic(input: TokenStream) -> TokenStream {
    // Build the impl
    let gen = impl_generic(&to_ast(&input));
    //    println!("{}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(LabelledGeneric)]
pub fn labelled_generic(input: TokenStream) -> TokenStream {
    // Build the impl
    let gen = impl_labelled_generic(&to_ast(&input));
    //    println!("{}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}