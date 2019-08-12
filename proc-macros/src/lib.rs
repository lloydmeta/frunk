//! Module that holds procedural macros for Frunk
//!
//!
//! # Examples
//!
//! ```
//! #![feature(proc_macro_hygiene)]
//! #[macro_use] extern crate frunk;
//! #[macro_use] extern crate frunk_core;
//! # extern crate frunk_proc_macros;
//! # use frunk_proc_macros::{path, Path};
//! # fn main() {
//! #[derive(LabelledGeneric)]
//! struct Dog<'a> {
//!     name: &'a str,
//!     dimensions: Dimensions,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct Cat<'a> {
//!     name: &'a str,
//!     dimensions: Dimensions,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct Dimensions {
//!     height: usize,
//!     width: usize,
//!     unit: SizeUnit,
//! }
//!
//! #[derive(Debug, Eq, PartialEq)]
//! enum SizeUnit {
//!     Cm,
//!     Inch,
//! }
//!
//! let mut dog = Dog {
//!     name: "Joe",
//!     dimensions: Dimensions {
//!         height: 10,
//!         width: 5,
//!         unit: SizeUnit::Inch,
//!     },
//! };
//!
//! let cat = Cat {
//!     name: "Schmoe",
//!     dimensions: Dimensions {
//!         height: 7,
//!         width: 3,
//!         unit: SizeUnit::Cm,
//!     },
//! };
//!
//! // generic, re-usable paths
//! let height_lens: Path!(dimensions.height) = path!(dimensions.height);
//! let unit_lens: Path!(dimensions.unit) = path!(dimensions.unit);
//!
//! assert_eq!(*height_lens.get(&dog), 10);
//! assert_eq!(*height_lens.get(&cat), 7);
//! assert_eq!(*unit_lens.get(&dog), SizeUnit::Inch);
//! assert_eq!(*unit_lens.get(&cat), SizeUnit::Cm);
//!
//! // modify
//! *height_lens.get(&mut dog) = 13;
//! assert_eq!(*height_lens.get(&dog), 13);
//! # }
//! ```

#![feature(proc_macro_hygiene)]

#[cfg(test)]
extern crate frunk;
extern crate frunk_core;
extern crate frunk_proc_macro_helpers;
extern crate proc_macro;

extern crate quote;
extern crate syn;

use frunk_proc_macro_helpers::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

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
