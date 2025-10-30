//! This module is held separate to put generated variadic generics for tuples
//! at the end of the documentation so as to not disturb the reader when reading
//! documentation.
//!
//! ```
//! # use frunk_core::hlist::*;
//! # use frunk_core::{hlist, HList};
//! # fn main() {
//! let h = hlist![ 42f32, true, "hello" ];
//! let t: (f32, bool, &str) = h.into();
//! assert_eq!(t, (42f32, true, "hello"));
//!
//! let t2 = (999, false, "world");
//! let h2: HList![ isize, bool, &str ] = t2.into();
//! assert_eq!(h2, hlist![ 999, false, "world" ]);
//! # }
//! ```

use crate::generic::Generic;
use crate::{hlist, HList};

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
        for HList![$( $dtype, )* $sone, $( $stype, )* ] {
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
            type Repr = HList![$t];
            fn into(self) -> Self::Repr { hlist![self.0] }
            fn from(r: Self::Repr) -> Self { (r.head,) }
        }

        impl<$t> From<($t,)> for HList![$t] {
            fn from(tup: ($t,)) -> Self { Generic::into(tup) }
        }

        #[allow(clippy::from_over_into)]
        impl<$t> Into<($t,)> for HList![$t] {
            fn into(self) -> ($t,) { Generic::from(self) }
        }
    };

    ( $type1: ident, $( $type: ident ),* ) => {
        tup_iso!($( $type ),*);

        impl<$type1, $($type),*> Generic for ($type1, $($type),*,) {
            type Repr = HList![$type1, $($type),*,];

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
        for HList![ $type1, $($type),*, ] {
            fn from(tup: ( $type1, $( $type ),*, ) ) -> Self {
                Generic::into(tup)
            }
        }

        #[allow(clippy::from_over_into)]
        impl< $type1, $($type),*> Into< ( $type1, $($type),*, ) >
        for HList![ $type1, $($type),*, ] {
            fn into(self) -> ( $type1, $( $type ),*, ) {
                Generic::from(self)
            }
        }
    };
}

impl Generic for () {
    type Repr = HList![];
    fn into(self) -> Self::Repr {
        hlist![]
    }
    fn from(_: Self::Repr) -> Self {}
}

impl From<()> for HList![] {
    fn from(_: ()) -> Self {
        hlist![]
    }
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
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13,);
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15,;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16,);
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,);
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,);
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,);
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, );
tup_def!( T0 ; F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, ;
               T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, );

tup_iso!(
    T23, T22, T21, T20, T19, T18, T17, T16, T15, T14, T13, T12, T11, T10, T9, T8, T7, T6, T5, T4,
    T3, T2, T1, T0
);
