//! Macros all collected into a single module so that the order of `mod`
//! statements in `lib.rs` does not matter.

/// Returns an `HList` based on the values passed in.
///
/// Helps to avoid having to write nested `HCons`.
///
/// # Examples
///
/// ```
/// # use frunk_core::hlist;
/// # fn main() {
/// let h = hlist![13.5f32, "hello", Some(41)];
/// let (h1, (h2, h3)) = h.into_tuple2();
/// assert_eq!(h1, 13.5f32);
/// assert_eq!(h2, "hello");
/// assert_eq!(h3, Some(41));
///
/// // Also works when you have trailing commas
/// let h4 = hlist!["yo",];
/// let h5 = hlist![13.5f32, "hello", Some(41),];
/// assert_eq!(h4, hlist!["yo"]);
/// assert_eq!(h5, hlist![13.5f32, "hello", Some(41)]);
///
/// // Use "...tail" to append an existing list at the end
/// let h6 = hlist![12, ...h5];
/// assert_eq!(h6, hlist![12, 13.5f32, "hello", Some(41)]);
/// # }
/// ```
#[macro_export]
macro_rules! hlist {
    () => { $crate::hlist::HNil };
    (...$rest:expr) => { $rest };
    ($a:expr) => { $crate::hlist![$a,] };
    ($a:expr, $($tok:tt)*) => {
        $crate::hlist::HCons {
            head: $a,
            tail: $crate::hlist![$($tok)*],
        }
    };
}

/// Macro for pattern-matching on HLists.
///
/// Taken from <https://github.com/tbu-/rust-rfcs/blob/master/text/0873-type-macros.md>
///
/// # Examples
///
/// ```
/// # use frunk_core::{hlist, hlist_pat};
/// # fn main() {
/// let h = hlist![13.5f32, "hello", Some(41)];
/// let hlist_pat![a1, a2, a3] = h;
/// assert_eq!(a1, 13.5f32);
/// assert_eq!(a2, "hello");
/// assert_eq!(a3, Some(41));
///
/// // Use "...tail" to match the rest of the list
/// let hlist_pat![b_head, ...b_tail] = h;
/// assert_eq!(b_head, 13.5f32);
/// assert_eq!(b_tail, hlist!["hello", Some(41)]);
///
/// // You can also use "..." to just ignore the rest.
/// let hlist_pat![c, ...] = h;
/// assert_eq!(c, 13.5f32);
/// # }
/// ```
#[macro_export]
macro_rules! hlist_pat {
    () => { $crate::hlist::HNil };
    (...) => { _ };
    (...$rest:pat) => { $rest };
    (_) => { $crate::hlist_pat![_,] };
    ($a:pat) => { $crate::hlist_pat![$a,] };
    (_, $($tok:tt)*) => {
        $crate::hlist::HCons {
            tail: $crate::hlist_pat![$($tok)*],
            ..
        }
    };
    ($a:pat, $($tok:tt)*) => {
        $crate::hlist::HCons {
            head: $a,
            tail: $crate::hlist_pat![$($tok)*],
        }
    };
}

/// Returns a type signature for an HList of the provided types
///
/// This is a type macro (introduced in Rust 1.13) that makes it easier
/// to write nested type signatures.
///
/// # Examples
///
/// ```
/// # use frunk_core::{hlist, HList};
/// # fn main() {
/// let h: HList!(f32, &str, Option<i32>) = hlist![13.5f32, "hello", Some(41)];
///
/// // Use "...Tail" to append another HList type at the end.
/// let h: HList!(f32, ...HList!(&str, Option<i32>)) = hlist![13.5f32, "hello", Some(41)];
/// # }
/// ```
#[macro_export]
macro_rules! HList {
    () => { $crate::hlist::HNil };
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { $crate::HList![$A,] };
    ($A:ty, $($tok:tt)*) => {
        $crate::hlist::HCons<$A, $crate::HList![$($tok)*]>
    };
}

/// Returns a type signature for a Coproduct of the provided types
///
/// This is a type macro (introduced in Rust 1.13) that makes it easier
/// to write nested type signatures.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use frunk_core::Coprod;
///
/// type I32Bool = Coprod!(i32, bool);
/// let co1 = I32Bool::inject(3);
///
/// // Use ...Tail to append another coproduct at the end.
/// let co2 = <Coprod!(&str, String, ...I32Bool)>::inject(3);
/// # }
/// ```
#[macro_export]
macro_rules! Coprod {
    () => { $crate::coproduct::CNil };
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { $crate::Coprod![$A,] };
    ($A:ty, $($tok:tt)*) => {
        $crate::coproduct::Coproduct<$A, $crate::Coprod![$($tok)*]>
    };
}

