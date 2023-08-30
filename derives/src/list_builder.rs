extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Block, DeriveInput, FnArg,
    GenericParam, Generics, Ident, Stmt, WherePredicate, TypeParamBound
};

// struct field or function argument
struct ArgPair {
    ident: syn::Ident,
    tp: syn::Type
}

impl ArgPair {
    /// ```ignore
    /// #[derive(ListBuild)]
    /// struct Foo {
    ///   foo: u8,
    ///   #[list_build_ignore]
    ///   bar: u16
    /// }
    /// // -> fn(l0: HList!(u8), bar: u16) 
    fn make_args(fields: Vec<ArgPair>) -> impl Iterator<Item = FnArg> {
       std::iter::once(syn::parse2(quote!{l0: L0}).unwrap()).chain(fields.into_iter().map(FnArg::from))
    }
}
impl From<(syn::Ident, syn::Type)> for ArgPair {
    fn from(value: (syn::Ident, syn::Type)) -> Self {
        Self{ ident: value.0, tp: value.1 }
    }
}
impl From<ArgPair> for syn::FnArg {
    fn from(value: ArgPair) -> Self {
        syn::FnArg::Typed(syn::PatType {
            attrs: Vec::new(),
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: value.ident.clone(),
                subpat: None,
            })),
            colon_token: syn::token::Colon { spans: [proc_macro2::Span::call_site()] },
            ty: Box::new(value.tp.clone())
        })
    }
}

/// each line in the where predicate is the type-binding, and the trait it must impl
#[derive(Clone)]
struct WhereLine {
    tp: syn::Type,
    pred: PluckParam
}

impl WhereLine {
    /// entry into the recursive build-up of the where-lines
    ///
    /// generates the base-line (L0: Plucker<SomeType, L1>) then goes into the recursive call
    fn gen_lines_top(types: &[syn::Type]) -> Vec<Self> {
        if types.len() == 0 {
            panic!("no pluckings happening");
        }
        let base = Self::gen_base(&types[0]);
        if types.len() == 1 {
            return vec![base];
        }
        Self::gen_lines_recur(vec![base],  &types[1..])
    }

    /// Beginning with the base line, it generates a new where-line for each type.
    /// Each line is an "absorbed" version of the previous line, with `Plucker<NextType, LN+1>` as
    /// the new trait-impl requirement.
    ///
    /// ```ignore
    ///  LN:   Plucker<TN, LN+1> // is saved, the next being...
    /// <LN as PLucker<TN, LN+1>>::Remainder: Plucker<TN+1, LN+2>
    /// ```
    fn gen_lines_recur(mut acc: Vec<Self>, types: &[syn::Type]) -> Vec<Self> {
        // use the previous predicate to make the new type
        let tp = acc.last().cloned().expect("should never recurse without the base...").absorb();

        let pred = PluckParam::from((types[0].clone(), acc.len() as u8 + 1));
        acc.push(Self{tp, pred});
        if types.len() == 1 {
            return acc;
        }
        Self::gen_lines_recur(acc,  &types[1..])
    }

