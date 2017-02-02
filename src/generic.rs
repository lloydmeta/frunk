pub trait Generic {
    type Repr;

    fn into_generic(self) -> Self::Repr;

    fn from_generic(r: Self::Repr) -> Self;
}

macro_rules! with_generic {
    ($(#[$struct_meta:meta])*
    pub struct $name:ident { $($fname:ident : $ftype:ty), *}
    ) => {
        with_generic![(pub) $(#[$struct_meta])* struct $name {$($fname: $ftype) ,*}];
    };

    ($(#[$struct_meta:meta])*
    struct $name:ident { $($fname:ident : $ftype:ty), *}
    ) => {
        with_generic![() $(#[$struct_meta])* struct $name {$($fname: $ftype), *}];
    };

    (
    ($($vis:tt)*)
    $(#[$struct_meta:meta])*
    struct $name:ident { $($fname:ident : $ftype:ty), *}
    ) => {

            $(#[$struct_meta])*
            $($vis)* struct $name {
                $($fname: $ftype,)*
            }

        impl Generic for $name {
            type Repr = Hlist!($($ftype),*);

            fn into_generic(self) -> Self::Repr {
                let $name { $($fname, )* } = self;
                hlist!($($fname),*)
            }

            fn from_generic(r: Self::Repr) -> Self {
                let hlist_pat!($($fname ), *) = r;
                $name { $($fname    : $fname),* }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::hlist::*;
    use super::*;

    with_generic! {
        #[derive(PartialEq, Eq, Debug)]
        pub struct Person {
            first_name: String,
            last_name:  String
        }
    }

    with_generic! {
        #[derive(PartialEq, Eq, Debug)]
        struct PrivatePerson {
            first_name: String,
            last_name:  String
        }
    }

    #[test]
    fn test_pub_struct_from_generic() {
        let h = hlist!("james".to_owned(), "may".to_owned());
        let p = <Person as Generic>::from_generic(h);
        assert_eq!(p,
        Person {
            first_name: "james".to_owned(),
            last_name: "may".to_owned(),
        });
    }

    #[test]
    fn test_nonpub_struct_from_generic() {
        let h = hlist!("james".to_owned(), "may".to_owned());
        let p = <PrivatePerson as Generic>::from_generic(h);
        assert_eq!(p,
        PrivatePerson {
            first_name: "james".to_owned(),
            last_name: "may".to_owned(),
        });
    }
}
