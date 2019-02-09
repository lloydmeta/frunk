use frunk_proc_macro_helpers::*;
use proc_macro::TokenStream;
use quote::ToTokens;
use quote::__rt::Span;
use syn::spanned::Spanned;
use syn::{
    Data, Field, Fields, FieldsUnnamed, GenericParam, Generics, Ident, Lifetime, LifetimeDef,
};

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

    let mut generics_ref: Generics = generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // instantiate a lifetime and lifetime def to add
    let ref_lifetime = Lifetime::new("'_frunk_labelled_generic_ref_", Span::call_site());
    let ref_lifetime_def = LifetimeDef::new(ref_lifetime.clone());

    // Constrain the generic lifetimes present in the concrete type to the reference lifetime
    // of our implementation of LabelledGeneric for the reference case (& and &mut)
    {
        let generics_ref_lifetimes_mut = generics_ref.lifetimes_mut();
        for existing_lifetime_mut in generics_ref_lifetimes_mut {
            existing_lifetime_mut.bounds.push(ref_lifetime.clone());
        }
    }

    // Add our current generic lifetime to the list of generics
    let ref_lifetime_generic = GenericParam::Lifetime(ref_lifetime_def);
    generics_ref.params.push(ref_lifetime_generic);

    // Get the generics back out for our reference types
    let (impl_generics_ref, _, where_clause_ref) = generics_ref.split_for_impl();
    let impl_generics_mut_ref = impl_generics_ref.clone();
    let where_clause_mut_ref = where_clause_ref.clone();

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

    let repr_type = build_labelled_repr(&fields, SelfKind::Value);
    let ref_repr_type = build_labelled_repr(&fields, SelfKind::Ref);
    let mut_ref_repr_type = build_labelled_repr(&fields, SelfKind::MutRef);
    let fnames: Vec<Ident> = fields
        .iter()
        // it's impossible to not have idents because we create synthetic names for tuple structs
        .map(|f| f.ident.clone().unwrap())
        .collect();
    let fnames_ref = fnames.clone();
    let fnames_mut_ref = fnames.clone();

    let hcons_constr = build_labelled_hcons_constr(&fields, SelfKind::Value);
    let hcons_constr_ref = build_labelled_hcons_constr(&fields, SelfKind::Ref);
    let hcons_constr_mut_ref = build_labelled_hcons_constr(&fields, SelfKind::MutRef);

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
    let struct_ref_deconstr = match struct_type {
        Tuple => quote! { #name ( #(ref #fnames_ref, )* ) },
        Named => quote! { #name { #(ref #fnames_ref, )* } },
    };
    let struct_mut_ref_deconstr = match struct_type {
        Tuple => quote! { #name ( #(ref mut #fnames_mut_ref, )* ) },
        Named => quote! { #name { #(ref mut #fnames_mut_ref, )* } },
    };
    let tree = quote! {
        #[allow(non_snake_case, non_camel_case_types)]
        impl #impl_generics ::frunk_core::labelled::LabelledGeneric for #name #ty_generics #where_clause {

            type Repr = #repr_type;

            #[inline(always)]
            fn into(self) -> Self::Repr {
                let #deconstructor = self;
                #hcons_constr
            }

            #[inline(always)]
            fn from(r: Self::Repr) -> Self {
                let #hcons_pat = r;
                #constructor
            }
        }

        #[allow(non_snake_case, non_camel_case_types)]
        impl #impl_generics_ref ::frunk_core::labelled::IntoLabelledGeneric for & '_frunk_labelled_generic_ref_ #name #ty_generics #where_clause_ref {

            type Repr = #ref_repr_type;

            #[inline(always)]
            fn into(self) -> Self::Repr {
                let #struct_ref_deconstr = *self;
                #hcons_constr_ref
            }

        }

        #[allow(non_snake_case, non_camel_case_types)]
        impl #impl_generics_mut_ref ::frunk_core::labelled::IntoLabelledGeneric for & '_frunk_labelled_generic_ref_ mut #name #ty_generics #where_clause_mut_ref {

            type Repr = #mut_ref_repr_type;

            #[inline(always)]
            fn into(self) -> Self::Repr {
                let #struct_mut_ref_deconstr = *self;
                #hcons_constr_mut_ref
            }

        }
    };
    //     print!("{}", tree);
    tree
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
fn build_labelled_repr(fields: &Vec<Field>, self_kind: SelfKind) -> impl ToTokens {
    match fields.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let field = fields[0].clone();
            let labelled_type = build_labelled_type_for(&field, self_kind);
            quote! { ::frunk_core::hlist::HCons<#labelled_type, ::frunk_core::hlist::HNil> }
        }
        _ => {
            let field = fields[0].clone();
            let labelled_type = build_labelled_type_for(&field, self_kind.clone());
            let tail = fields[1..].to_vec();
            let tail_type = build_labelled_repr(&tail, self_kind.clone());
            quote! { ::frunk_core::hlist::HCons<#labelled_type, #tail_type> }
        }
    }
}

/// Given a field, returns an AST for its Field (see labelled module in core) type,
/// which holds its name (or an approximation) and type.
fn build_labelled_type_for(field: &Field, self_kind: SelfKind) -> impl ToTokens {
    let ident = field.clone().ident.unwrap(); // this method is for labelled structs only
    let name_as_type = build_type_level_name_for(&ident);
    let ref field_type = field.ty;
    match self_kind {
        SelfKind::Value => quote! { ::frunk_core::labelled::Field<#name_as_type, #field_type> },
        SelfKind::Ref => {
            quote! { ::frunk_core::labelled::Field<#name_as_type, &'_frunk_labelled_generic_ref_ #field_type> }
        }
        SelfKind::MutRef => {
            quote! { ::frunk_core::labelled::Field<#name_as_type, &'_frunk_labelled_generic_ref_ mut #field_type> }
        }
    }
}

/// Given a number of Idents that act as accessors and struct member
/// names, returns an AST representing how to construct an HList containing
/// Field values.
///
/// Assumes that there are bindings in the immediate environment with those names that
/// are bound to properly-typed values.
fn build_labelled_hcons_constr(fields: &Vec<Field>, self_kind: SelfKind) -> impl ToTokens {
    match fields.len() {
        0 => quote! { ::frunk_core::hlist::HNil },
        1 => {
            let field = fields[0].clone();
            let labelled_constructor = build_field_constr_for(&field, self_kind);
            quote! { ::frunk_core::hlist::HCons{ head: #labelled_constructor, tail: ::frunk_core::hlist::HNil } }
        }
        _ => {
            let field = fields[0].clone();
            let labelled_constructor = build_field_constr_for(&field, self_kind);
            let tail = fields[1..].to_vec();
            let hlist_tail = build_labelled_hcons_constr(&tail, self_kind);
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
fn build_field_constr_for(field: &Field, self_kind: SelfKind) -> impl ToTokens {
    let name_as_type = build_type_level_name_for(&field.clone().ident.unwrap());
    let field_type = field.ty.clone();
    let field_name = field.ident.clone();
    let field_name_str = field
        .ident
        .clone()
        .expect("Field name should exist!")
        .to_string();

    match self_kind {
        SelfKind::Value => {
            quote! { ::frunk_core::labelled::field_with_name::<#name_as_type, #field_type>(#field_name_str, #field_name) }
        }
        SelfKind::Ref => {
            quote! { ::frunk_core::labelled::field_with_name::<#name_as_type, &'_frunk_labelled_generic_ref_ #field_type>(#field_name_str, #field_name) }
        }
        SelfKind::MutRef => {
            quote! { ::frunk_core::labelled::field_with_name::<#name_as_type, &'_frunk_labelled_generic_ref_ mut #field_type>(#field_name_str, #field_name) }
        }
    }
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

#[derive(Copy, Clone)]
enum SelfKind {
    Value,
    Ref,
    MutRef,
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
