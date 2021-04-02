use frunk_proc_macro_helpers::*;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::Data;

/// Given an AST, returns an implementation of Generic using HList
///
/// Only works with Structs and Tuple Structs
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
        _ => panic!("Only Structs are supported. Enums/Unions cannot be turned into Generics."),
    };

    //     print!("{}", tree);
    tree
}