    /// L0: Plucker<tp, L1>
    fn gen_base(tp: &syn::Type) -> Self {
        let pred = syn::parse2( quote!{frunk::hlist::Plucker<#tp, L1>} ).expect("quote the base plucker");
        // Create the WhereLine
        WhereLine {
            tp: syn::parse2( quote!{L0} ).expect("quote the L0"),
            pred: PluckParam(pred),
        }
    }

    /// Does the "absorption" needed for the next line for each gen_lines_recur
    /// Tn: Pn -> <Tn as Pn>::Remainder
    fn absorb(self) -> syn::Type {
        let WhereLine { tp, pred } = self;
        let pred = pred.0;
        
        let res = quote!{<#tp as #pred>::Remainder};
        syn::parse2(res).expect("absorbing")
    }
}

impl From<WhereLine> for WherePredicate {
    fn from(line: WhereLine) -> Self {
        let WhereLine { tp, pred } = line;
        let bound: TypeParamBound = pred.0.into();
        let predicate = quote! { #tp: #bound };
        syn::parse2(predicate).expect("whereline to pred")
    }
}

/// shim allowing PredicateVec::from(line_vec).into() where a `syn::WhereClause` is needed
struct PredicateVec {
    preds: Vec<WherePredicate>
}

impl From<Vec<WhereLine>> for PredicateVec {
    fn from(value: Vec<WhereLine>) -> Self {
        Self{ preds: value.into_iter().map(WherePredicate::from).collect() }
    }
}
impl From<PredicateVec> for syn::WhereClause {
    fn from(value: PredicateVec) -> Self {
        Self {
            where_token: syn::Token![where](proc_macro2::Span::call_site()),
            predicates: syn::punctuated::Punctuated::from_iter(value.preds),
        }
    }
}

#[derive(Clone)]
struct PluckParam(syn::TypeParamBound);
impl From<(syn::Type, u8)> for PluckParam {
    fn from((ty, n): (syn::Type, u8)) -> Self {


        // Create an ident for "LN" where N is the u8 value
        let l_ident = syn::Ident::new(&format!("L{}", n), Span::call_site());
        return Self(syn::parse2(quote!{frunk::hlist::Plucker<#ty, #l_ident>}).unwrap());
    }
}



pub fn list_build_inner(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    // get the fields: the list-built fields, and the manually-built fields
    let (input, annotated_fields, non_annotated_fields) = parse_fields(input);

    
    let block = gen_stmts(
        &annotated_fields.iter().map(|ArgPair{ident, ..}| ident.clone()).collect(),
        &non_annotated_fields
            .iter()
            .map(|ArgPair{ident, ..}| ident.clone())
            .collect::<Vec<_>>()[..],
    );

    // hl_new args include the injected list, and values for the non-list built args
    let args = ArgPair::make_args(non_annotated_fields);

    if annotated_fields.len() == 0 {
        panic!("redundant builder annotations");
    }
    let types = annotated_fields.iter().map(|ArgPair{ tp, .. }| tp.clone()).collect::<Vec<_>>();

    // make all where-clauses
    let lines: Vec<WhereLine> = WhereLine::gen_lines_top(&types[..]);

    // Take the last clause and absorb that to make the ret-val
    let ret = WhereLine::absorb(lines.last().expect("last line").clone());



    let fn_ident = syn::Ident::new("hl_new", proc_macro2::Span::call_site());
    let output: syn::ReturnType = syn::ReturnType::Type(
        syn::token::RArrow::default(),
        Box::new(syn::Type::Tuple(syn::TypeTuple {
            paren_token: syn::token::Paren::default(),
            elems: {
                let mut punctuated = Punctuated::new();
                punctuated.push(syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::parse_str("Self").expect("parseing Self"),
                }));
                punctuated.push(ret);
                punctuated
            },
        })),
    );
    // tie all the signature details together
    let sig = syn::Signature {
        ident: fn_ident,
        generics: Generics {
            where_clause: Some(PredicateVec::from(lines).into()),
            params: make_generic_params(annotated_fields.len()),
            ..Default::default()
        },
        inputs: Punctuated::from_iter(args),

        output,
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        variadic: None,
        fn_token: Default::default(),
        paren_token: Default::default(),
    };
    let struct_ident = input.clone().ident;
    let fun = syn::ItemFn {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        sig,
        block: Box::new(block),
    };

    // et, voile! en a des code magnifique!
    quote! {
        impl #struct_ident {
            #fun
        }
    }.into()
}
/// collects the field name/type pairs, splitting them according to fields being built by the list
/// or as args passed into the constructor
fn parse_fields(
    mut input: DeriveInput,
) -> (
    DeriveInput,
    Vec<ArgPair>,
    Vec<ArgPair>,
) {
    let mut list_built = Vec::new();
    let mut ignored_fields = Vec::new();


    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(named),
        ..
    }) = &mut input.data
    {
        for field in &mut named.named {
            let ignored = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("list_build_ignore"));

            let field_ident = field.ident.clone().expect("field ident"); // Assuming named fields
            let field_type = field.ty.clone(); // Type

            if ignored {
                ignored_fields.push((field_ident, field_type).into());
            } else {
                list_built.push((field_ident, field_type).into());
                // Remove the hl_field attribute
                field.attrs.retain(|attr| !attr.path().is_ident("list_build_ignore"));
            }
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
    syn::parse_str::<Generics>(&gens).expect("parsing the make_generic_params").params
}


// generates a line of code for each field that needs a value plucked out of the constructor list. 
// ```ignore
// let (list_built0, l1) = frunk::hlist::Plucker::pluck(l0);
// ```
// ...and for the fileds ignored, just moves from the function argument to the rusulting structs
// field
fn gen_stmts(fields: &Vec<Ident>, args: &[Ident]) -> Block {
    let mut list_n = 0;
    let mut stmts: Vec<Stmt> = vec![];
    // Generate the "let (field, lX) = lY.pluck();" statements
    for id in fields {
        let next_list = list_n + 1;
        let next_list = syn::Ident::new(&format!("l{}", next_list), Span::call_site());
        let list_n_tok = syn::Ident::new(&format!("l{}", list_n), Span::call_site());
        let stmt: Stmt = syn::parse2(quote! {
            let (#id, #next_list) = frunk::hlist::Plucker::pluck(#list_n_tok);
        })
        .expect("");
        stmts.push(stmt);
        list_n += 1;
    }

    // Generate the "Self { fields... }" part of the block
    let all_fields = [&fields[..], args].concat();
    let list_n_ident = syn::Ident::new(&format!("l{}", list_n), proc_macro2::Span::call_site());
    let self_stmt: Stmt = syn::parse2(quote! {
        return (Self { #(#all_fields,)* }, #list_n_ident);
    })
    .expect("");
    stmts.push(self_stmt);

    Block {
        stmts,
        brace_token: Default::default(),
    }
}

