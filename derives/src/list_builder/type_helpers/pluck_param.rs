
#[derive(Clone)]
pub(crate) struct PluckParam(pub(crate) syn::TypeParamBound);
impl From<(syn::Type, u8)> for PluckParam {
    fn from((ty, n): (syn::Type, u8)) -> Self {


        // Create an ident for "LN" where N is the u8 value
        let l_ident = syn::Ident::new(&format!("L{}", n), Span::call_site());
        return Self(syn::parse2(quote!{frunk::hlist::Plucker<#ty, #l_ident>}).unwrap());
    }
}
