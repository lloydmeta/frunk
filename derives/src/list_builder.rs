extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
#[cfg(feature = "nightly")]
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Block, DeriveInput, GenericParam,
    Generics, Stmt,
};

mod type_helpers;
use type_helpers::{Annotation, ArgPair, PredicateVec, WhereLine};

use self::type_helpers::AnnoErr;

pub fn list_build_inner(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    // get the fields: the list-built fields, and the manually-built fields
    let (input, list_built_fields, ignored_fields) = parse_fields(input);

    let block = gen_stmts(&list_built_fields, &ignored_fields);

    // hl_new args include the injected list, and values for the non-list built args
    let args = ArgPair::make_args(ignored_fields);

    if list_built_fields.len() == 0 {
        panic!("redundant builder annotations");
    }
    let types = list_built_fields
        .iter()
        .map(|(ArgPair { tp, .. }, _)| tp.clone())
        .collect::<Vec<_>>();

    // make all where-clauses
    let lines: Vec<WhereLine> = WhereLine::gen_lines_top(&types);

    // Take the last clause and absorb that to make the ret-val
    let ret = WhereLine::absorb(lines.last().expect("last line").clone());

    let output = quote! { -> (Self, #ret) };
    let where_clause: syn::WhereClause = PredicateVec::from(lines.clone()).into();
    let gens = make_generic_params(list_built_fields.len());
    let struct_ident = input.clone().ident;
    let fun = quote! {
        fn hl_new<#gens>(#(#args),*) #output
        #where_clause
        {
            #block
        }
    };

    // et, voile! en a des code magnifique!
    quote! {
        impl #struct_ident {
            #fun
        }
    }
    .into()
}
/// collects the field name/type pairs, splitting them according to fields being built by the list
/// or as args passed into the constructor
fn parse_fields(
    mut input: DeriveInput,
) -> (DeriveInput, Vec<(ArgPair, Option<syn::Expr>)>, Vec<ArgPair>) {
    let mut list_built = Vec::new();
    let mut ignored_fields = Vec::new();

    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(named),
        ..
    }) = &mut input.data
    {
        for field in &named.named {
            // ignored and pluck-type are mutually exclusive...
            match Annotation::try_from(&field.attrs[..]) {
                Ok(Annotation::Plucker { ty, map }) => list_built.push((
                    (field.ident.clone().expect("field_ident"), ty).into(),
                    Some(map),
                )),
                Ok(Annotation::Ignore) => ignored_fields
                    .push((field.ident.clone().expect("field_ident"), field.ty.clone()).into()),
                Err(AnnoErr::NoMatch) => todo!(),
                Err(AnnoErr::XOR) => {
                    #[cfg(feature = "nightly")]
                    field
                        .span()
                        .unwrap()
                        .error("Redundant pluck-type on ignored field")
                        .emit();
                    #[cfg(not(feature = "nightly"))]
                    panic!("don't ignore fields with pluckers");
                }
            };
        }
    }
    (input, list_built, ignored_fields)
}

// `<L0, L1, ..., L(N-1)>` for the `fn hl_new<L1, ...>`
fn make_generic_params(count: usize) -> Punctuated<GenericParam, Comma> {
    let gens: String = (0..count + 1)
        .map(|i| format!("L{}", i))
        .collect::<Vec<String>>()
        .join(", ");
    let gens = format!("<{}>", gens);
    syn::parse_str::<Generics>(&gens)
        .expect("parsing the make_generic_params")
        .params
}

// generates a line of code for each field that needs a value plucked out of the constructor list.
// ```ignore
// let (list_built0, l1) = frunk::hlist::Plucker::pluck(l0);
// ```
// ...and for the fileds ignored, just moves from the function argument to the rusulting structs
// field
fn gen_stmts(fields: &[(ArgPair, Option<syn::Expr>)], args: &[ArgPair]) -> Block {
    let mut list_n = 0;
    let mut stmts: Vec<Stmt> = vec![];
    // Generate the "let (field, lX) = lY.pluck();" statements
    for (arg_pair, expr) in fields {
        let next_list = list_n + 1;
        let next_list = syn::Ident::new(&format!("l{}", next_list), Span::call_site());
        let list_n_tok = syn::Ident::new(&format!("l{}", list_n), Span::call_site());
        let field_name = &arg_pair.ident;
        let plucking = quote! {
            let (#field_name, #next_list) = frunk::hlist::Plucker::pluck(#list_n_tok);
        };

        let stmt: Stmt = syn::parse2(plucking.clone())
            .unwrap_or_else(|_| panic!("Failed to parse statement: {}", plucking.to_string()));

        stmts.push(stmt);
        if let Some(expr) = expr {
            let mapping = quote! {
                let #field_name = #expr;
            };
            stmts.push(syn::parse2(mapping).unwrap());
        };
        list_n += 1;
    }

    // Generate the "Self { fields... }" part of the block
    let args = args
        .iter()
        .map(|ArgPair { ident, .. }| ident.clone())
        .collect::<Vec<_>>();
    let all_fields = [
        &fields
            .iter()
            .map(|(field, _)| field.ident.clone())
            .collect::<Vec<_>>()[..],
        &args[..],
    ]
    .concat();
    let list_n_ident = syn::Ident::new(&format!("l{}", list_n), proc_macro2::Span::call_site());
    let self_stmt: Stmt = syn::parse2(quote! {
        return (Self { #(#all_fields,)* }, #list_n_ident);
    })
    .expect("generating the Self...");
    stmts.push(self_stmt);

    Block {
        stmts,
        brace_token: Default::default(),
    }
}
