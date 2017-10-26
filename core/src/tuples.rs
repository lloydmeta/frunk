//! This module is held separate to put generated variadic generics for tuples
//! at the end of the documentation so as to not disturb the reader when reading
//! documentation.
//!
//! ```
//! # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
//! let h = hlist![ 42f32, true, "hello" ];
//! let t: (f32, bool, &str) = h.into();
//! assert_eq!(t, (42f32, true, "hello"));
//! # }
//! ```

use generic::Generic;

macro_rules! tup_def {
    ( $($dtype: ident),* ; ; ) => {};
    ( $($dtype: ident),* ;
      $fone: ident $(, $ftype: ident)*, ;
      $sone: ident $(, $stype: ident)*,
    ) => {
        tup_def!( $($dtype,)* $sone ; $($ftype,)* ; $($stype,)* );

        impl< $($dtype: Default,)*
                $fone , $sone:  From<$fone> ,
              $($ftype, $stype: From<$ftype>,)*
        > From< ( $fone, $( $ftype, )* ) >
        for Hlist![$( $dtype, )* $sone, $( $stype, )* ] {
            fn from(f: ( $fone, $( $ftype, )* )) -> Self {
                #[allow(non_snake_case)]
                let ( $fone, $( $ftype, )* ) = f;
                hlist![$($dtype::default(),)* $fone.into(), $($ftype.into(),)*]
            }
        }
    };
}

macro_rules! tup_iso {
    ( $t: ident ) => {
        impl<$t> Generic for ($t,) {
            type Repr = Hlist![$t];
            fn into(self) -> Self::Repr { hlist![self.0] }
            fn from(r: Self::Repr) -> Self { (r.head,) }
        }

        impl<$t> From<($t,)> for Hlist![$t] {
            fn from(tup: ($t,)) -> Self { Generic::into(tup) }
        }

        impl<$t> Into<($t,)> for Hlist![$t] {
            fn into(self) -> ($t,) { Generic::from(self) }
        }
    };

    ( $type1: ident, $( $type: ident ),* ) => {
        tup_iso!($( $type ),*);

        impl<$type1, $($type),*> Generic for ($type1, $($type),*,) {
            type Repr = Hlist![$type1, $($type),*,];

            fn into(self) -> Self::Repr {
                #[allow(non_snake_case)]
                let ($type1, $($type),*,) = self;
                hlist![$type1, $($type),*,]
            }

            fn from(r: Self::Repr) -> Self {
                #[allow(non_snake_case)]
                let hlist_pat![$type1, $($type),*,] = r;
                ($type1, $($type),*,)
            }
        }

        impl<$type1, $($type),*> From< ( $type1, $($type),*, ) >
        for Hlist![ $type1, $($type),*, ] {
            fn from(tup: ( $type1, $( $type ),*, ) ) -> Self {
                Generic::into(tup)
            }
        }

        impl< $type1, $($type),*> Into< ( $type1, $($type),*, ) >
        for Hlist![ $type1, $($type),*, ] {
            fn into(self) -> ( $type1, $( $type ),*, ) {
                Generic::from(self)
            }
        }
    };
}

impl Generic for () {
    type Repr = Hlist![];
    fn into(self) -> Self::Repr {
        hlist![]
    }
    fn from(_: Self::Repr) -> Self {
        ()
    }
}

impl From<()> for Hlist![] {
    fn from(_: ()) -> Self { hlist![] }
}

tup_def!( T0 ; F1, ; T1, );
tup_def!( T0 ; F1, F2, ;
               T1, T2, );
tup_def!( T0 ; F1, F2, F3, ;
               T1, T2, T3, );
tup_def!( T0 ; F1, F2, F3, F4, ;
               T1, T2, T3, T4, );
tup_def!( T0 ; F1, F2, F3, F4, F5, ;
               T1, T2, T3, T4, T5, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, ;
               T1, T2, T3, T4, T5, T6, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, ;
               T1, T2, T3, T4, T5, T6, T7, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, ;
               T1, T2, T3, T4, T5, T6, T7, T8, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, );

tup_iso!(T9, T8, T7, T6, T5, T4, T3, T2, T1, T0);
