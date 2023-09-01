use super::PluckParam;


#[derive(Clone)]
pub(crate) struct WhereLine {
    tp: syn::Type,
    pred: PluckParam
}

impl WhereLine {
    /// entry into the recursive build-up of the where-lines
    ///
    /// generates the base-line (L0: Plucker<SomeType, L1>) then goes into the recursive call
    pub(crate) fn gen_lines_top(types: &[syn::Type]) -> Vec<Self> {
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
    /// `Tn: Pn` -> `<Tn as Pn>::Remainder`
    pub(crate) fn absorb(self) -> syn::Type {
        let WhereLine { tp, pred } = self;
        let pred = pred.0;
        
        let res = quote!{<#tp as #pred>::Remainder};
        syn::parse2(res).expect("absorbing")
    }
}

impl From<WhereLine> for syn::WherePredicate {
    fn from(line: WhereLine) -> Self {
        let WhereLine { tp, pred } = line;
        let bound: syn::TypeParamBound = pred.0.into();
        let predicate = quote! { #tp: #bound };
        syn::parse2(predicate).expect("whereline to pred")
    }
}
/// each line in the where predicate is the type-binding, and the trait it must impl

/// shim allowing PredicateVec::from(line_vec).into() where a `syn::WhereClause` is needed
pub(crate) struct PredicateVec {
    pub(crate) preds: Vec<syn::WherePredicate>
}

impl From<Vec<WhereLine>> for PredicateVec {
    fn from(value: Vec<WhereLine>) -> Self {
        Self{ preds: value.into_iter().map(syn::WherePredicate::from).collect() }
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
