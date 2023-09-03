
pub(crate) enum Annotation {
    Plucker{ty: syn::Type, map: syn::Expr},
    Ignore,
}

pub(crate) enum AnnoErr {
    XOR,
    NoMatch
}
impl core::convert::TryFrom<&[syn::Attribute]> for Annotation {
    type Error = AnnoErr;
    fn try_from(attrs: &[syn::Attribute]) -> Result<Self, Self::Error> {
        let mut annos = attrs.iter().filter_map(|attr| Self::try_from(attr).ok());
        let Some(fst) = annos.next() else {
            return Err(AnnoErr::NoMatch);
        };

        if let Some(_) = annos.next() {
            return Err(AnnoErr::XOR);
        }
        Ok(fst)

    }
}
impl TryFrom<&syn::Attribute> for Annotation {
    type Error = ();

    fn try_from(attr: &syn::Attribute) -> Result<Self, Self::Error> {
        match attr.path().get_ident().map(|id| quote!{#id}.to_string()) {
            Some(ref s) if s == "plucker" => {
                let arg_parser = |input: syn::parse::ParseStream| {
                    // Eg, with the arg: u8, map=core::convert::From::from(arg_name)


                    // consume leading type
                    let ty: syn::Type = input.parse().expect("type here");
                    // , map=core::convert::From::from(arg_name)
                    // consume comma
                    let _ = input.parse::<Option<syn::Token![,]>>().expect("comma here");
                    // map=core::convert::From::from(arg_name)
                    // consume `map`
                    let _ = input.parse::<syn::Ident>().expect("'map' here");
                    // =core::convert::From::from(arg_name)
                    // consume `=`
                    input.parse::<syn::Token![=]>().expect("equalse sign here");
                    // core::convert::From::from(arg_name)
                    // parse in the expression
                    let mapping = input.parse().expect("parsing expression");
                    Ok((ty, mapping))

                };

                let (ty, map) = attr.parse_args_with(arg_parser).expect("mapping with arg parser");
                Ok(Annotation::Plucker{ty, map})
            }
            Some(ref s) if s == "list_build_ignore" => Ok(Self::Ignore),
            _ => Err(())

        }

    }
}

