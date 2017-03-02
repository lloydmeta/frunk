use quote::Tokens;
use common::{build_hcons_constr, to_ast};
use syn::{Ident, Body, VariantData, Field, Ty};
use proc_macro::TokenStream;

pub fn impl_generic(input: TokenStream) -> Tokens {
    let ast = to_ast(&input);
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

fn build_repr(field_types: &Vec<Ty>) -> Tokens {
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


fn build_new_struct_constr(struct_name: &Ident, bindnames: &Vec<Ident>, is_tuple_struct: bool) -> Tokens {
    if is_tuple_struct {
        let cloned_bind = bindnames.clone();
        quote! { #struct_name (#(#cloned_bind),* ) }
    } else {
        let cloned_bind1 = bindnames.clone();
        let cloned_bind2 = bindnames.clone();
        quote! { #struct_name { #(#cloned_bind1: #cloned_bind2),* } }
    }
}
