use proc_macro::TokenStream;
use quote::ToTokens;
use quote::__rt::Span;
use syn;
use syn::{DeriveInput, Ident};

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