/// Used for creating a Field
///
/// There are 3 forms of this macro:
///
/// * Create an instance of the `Field` struct with a tuple name type
///   and any given value. The runtime-retrievable static name
///   field will be set to the the concatenation of the types passed in the
///   tuple type used as the first argument.
///
/// # Examples
///
/// ```
/// use frunk::labelled::chars::*;
/// use frunk_core::field;
/// # fn main() {
/// let labelled = field![(n,a,m,e), "joe"];
/// assert_eq!(labelled.name, "name");
/// assert_eq!(labelled.value, "joe")
/// # }
/// ```
///
/// * Create an instance of the `Field` struct with a custom, non-tuple
///   name type and a value. The runtime-retrievable static name field
///   will be set to the stringified version of the type provided.
///
/// ```
/// # fn main() {
/// use frunk_core::field;
/// enum first_name {}
/// let labelled = field![first_name, "Joe"];
/// assert_eq!(labelled.name, "first_name");
/// assert_eq!(labelled.value, "Joe");
/// # }
/// ```
///
/// * Create an instance of the `Field` struct with any name type and value,
///   _and_ a custom name, passed as the last argument in the macro
///
/// ```
/// use frunk::labelled::chars::*;
/// use frunk_core::field;
/// # fn main() {
/// // useful aliasing of our type-level string
/// type age = (a, g, e);
/// let labelled = field![age, 30, "Age"];
/// assert_eq!(labelled.name, "Age");
/// assert_eq!(labelled.value, 30);
/// # }
/// ```
#[macro_export]
macro_rules! field {
    // No name provided and type is a tuple
    (($($repeated: ty),*), $value: expr) => {
        $crate::field!( ($($repeated),*), $value, concat!( $(stringify!($repeated)),* ) )
    };
    // No name provided and type is a tuple, but with trailing commas
    (($($repeated: ty,)*), $value: expr) => {
        $crate::field!( ($($repeated),*), $value )
    };
    // We are provided any type, with no stable name
    ($name_type: ty, $value: expr) => {
        $crate::field!( $name_type, $value, stringify!($name_type) )
    };
    // We are provided any type, with a stable name
    ($name_type: ty, $value: expr, $name: expr) => {
        $crate::labelled::field_with_name::<$name_type,_>($name, $value)
    }
}

