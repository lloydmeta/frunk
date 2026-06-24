use frunk_proc_macro_helpers::*;
use proc_macro::TokenStream;
use quote::ToTokens;
use std::iter::repeat;
use syn::Data;

/// Given an AST, returns an implementation of Generic using an HList
/// representation for structs and a Coproduct of payload HLists for enums.
///
/// Works with structs, tuple structs, and enums.
pub fn impl_generic(input: TokenStream) -> impl ToTokens {
    let ast = to_ast(input);
    let name = &ast.ident;

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    #[allow(clippy::let_and_return)]
    let tree = match ast.data {
        Data::Struct(ref data) => {
            let field_bindings = FieldBindings::new(&data.fields);
            let repr_type = field_bindings.build_hlist_type(FieldBinding::build_type);
            let hcons_constr = field_bindings.build_hlist_constr(FieldBinding::build);
            let type_constr = field_bindings.build_type_constr(FieldBinding::build);

            quote! {
                #[allow(non_snake_case, non_camel_case_types)]
                impl #impl_generics ::frunk_core::generic::Generic for #name #ty_generics #where_clause {

                    type Repr = #repr_type;

                    fn into(self) -> Self::Repr {
                        let #name #type_constr = self;
                        #hcons_constr
                    }

                    fn from(r: Self::Repr) -> Self {
                        let #hcons_constr = r;
                        #name #type_constr
                    }
                }
            }
        }
        Data::Enum(ref data) => {
            let variant_bindings = VariantBindings::new(&data.variants);
            let repr_type = &variant_bindings.build_coprod_type(VariantBinding::build_hlist_type);
            let coprod_constrs =
                &variant_bindings.build_coprod_constrs(VariantBinding::build_hlist_constr);
            let coprod_unreachable = &variant_bindings.build_coprod_unreachable_arm(false);
            let type_constrs1 =
                &variant_bindings.build_variant_constrs(VariantBinding::build_type_constr);
            let type_constrs2 = type_constrs1;
            let name_it1 = repeat(name);
            let name_it2 = repeat(name);

            quote! {
                #[allow(non_snake_case, non_camel_case_types)]
                impl #impl_generics ::frunk_core::generic::Generic for #name #ty_generics #where_clause {

                    type Repr = #repr_type;

                    fn into(self) -> Self::Repr {
                        match self {
                            #(
                                #name_it1 :: #type_constrs1 => #coprod_constrs,
                            )*
                        }
                    }

                    fn from(r: Self::Repr) -> Self {
                        match r {
                            #(
                                #coprod_constrs => #name_it2 :: #type_constrs2,
                            )*
                            #coprod_unreachable
                        }
                    }
                }
            }
        }
        _ => panic!("Only Structs and Enums can be turned into Generics."),
    };

    //     print!("{}", tree);
    tree
}
