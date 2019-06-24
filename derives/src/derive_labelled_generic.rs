use std::iter::repeat;

use frunk_proc_macro_helpers::*;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::Data;

/// Given an AST, returns an implementation of Generic using HList with
/// Field (see frunk_core::labelled) elements
///
/// Only works with Structs and Tuple Structs
pub fn impl_labelled_generic(input: TokenStream) -> impl ToTokens {
    let ast = to_ast(input);
    let name = &ast.ident;

    let generics = &ast.generics;
    let generics_ref = ref_generics(generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let (impl_generics_ref, _, where_clause_ref) = generics_ref.split_for_impl();

    let tree = match ast.data {
        Data::Struct(ref data) => {
            let field_bindings = FieldBindings::new(&data.fields);
            let repr_type = field_bindings.build_hlist_type(FieldBinding::build_field_type);
            let repr_type_ref = field_bindings.build_hlist_type(FieldBinding::build_field_type_ref);
            let repr_type_mut = field_bindings.build_hlist_type(FieldBinding::build_field_type_mut);
            let hcons_expr = field_bindings.build_hlist_constr(FieldBinding::build_field_expr);
            let hcons_pat = field_bindings.build_hlist_constr(FieldBinding::build_field_pat);
            let type_constr = field_bindings.build_type_constr(FieldBinding::build);
            let type_pat_ref = field_bindings.build_type_constr(FieldBinding::build_pat_ref);
            let type_pat_mut = field_bindings.build_type_constr(FieldBinding::build_pat_mut);

            quote! {
                #[allow(non_snake_case, non_camel_case_types)]
                impl #impl_generics ::frunk_core::labelled::LabelledGeneric for #name #ty_generics #where_clause {

                    type Repr = #repr_type;

                    #[inline(always)]
                    fn into(self) -> Self::Repr {
                        let #name #type_constr = self;
                        #hcons_expr
                    }

                    #[inline(always)]
                    fn from(r: Self::Repr) -> Self {
                        let #hcons_pat = r;
                        #name #type_constr
                    }
                }

                #[allow(non_snake_case, non_camel_case_types)]
                impl #impl_generics_ref ::frunk_core::labelled::IntoLabelledGeneric for & '_frunk_ref_ #name #ty_generics #where_clause_ref {

                    type Repr = #repr_type_ref;

                    #[inline(always)]
                    fn into(self) -> Self::Repr {
                        let #name #type_pat_ref = *self;
                        #hcons_expr
                    }

                }

                #[allow(non_snake_case, non_camel_case_types)]
                impl #impl_generics_ref ::frunk_core::labelled::IntoLabelledGeneric for & '_frunk_ref_ mut #name #ty_generics #where_clause_ref {

                    type Repr = #repr_type_mut;

                    #[inline(always)]
                    fn into(self) -> Self::Repr {
                        let #name #type_pat_mut = *self;
                        #hcons_expr
                    }

                }
            }
        }
        Data::Enum(ref data) => {
            let variant_bindings = VariantBindings::new(&data.variants);
            let repr_type =
                &variant_bindings.build_coprod_type(VariantBinding::build_hlist_field_type);
            let repr_type_ref =
                &variant_bindings.build_coprod_type(VariantBinding::build_hlist_field_type_ref);
            let repr_type_mut =
                &variant_bindings.build_coprod_type(VariantBinding::build_hlist_field_type_mut);
            let coprod_exprs =
                &variant_bindings.build_coprod_constrs(VariantBinding::build_hlist_field_expr);
            let coprod_pats =
                &variant_bindings.build_coprod_constrs(VariantBinding::build_hlist_field_pat);
            let coprod_unreachable = &variant_bindings.build_coprod_unreachable_arm(false);
            let type_constrs1 =
                &variant_bindings.build_variant_constrs(VariantBinding::build_type_constr);
            let type_constrs2 = type_constrs1;
            let type_pat_ref =
                &variant_bindings.build_variant_constrs(VariantBinding::build_type_pat_ref);
            let type_pat_mut =
                &variant_bindings.build_variant_constrs(VariantBinding::build_type_pat_mut);
            let name_it1 = repeat(name);
            let name_it2 = repeat(name);
            let name_it3 = repeat(name);
            let name_it4 = repeat(name);

            let base_impl = quote! {
                #[allow(non_snake_case, non_camel_case_types)]
                impl #impl_generics ::frunk_core::labelled::LabelledGeneric for #name #ty_generics #where_clause {

                    type Repr = #repr_type;

                    #[inline(always)]
                    fn into(self) -> Self::Repr {
                        match self {
                            #(
                                #name_it1 :: #type_constrs1 => #coprod_exprs,
                            )*
                        }
                    }

                    #[inline(always)]
                    fn from(r: Self::Repr) -> Self {
                        match r {
                            #(
                                #coprod_pats => #name_it2 :: #type_constrs2,
                            )*
                            #coprod_unreachable
                        }
                    }
                }
            };

            let ref_impl = quote! {
                #[allow(non_snake_case, non_camel_case_types)]
                impl #impl_generics_ref ::frunk_core::labelled::IntoLabelledGeneric for & '_frunk_ref_ #name #ty_generics #where_clause_ref {

                    type Repr = #repr_type_ref;

                    #[inline(always)]
                    fn into(self) -> Self::Repr {
                        match self {
                            #(
                                #name_it3 :: #type_pat_ref => #coprod_exprs,
                            )*
                        }
                    }

                }
            };

            let mut_impl = quote! {
                #[allow(non_snake_case, non_camel_case_types)]
                impl #impl_generics_ref ::frunk_core::labelled::IntoLabelledGeneric for & '_frunk_ref_ mut #name #ty_generics #where_clause_ref {

                    type Repr = #repr_type_mut;

                    #[inline(always)]
                    fn into(self) -> Self::Repr {
                        match self {
                            #(
                                #name_it4 :: #type_pat_mut => #coprod_exprs,
                            )*
                        }
                    }

                }
            };

            quote! { #base_impl #ref_impl #mut_impl }
        }
        _ => panic!(
            "Only Structs are supported. Enums/Unions cannot be turned into Labelled Generics."
        ),
    };

    tree
}
