extern crate proc_macro;
#[macro_use]
extern crate frunk_core;
#[macro_use]
extern crate quote;
extern crate syn;

use syn::{Ident, Body, Variant, VariantData, Field, Ty};
use proc_macro::TokenStream;
use frunk_core::hlist::*;
use frunk_core::generic::*;

#[proc_macro_derive(Generic)]
pub fn generic(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_generic(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}


fn impl_generic(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fields: &Vec<Field> = match ast.body {
        Body::Struct(VariantData::Struct(ref fields)) => fields,
        _ => panic!("Only structs are supported")
    };
    let field_types: Vec<Ty> = fields.iter()
        .map(|f| f.ty.clone()).collect();
    let fnames1: Vec<Ident> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.clone().unwrap_or(Ident::new(format!("_{}", i))))
        .collect();
    let fnames2: Vec<Ident> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.clone().unwrap_or(Ident::new(format!("_{}", i))))
        .collect();
    let fnames3: Vec<Ident> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.clone().unwrap_or(Ident::new(format!("_{}", i))))
        .collect();
    let fnames4: Vec<Ident> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.clone().unwrap_or(Ident::new(format!("_{}", i))))
        .collect();
    let fnames5: Vec<Ident> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.clone().unwrap_or(Ident::new(format!("_{}", i))))
        .collect();

    quote! {
        impl #impl_generics ::frunk_core::generic::Generic for #name #ty_generics #where_clause {
            type Repr = ::frunk_core::hlist::Hlist![#(#field_types),*];

            fn into_generic(self) -> Self::Repr {
                let #name { #(#fnames1, )* } = self;
                ::frunk_core::hlist::hlist!(#(#fnames2),*)
            }

            fn from_generic(r: Self::Repr) -> Self {
                let ::frunk_core::hlist::hlist_pat!(#(#fnames3 ), *) = r;
                #name { #(#fnames4    : #fnames5),* }
            }
        }
    }
}


#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_hello_world(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }
}