/// Returns a polymorphic function for use with mapping/folding heterogeneous
/// types.
///
/// This macro is intended for use with simple scenarios, and doesn't handle
/// trait implementation bounds or where clauses (it might in the future when
/// procedural macros land). If it doesn't work for you, simply implement
/// Func on your own.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use frunk_core::{Coprod, poly_fn};
/// type I32F32Str<'a> = Coprod!(i32, f32, &'a str);
///
/// let co1 = I32F32Str::inject("lollerskates");
/// let folded = co1.fold(poly_fn!(
///   ['a] |x: &'a str| -> i8 { 1 },
///   |x: i32| -> i8 { 2 },
///   |f: f32| -> i8 { 3 },
/// ));
///
/// assert_eq!(folded, 1);
/// # }
#[macro_export]
macro_rules! poly_fn {
    // encountered first func w/ type params
    ([$($tparams: tt),*] |$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block , $($rest: tt)*)
    => { $crate::poly_fn!(
       p~ [$($tparams, )*] |$arg: $arg_typ| -> $ret_typ $body, ~p  f~ ~f $($rest)*
    )};
    // encountered first func w/ type params, trailing comma on tparams
    ([$($tparams: tt, )*] |$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block , $($rest: tt)*)
    => { $crate::poly_fn!(
       p~ [$($tparams, )*] |$arg: $arg_typ| -> $ret_typ $body, ~p  f~ ~f $($rest)*
    )};
    // encountered first func w/o type params
    (|$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block, $($rest: tt)*)
    => { $crate::poly_fn!(
       p~ ~p  f~ |$arg: $arg_typ| -> $ret_typ $body, ~f $($rest)*
    )};

    // encountered non-first func w/ type params
    (p~ $([$($pars: tt, )*] |$p_args: ident : $p_arg_typ: ty| -> $p_ret_typ: ty $p_body: block , )* ~p f~ $(|$f_args: ident : $f_arg_typ: ty| -> $f_ret_typ: ty $f_body: block , )* ~f [$($tparams: tt),*] |$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block , $($rest: tt)*)
    => { $crate::poly_fn!(
       p~ [$($tparams, )*] |$arg: $arg_typ| -> $ret_typ $body, $( [$($pars, )*] |$p_args: $p_arg_typ| -> $p_ret_typ $p_body, )* ~p  f~ $(|$f_args: $f_arg_typ| -> $f_ret_typ $f_body, )* ~f $($rest)*
    )};
    // encountered non-first func w/ type params, trailing comma in tparams
    (p~ $([$($pars: tt, )*] |$p_args: ident : $p_arg_typ: ty| -> $p_ret_typ: ty { $p_body: block }, )* ~p f~ $(|$f_args: ident : $f_arg_typ: ty| -> $f_ret_typ: ty $f_body: block, )* ~f [$($tparams: tt, )*] |$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block, $($rest: tt)*)
    => { $crate::poly_fn!(
       p~ [$($tparams, )*] |$arg: $arg_typ| -> $ret_typ $body, $( [$($pars, )*] |$p_args: $p_arg_typ| -> $p_ret_typ $p_body, )* ~p  f~ $(|$f_args: $f_arg_typ| -> $f_ret_typ $f_body, )* ~f $($rest)*
    )};
    // encountered non-first func w/o type params
    (p~ $([$($pars: tt, )*] |$p_args: ident : $p_arg_typ: ty| -> $p_ret_typ: ty $p_body: block, )* ~p f~ $(|$f_args: ident : $f_arg_typ: ty| -> $f_ret_typ: ty $f_body: block, )* ~f |$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block, $($rest: tt)*)
    => { $crate::poly_fn!(
       p~ $( [$($pars, )*] |$p_args: $p_arg_typ| -> $p_ret_typ $p_body, )* ~p  f~ |$arg: $arg_typ| -> $ret_typ $body, $(|$f_args: $f_arg_typ| -> $f_ret_typ $f_body, )* ~f $($rest)*
    )};

    // last w/ type params, for when there is no trailing comma on the funcs...
    (p~ $([$($pars: tt, )*] |$p_args: ident : $p_arg_typ: ty| -> $p_ret_typ: ty $p_body: block, )* ~p f~ $(|$f_args: ident : $f_arg_typ: ty| -> $f_ret_typ: ty $f_body: block, )* ~f [$($tparams: tt),*] |$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block)
    => { $crate::poly_fn!(
       p~ [$($tparams, )*] |$arg: $arg_typ| -> $ret_typ $body, $( [$($pars, )*] |$p_args: $p_arg_typ| -> $p_ret_typ $p_body, )* ~p  f~ $(|$f_args: $f_arg_typ| -> $f_ret_typ $f_body, )* ~f
    )};
    // last w/ type params, for when there is a trailing comma in tparams, but no trailing comma on the funcs..
    (p~ $([$($pars: tt, )*] |$p_args: ident : $p_arg_typ: ty| -> $p_ret_typ: ty $p_body: block, )* ~p f~ $(|$f_args: ident : $f_arg_typ: ty| -> $f_ret_typ: ty $f_body: block, )* ~f [$($tparams: tt, )*] |$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block)
    => { $crate::poly_fn!(
       p~ [$($tparams, )*] |$arg: $arg_typ| -> $ret_typ $body, $( [$($pars, )*] |$p_args: $p_arg_typ| -> $p_ret_typ $p_body, )* ~p  f~ $(|$f_args: $f_arg_typ| -> $f_ret_typ $f_body, )* ~f
    )};
    // last w/o type params, for when there is no trailing comma on the funcs...
    (p~ $([$($pars: tt)*] |$p_args: ident : $p_arg_typ: ty| -> $p_ret_typ: ty $p_body: block, )* ~p f~ $(|$f_args: ident : $f_arg_typ: ty| -> $f_ret_typ: ty $f_body: block, )* ~f |$arg: ident : $arg_typ: ty| -> $ret_typ: ty $body: block)
    => { $crate::poly_fn!(
       p~ $( [$($pars, )*] |$p_args: $p_arg_typ| -> $p_ret_typ $p_body, )* ~p  f~ |$arg: $arg_typ| -> $ret_typ $body, $(|$f_args: $f_arg_typ| -> $f_ret_typ $f_body, )* ~f
    )};

    // unroll
    (p~ $([$($pars: tt, )*] |$p_args: ident : $p_arg_typ: ty| -> $p_ret_typ: ty $p_body: block, )* ~p f~ $(|$args: ident : $arg_typ: ty| -> $ret_typ: ty $body: block, )* ~f) => {{
        struct F;
        $(
            impl<$($pars,)*> $crate::traits::Func<$p_arg_typ> for F {
                type Output = $p_ret_typ;

                fn call($p_args: $p_arg_typ) -> Self::Output { $p_body }
            }
        )*
        $(
            impl $crate::traits::Func<$arg_typ> for F {
                type Output = $ret_typ;

                fn call($args: $arg_typ) -> Self::Output { $body }
            }
        )*
        $crate::traits::Poly(F)
    }}
}

