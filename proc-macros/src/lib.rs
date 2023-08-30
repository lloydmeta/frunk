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
#[proc_macro_derive(ListBuild)]
pub fn list_build(item: TokenStream) -> TokenStream {
    list_builder::list_build_inner(item)
}
