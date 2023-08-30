#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk Proc Macros
//!
//! This library holds procedural macros for frunk
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/frunk)
//!   2. [Crates.io page](https://crates.io/crates/frunk)

extern crate frunk_core;
extern crate frunk_proc_macro_helpers;
extern crate proc_macro;

extern crate quote;
extern crate syn;

use frunk_proc_macro_helpers::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

mod list_builder;

/// Build a generic path that can be used for traversals
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    let path_type = build_path_type(expr);
    let ast = quote! {
        {
            let p: #path_type = ::frunk_core::path::Path::new();
            p
        }
    };
    //    println!("ast: [{}]", ast);
    TokenStream::from(ast)
}

/// Build a generic path that can be used for traversals
#[proc_macro]
#[allow(non_snake_case)]
pub fn Path(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    let path_type = build_path_type(expr);
    let ast = quote! {
        #path_type
    };
    //    println!("ast: [{}]", ast);
    TokenStream::from(ast)
}


/// Constructs a struct using an `HList`.
///
/// This trait allows you to create an instance of a struct from an HList. You keep the
/// remaining items in the `HList` after `DerivedListBuild::hl_new(...)`
///
/// # Examples
///
/// ```
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
#[proc_macro_derive(ListBuild)]
pub fn list_build(item: TokenStream) -> TokenStream {
    list_builder::list_build_inner(item)
}
