

// struct field or function argument
pub(crate) struct ArgPair {
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
    pub(crate) fn make_args(fields: Vec<ArgPair>) -> impl Iterator<Item = syn::FnArg> {
       std::iter::once(syn::parse2(quote!{l0: L0}).unwrap()).chain(fields.into_iter().map(syn::FnArg::from))
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
