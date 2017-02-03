extern crate proc_macro;
#[macro_use]
extern crate frunk_core;
#[macro_use]
extern crate quote;
extern crate syn;

use syn::{Ident, Body, VariantData, Field, Ty};
use proc_macro::TokenStream;

#[proc_macro_derive(Generic)]
pub fn generic(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_generic(&ast);

//    println!("{}", gen);

    // Return the generated impl
    gen.parse().unwrap()
}


fn impl_generic(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fields: &Vec<Field> = match ast.body {
        Body::Struct(VariantData::Struct(ref fields)) => fields,
        Body::Struct(VariantData::Tuple(ref fields)) => fields,
        _ => panic!("Only structs are supported")
    };
    let field_types: Vec<Ty> = fields.iter()
        .map(|f| f.ty.clone()).collect();
    let repr_type = build_repr(&field_types);
    let maybe_fnames: Vec<Option<Ident>> = fields
        .iter()
        .map(|f| f.ident.clone())
        .collect();
    let is_tuple_struct = maybe_fnames.iter().all(|m_f| m_f.is_none());

    let fnames: Vec<Ident> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ident.clone().unwrap_or(Ident::new(format!("_{}", i))))
        .collect();
    let hcons_constr = build_hcons_constr(&fnames);
    let hcons_pat = build_hcons_constr(&fnames);
    let new_struct_constr = build_new_struct_constr(name, &fnames, is_tuple_struct);

    let struct_deconstr = if is_tuple_struct {
        quote! { #name ( #(#fnames, )* ) }
    } else {
        quote! { #name { #(#fnames, )* } }
    };

    quote! {
        impl #impl_generics ::frunk_core::generic::Generic<#repr_type> for #name #ty_generics #where_clause {

            fn into(self) -> #repr_type {
                let #struct_deconstr = self;
                #hcons_constr
            }

            fn from(r: #repr_type) -> Self {
                let #hcons_pat = r;
                #new_struct_constr
            }
        }
    }
}

/*
    For some reason, normal macros don't work inside the quote (non-items not allowed error),
    so we re-do our macros procedurally here. Ironically easier to understand..
*/

fn build_repr(field_types: &Vec<Ty>) -> quote::Tokens {
    match field_types.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let h = field_types[0].clone();
            quote! { ::frunk_core::hlist::HCons<#h, ::frunk_core::hlist::HNil> }
        },
        _ => {
            let h = field_types[0].clone();
            let tail = field_types[1..].to_vec();
            let tail_type = build_repr(&tail);
            quote! { ::frunk_core::hlist::HCons<#h, #tail_type> }
        }
    }
}

fn build_hcons_constr(field_types: &Vec<Ident>) -> quote::Tokens {
    match field_types.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let h = field_types[0].clone();
            quote! { ::frunk_core::hlist::HCons{ head: #h, tail: ::frunk_core::hlist::HNil } }
        },
        _ => {
            let h = field_types[0].clone();
            let tail = field_types[1..].to_vec();
            let hlist_tail = build_hcons_constr(&tail);
            quote! { ::frunk_core::hlist::HCons{ head: #h, tail: #hlist_tail }}
        }
    }
}

fn build_new_struct_constr(struct_name: &Ident, bindnames: &Vec<Ident>, is_tuple_struct: bool) -> quote::Tokens {
    if is_tuple_struct {
        let cloned_bind = bindnames.clone();
        quote! { #struct_name (#(#cloned_bind),* ) }
    } else {
        let cloned_bind1 = bindnames.clone();
        let cloned_bind2 = bindnames.clone();
        quote! { #struct_name { #(#cloned_bind1: #cloned_bind2),* } }
    }
}