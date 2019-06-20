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
        },
        _ => panic!(
            "Only Structs are supported. Enums/Unions cannot be turned into Labelled Generics."
        ),
    };

    //     print!("{}", tree);
    tree
}