#[cfg(test)]
mod tests {
    #[allow(clippy::diverging_sub_expression)]
    #[test]
    fn trailing_commas() {
        use crate::test_structs::unit_copy::{A, B};

        let hlist_pat![]: HList![] = hlist![];
        let hlist_pat![A]: HList![A] = hlist![A];
        let hlist_pat![A,]: HList![A,] = hlist![A,];
        let hlist_pat![A, B]: HList![A, B] = hlist![A, B];
        let hlist_pat![A, B,]: HList![A, B,] = hlist![A, B,];

        let falsum = || false;
        if falsum() {
            let _: Coprod![] = panic!();
        }
        if falsum() {
            let _: Coprod![A] = panic!();
        }
        if falsum() {
            let _: Coprod![A,] = panic!();
        }
        if falsum() {
            let _: Coprod![A, B] = panic!();
        }
        if falsum() {
            let _: Coprod![A, B,] = panic!();
        }
    }

    #[test]
    fn ellipsis_tail() {
        use crate::coproduct::Coproduct;
        use crate::test_structs::unit_copy::{A, B, C};

        // hlist: accepted locations, and consistency between macros
        let hlist_pat![...hlist_pat![C]]: HList![...HList![C]] = { hlist![...hlist![C]] };
        let hlist_pat![A, ...hlist_pat![C]]: HList![A, ...HList![C]] = { hlist![A, ...hlist![C]] };
        let hlist_pat![A, B, ...hlist_pat![C]]: HList![A, B, ...HList![C]] =
            { hlist![A, B, ...hlist![C]] };

        // hlist: ellipsis semantics
        //   (by pairing an ellipsis call with a non-ellipsis call)
        let hlist_pat![A, B, C] = hlist![A, ...hlist![B, C]];
        let hlist_pat![A, ...hlist_pat![B, C]] = hlist![A, B, C];

        // coprod: accepted locations and semantics
        let choice: Coprod![A, B, C] = Coproduct::inject(A);
        let _: Coprod![...Coprod![A, B, C]] = choice;
        let _: Coprod![A, ...Coprod![B, C]] = choice;
        let _: Coprod![A, B, ...Coprod![C]] = choice;
    }

    #[test]
    fn ellipsis_ignore() {
        use crate::test_structs::unit_copy::{A, B, C, D, E};

        // '...' accepted locations
        let hlist_pat![...] = hlist![A, B, C, D, E];
        let hlist_pat![A, ...] = hlist![A, B, C, D, E];
        let hlist_pat![A, B, ...] = hlist![A, B, C, D, E];
    }

    #[test]
    fn poly_fn_macro_test() {
        let h = hlist![9000, "joe", 41f32, "schmoe", 50];
        let h2 = h.map(poly_fn!(
            |x: i32| -> bool { x > 100 },
            |_x: f32| -> &'static str { "dummy" },
            ['a] |x: &'a str| -> usize { x.len() }
        ));
        assert_eq!(h2, hlist![true, 3, "dummy", 6, false]);
    }

    #[test]
    fn poly_fn_macro_coproduct_test() {
        type I32F32StrBool<'a> = Coprod!(i32, f32, &'a str);

        let co1 = I32F32StrBool::inject("lollerskates");
        let folded = co1.fold(poly_fn!(
            ['a] |_x: &'a str| -> i8 { 1 },
            |_x: i32| -> i8 { 2 },
            |_f: f32| -> i8 { 3 },
        ));
        assert_eq!(folded, 1);
    }

    #[test]
    fn poly_fn_macro_trailing_commas_test() {
        let h = hlist![9000, "joe", 41f32, "schmoe", 50];
        let h2 = h.map(poly_fn!(
            |x: i32| -> bool { x > 100 },
            |_x: f32| -> &'static str { "dummy" },
            ['a,] |x: &'a str| -> usize { x.len() },
        ));
        assert_eq!(h2, hlist![true, 3, "dummy", 6, false]);
    }

    #[test]
    fn poly_fn_macro_multiline_bodies_test() {
        let h = hlist![9000, 1, -1];
        let h2 = h.map(poly_fn!(|x: i32| -> bool {
            let a = if x > 100 { 1 } else { -1 };
            a > 0
        },));
        assert_eq!(h2, hlist![true, false, false]);
    }

    #[test]
    #[deny(clippy::unneeded_field_pattern)]
    fn unneeded_field_pattern() {
        let hlist_pat![_, _] = hlist![1, 2];
        let hlist_pat![foo, _, baz] = hlist!["foo", "bar", "baz"];
        assert_eq!(foo, "foo");
        assert_eq!(baz, "baz");
    }
}
