use common::{build_hcons_constr, call_site_ident, to_ast};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Data, Field, Fields, FieldsUnnamed, Ident};

/// These are assumed to exist as enums in frunk_core::labelled
const ALPHA_CHARS: &'static [char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// These are assumed to exist as enums in frunk_core::labelled as underscore prepended enums
const UNDERSCORE_CHARS: &'static [char] = &['_', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

enum StructType {
    Named,
    Tuple,
}
use self::StructType::*;

/// Given an AST, returns an implementation of Generic using HList with
/// Field (see frunk_core::labelled) elements
///
/// Only works with Structs and Tuple Structs
pub fn impl_labelled_generic(input: TokenStream) -> impl ToTokens {
    let ast = to_ast(input);
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let (struct_type, fields): (StructType, Vec<Field>) = match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => (Named, fields.named.iter().cloned().collect()),
            Fields::Unnamed(ref fields) => (Tuple, create_indexed_fields(fields)),
            Fields::Unit => panic!("Unit structs cannot be turned into Labelled Generics"),
        },
        _ => panic!(
            "Only Structs are supported. Enums/Unions cannot be turned into Labelled Generics."
        ),
    };

    let repr_type = build_labelled_repr(&fields);
    let fnames: Vec<Ident> = fields
        .iter()
        // it's impossible to not have idents because we create synthetic names for tuple structs
        .map(|f| f.ident.clone().unwrap())
        .collect();

    let hcons_constr = build_labelled_hcons_constr(&fields);
    let hcons_pat = build_hcons_constr(&fnames);
    let (constructor, deconstructor) = match struct_type {
        Tuple => (
            Box::new(build_new_labelled_tuple_struct_constr(name, &fnames)) as Box<dyn ToTokens>,
            quote! { #name ( #(#fnames, )* ) },
        ),
        Named => (
            Box::new(build_new_labelled_struct_constr(name, &fnames)) as Box<dyn ToTokens>,
            quote! { #name { #(#fnames, )* } },
        ),
    };

    quote! {
        #[allow(non_snake_case, non_camel_case_types)]
        impl #impl_generics ::frunk_core::labelled::LabelledGeneric for #name #ty_generics #where_clause {

            type Repr = #repr_type;

            fn into(self) -> Self::Repr {
                let #deconstructor = self;
                #hcons_constr
            }

            fn from(r: Self::Repr) -> Self {
                let #hcons_pat = r;
                #constructor
            }
        }
    }
}

/// Create synthetic names for tuple struct members
fn create_indexed_fields(fields: &FieldsUnnamed) -> Vec<Field> {
    fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let mut f = f.clone();
            f.ident = Some(Ident::new(&format!("_{}", i), f.span()));
            f
        })
        .collect()
}

/// Builds the labelled HList representation for a vector of fields
fn build_labelled_repr(fields: &Vec<Field>) -> impl ToTokens {
    match fields.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let field = fields[0].clone();
            let labelled_type = build_labelled_type_for(&field);
            quote! { ::frunk_core::hlist::HCons<#labelled_type, ::frunk_core::hlist::HNil> }
        }
        _ => {
            let field = fields[0].clone();
            let labelled_type = build_labelled_type_for(&field);
            let tail = fields[1..].to_vec();
            let tail_type = build_labelled_repr(&tail);
            quote! { ::frunk_core::hlist::HCons<#labelled_type, #tail_type> }
        }
    }
}

/// Given a field, returns an AST for its Field (see labelled module in core) type,
/// which holds its name (or an approximation) and type.
fn build_labelled_type_for(field: &Field) -> impl ToTokens {
    let ident = field.clone().ident.unwrap(); // this method is for labelled structs only
    let name_as_type = build_type_level_name_for(&ident);
    let ref field_type = field.ty;
    quote! { ::frunk_core::labelled::Field<#name_as_type, #field_type> }
}

/// Given an Ident returns an AST for its type level representation based on the
/// enums generated in frunk_core::labelled.
///
/// For example, given first_name, returns an AST for Hlist!(f,i,r,s,t,__,n,a,m,e)
fn build_type_level_name_for(ident: &Ident) -> impl ToTokens {
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

/// Given a number of Idents that act as accessors and struct member
/// names, returns an AST representing how to construct an HList containing
/// Field values.
///
/// Assumes that there are bindings in the immediate environment with those names that
/// are bound to properly-typed values.
fn build_labelled_hcons_constr(fields: &Vec<Field>) -> impl ToTokens {
    match fields.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let field = fields[0].clone();
            let labelled_constructor = build_field_constr_for(&field);
            quote! { ::frunk_core::hlist::HCons{ head: #labelled_constructor, tail: ::frunk_core::hlist::HNil } }
        }
        _ => {
            let field = fields[0].clone();
            let labelled_constructor = build_field_constr_for(&field);
            let tail = fields[1..].to_vec();
            let hlist_tail = build_labelled_hcons_constr(&tail);
            quote! { ::frunk_core::hlist::HCons{ head: #labelled_constructor, tail: #hlist_tail }}
        }
    }
}

/// Given a field, returns an AST for calling the Field constructor that holds its
/// value.
///
/// This calls a method in frunk_core::labelled called "field_with_name", filling in the value and the
/// field name.
///
/// For example, given a field "age" of type i32, returns: field_with_name::<(a,g,e), i32>(age, "age")
fn build_field_constr_for(field: &Field) -> impl ToTokens {
    let name_as_type = build_type_level_name_for(&field.clone().ident.unwrap());
    let field_type = field.ty.clone();
    let field_name = field.ident.clone();
    let field_name_str = field
        .ident
        .clone()
        .expect("Field name should exist!")
        .to_string();
    quote! { ::frunk_core::labelled::field_with_name::<#name_as_type, #field_type>(#field_name_str, #field_name) }
}

/// Given a struct name, and a number of Idents that act as accessors and struct member
/// names, returns an AST representing how to construct said struct.
///
/// Assumes that there are bindings in the immediate environment with those names that
/// are bound to Field values.
///
/// The opposite of build_labelled_hcons_constr for named structs
fn build_new_labelled_struct_constr(struct_name: &Ident, bindnames: &Vec<Ident>) -> impl ToTokens {
    let cloned_bind1 = bindnames.clone();
    let cloned_bind2 = bindnames.clone();
    quote! { #struct_name { #(#cloned_bind1: #cloned_bind2.value),* } }
}

/// Given a tuple struct name, and a number of Idents that act as accessors
/// names, returns an AST representing how to construct said tuple struct.
///
/// Assumes that there are bindings in the immediate environment with those names that
/// are bound to Field values.
///
/// The opposite of build_labelled_hcons_constr for yuple structs
fn build_new_labelled_tuple_struct_constr(
    struct_name: &Ident,
    bindnames: &Vec<Ident>,
) -> impl ToTokens {
    let cloned_bind = bindnames.clone();
    quote! { #struct_name ( #(#cloned_bind.value),* ) }
}
