use quote::Tokens;
use common::{build_hcons_constr, to_ast};
use syn::{Ident, Body, VariantData, Field};
use proc_macro::TokenStream;

/// These are assumed to exist as enums in frunk_core::labelled
const ALPHA_CHARS: &'static [char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

/// These are assumed to exist as enums in frunk_core::labelled as underscore prepended enums
const UNDERSCORE_CHARS: &'static [char] = &['_', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Given an AST, returns an implementation of Generic using HList with
/// Labelled (see frunk_core::labelled) elements
///
/// Only works with Structs and Tuple Structs
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
        #[allow(non_snake_case, non_camel_case_types)]
        impl #impl_generics ::frunk_core::labelled::LabelledGeneric for #name #ty_generics #where_clause {

            type Repr = #repr_type;

            fn into(self) -> Self::Repr {
                let #struct_deconstr = self;
                #hcons_constr
            }

            fn from(r: Self::Repr) -> Self {
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
/// For example, given first_name, returns an AST for Hlist!(f,i,r,s,t,__,n,a,m,e)
fn build_type_level_name_for(ident: &Ident) -> Tokens {
    let name = ident.as_ref();
    let name_as_idents: Vec<Ident> = name.chars().flat_map(|c| encode_as_ident(&c)).collect();
    let name_as_tokens: Vec<Tokens> = name_as_idents.iter().map(|ident| {
        quote! { ::frunk_core::labelled::#ident }
    }).collect();
    build_hcons_type(&name_as_tokens)
}

fn build_hcons_type(idents: &Vec<Tokens>) -> Tokens {
    match idents.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let h = idents[0].clone();
            quote! { ::frunk_core::hlist::HCons<#h, ::frunk_core::hlist::HNil> }
        },
        _ => {
            let h = idents[0].clone();
            let tail = idents[1..].to_vec();
            let tail_type = build_hcons_type(&tail);
            quote! { ::frunk_core::hlist::HCons<#h, #tail_type> }
        }
    }
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
        vec![Ident::new(c.to_string())]
    } else if UNDERSCORE_CHARS.contains(c) {
        vec![Ident::new(format!("_{}", c))]
    } else {
        // UTF escape and get the hexcode
        let as_unicode = c.escape_unicode();
        // as_unicode can be multiple unicode codepoints encoded as \u{2764}\u{fe0f} (❤️)
        // so we filter on alphanumeric to get just u's as a delimiters along with the
        // hex portions
        let delimited_hex = as_unicode.filter(|c| c.is_alphanumeric());
        let mut hex_idents: Vec<Ident> = delimited_hex.flat_map(|c| encode_as_ident(&c)).collect();
        // sandwich between _uc and uc_
        let mut book_ended: Vec<Ident> = vec![Ident::new("_uc")];
        book_ended.append(&mut hex_idents);
        book_ended.push(Ident::new("uc_"));
        book_ended
    }
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
/// For example, given a field "age" of type i32, returns: label::<Hlist!(a,g,e), i32>(age)
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