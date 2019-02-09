#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk Proc Macro internals
//!
//! This library holds common logic for procedural macros used by frunk
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/frunk)
//!   2. [Crates.io page](https://crates.io/crates/frunk)

extern crate frunk_core;
extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::ToTokens;
use quote::__rt::Span;
use syn::{DeriveInput, Expr, Ident, Member};

/// These are assumed to exist as enums in frunk_core::labelled
const ALPHA_CHARS: &'static [char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// These are assumed to exist as enums in frunk_core::labelled as underscore prepended enums
const UNDERSCORE_CHARS: &'static [char] = &['_', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Parses a TokenStream (usually received as input into a
/// custom derive function), into a syn MacroInput AST,
/// which is nice.
pub fn to_ast(input: TokenStream) -> DeriveInput {
    // Parse the string representation
    syn::parse(input).unwrap()
}

/// Returns an Ident
pub fn call_site_ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

/// Given a identifiers, creates an AST for building an HList (HCons)
/// using those identifiers as accessors.
///
/// Subsequently, this same function can be used for pattern matching too!
pub fn build_hcons_constr(accessors: &Vec<Ident>) -> impl ToTokens {
    match accessors.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let h = accessors[0].clone();
            quote! { ::frunk_core::hlist::HCons{ head: #h, tail: ::frunk_core::hlist::HNil } }
        }
        _ => {
            let h = accessors[0].clone();
            let tail = accessors[1..].to_vec();
            let hlist_tail = build_hcons_constr(&tail);
            quote! { ::frunk_core::hlist::HCons{ head: #h, tail: #hlist_tail }}
        }
    }
}

/// Given an Ident returns an AST for its type level representation based on the
/// enums generated in frunk_core::labelled.
///
/// For example, given first_name, returns an AST for Hlist!(f,i,r,s,t,__,n,a,m,e)
pub fn build_type_level_name_for(ident: &Ident) -> impl ToTokens {
    let as_string = ident.to_string();
    let name = as_string.as_str();
    let name_as_idents: Vec<Ident> = name.chars().flat_map(|c| encode_as_ident(&c)).collect();
    let name_as_tokens: Vec<_> = name_as_idents
        .iter()
        .map(|ident| {
            quote! { ::frunk_core::labelled::chars::#ident }
        })
        .collect();
    quote! { (#(#name_as_tokens),*) }
}

/// Given a char, encodes it as a vector of Ident
///
/// Takes care of checking to see whether the char can be used as is,
/// or needs to be encoded as an underscored character (_ and numbers),
/// or needs to be encoded as a unicode.
///
/// This method assumes that _uc and uc_ are in frunk_core::labelled as enums
fn encode_as_ident(c: &char) -> Vec<Ident> {
    if ALPHA_CHARS.contains(c) {
        vec![call_site_ident(&c.to_string())]
    } else if UNDERSCORE_CHARS.contains(c) {
        vec![call_site_ident(&format!("_{}", c))]
    } else {
        // UTF escape and get the hexcode
        let as_unicode = c.escape_unicode();
        // as_unicode can be multiple unicode codepoints encoded as \u{2764}\u{fe0f} (❤️)
        // so we filter on alphanumeric to get just u's as a delimiters along with the
        // hex portions
        let delimited_hex = as_unicode.filter(|c| c.is_alphanumeric());
        let mut hex_idents: Vec<Ident> = delimited_hex.flat_map(|c| encode_as_ident(&c)).collect();
        // sandwich between _uc and uc_
        let mut book_ended: Vec<Ident> = vec![call_site_ident("_uc")];
        book_ended.append(&mut hex_idents);
        book_ended.push(call_site_ident("uc_"));
        book_ended
    }
}

pub fn build_path_type(path_expr: Expr) -> impl ToTokens {
    let idents = find_idents_in_expr(path_expr);
    idents.iter().map(|i| build_type_level_name_for(i)).fold(
        quote!(::frunk_core::hlist::HNil),
        |acc, t| {
            quote! {
            ::frunk_core::path::Path<
                ::frunk_core::hlist::HCons<
                   #t,
                   #acc
                >
              >
            }
        },
    )
}

/// Returns the idents in a path like expression in reverse
pub fn find_idents_in_expr(path_expr: Expr) -> Vec<Ident> {
    fn go(current: Expr, mut v: Vec<Ident>) -> Vec<Ident> {
        match current {
            Expr::Field(e) => {
                let m = e.member;
                match m {
                    Member::Named(i) => {
                        v.push(i);
                    }
                    _ => panic!("Only named access is supported"),
                }
                go(*e.base, v)
            }
            Expr::Path(p) => {
                if p.path.segments.len() != 1 {
                    panic!("Invalid name; this has collons in it")
                } else {
                    let i = p.path.segments[0].ident.clone();
                    v.push(i);
                    v
                }
            }
            _ => panic!("Invalid input"),
        }
    }
    go(path_expr, Vec::new())
}
