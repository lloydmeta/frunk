use quote::Tokens;
use common::{build_hcons_constr, to_ast};
use syn::{Ident, Body, VariantData, Field};
use proc_macro::TokenStream;


pub fn impl_labelled_generic(input: TokenStream) -> Tokens {
    let ast = to_ast(&input);
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fields: &Vec<Field> = match ast.body {
        Body::Struct(VariantData::Struct(ref fields)) => fields,
        _ => panic!("Only Structs are supported. Tuple structs cannot be turned into Labelled Generics.")
    };
    let repr_type = build_labelled_repr(fields);

    let fnames: Vec<Ident> = fields
        .iter()
        // it's impossible to not have idents because we're sure this isn't a tuple struct
        .map(|f| f.ident.clone().unwrap())
        .collect();

    let hcons_constr = build_labelled_hcons_constr(&fields);
    let hcons_pat = build_hcons_constr(&fnames);
    let new_struct_constr = build_new_labelled_struct_constr(name, &fnames);
    let struct_deconstr = quote! { #name { #(#fnames, )* } };

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

/// Builds the labelled HList representation for a vector of fields
fn build_labelled_repr(fields: &Vec<Field>) -> Tokens {
    match fields.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let field = fields[0].clone();
            let labelled_type = build_labelled_type_for(&field);
            quote! { ::frunk_core::hlist::HCons<#labelled_type, ::frunk_core::hlist::HNil> }
        },
        _ => {
            let field = fields[0].clone();
            let labelled_type = build_labelled_type_for(&field);
            let tail = fields[1..].to_vec();
            let tail_type = build_labelled_repr(&tail);
            quote! { ::frunk_core::hlist::HCons<#labelled_type, #tail_type> }
        }
    }
}

/// Given a field, returns an AST for its Labelled (see labelled module in core) type,
/// which holds its name (or an approximation) and type.
fn build_labelled_type_for(field: &Field) -> Tokens {
    let ident = field.clone().ident.unwrap(); // this method is for labelled structs only
    let name_as_type = build_type_level_name_for(&ident);
    let ref field_type = field.ty;
    quote! { ::frunk_core::labelled::Labelled<#name_as_type, #field_type> }
}

/// Given an Ident returns an AST for its type level representation based on the
/// enums generated in frunk_core::labelled.
///
/// For example, given first_name, returns an AST for (f,i,r,s,t,__,n,a,m,e)
fn build_type_level_name_for(ident: &Ident) -> Tokens {
    let name = ident.as_ref();
    let name_as_types: Vec<Tokens> = name.chars().map(|c| {
        // Here we assume we have every single possible letter created via our label.
        if c.is_alphabetic() {
            let as_ident = Ident::new(c.to_string());
            quote! { ::frunk_core::labelled::#as_ident }
        } else {
            // non-alpha chars are represented with an underscore first.
            let as_ident = Ident::new(format!("_{}", c));
            quote! { ::frunk_core::labelled::#as_ident }
        }
    }).collect();
    quote! { (#(#name_as_types),*) }
}

/// Given a number of Idents that act as accessors and struct member
/// names, returns an AST representing how to construct an HList containing
/// Labelled values.
///
/// Assumes that there are bindings in the immediate environment with those names that
/// are bound to properly-typed values.
fn build_labelled_hcons_constr(fields: &Vec<Field>) -> Tokens {
    match fields.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let field = fields[0].clone();
            let labelled_constructor = build_labelled_constr_for(&field);
            quote! { ::frunk_core::hlist::HCons{ head: #labelled_constructor, tail: ::frunk_core::hlist::HNil } }
        },
        _ => {
            let field = fields[0].clone();
            let labelled_constructor = build_labelled_constr_for(&field);
            let tail = fields[1..].to_vec();
            let hlist_tail = build_labelled_hcons_constr(&tail);
            quote! { ::frunk_core::hlist::HCons{ head: #labelled_constructor, tail: #hlist_tail }}
        }
    }
}

/// Given a field, returns an AST for calling the Labelled constructor that holds its
/// value.
///
/// This calls a method in frunk_core::labelled called "label"
///
/// For example, given a field "age" of type i32, returns: label::<(a,g,e), i32>(age)
fn build_labelled_constr_for(field: &Field) -> Tokens {
    let name_as_type = build_type_level_name_for(&field.clone().ident.unwrap());
    let field_type = field.ty.clone();
    let field_name = field.ident.clone();
    quote! { ::frunk_core::labelled::label::<#name_as_type, #field_type>(#field_name) }
}

/// Given a struct name, and a number of Idents that act as accessors and struct member
/// names, returns an AST representing how to construct said struct.
///
/// Assumes that there are bindings in the immediate environment with those names that
/// are bound to Labelled values.
///
/// The opposite of build_labelled_hcons_constr
fn build_new_labelled_struct_constr(struct_name: &Ident, bindnames: &Vec<Ident>) -> Tokens {
    let cloned_bind1 = bindnames.clone();
    let cloned_bind2 = bindnames.clone();
    quote! { #struct_name { #(#cloned_bind1: #cloned_bind2.value),* } }
}