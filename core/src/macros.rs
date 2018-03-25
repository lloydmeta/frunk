//! Macros all collected into a single module so that the order of `mod`
//! statements in `lib.rs` does not matter.

/// Returns an `HList` based on the values passed in.
///
/// Helps to avoid having to write nested `HCons`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk_core; use ::frunk_core::hlist::*; fn main() {
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
    ($a:expr) => { hlist![$a,] };
    ($a:expr, $($tok:tt)*) => {
        $crate::hlist::HCons {
            head: $a,
            tail: hlist![$($tok)*],
        }
    };
}

/// Macro for pattern-matching on HLists.
///
/// Taken from https://github.com/tbu-/rust-rfcs/blob/master/text/0873-type-macros.md
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk_core; fn main() {
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
    ($a:pat) => { hlist_pat![$a,] };
    ($a:pat, $($tok:tt)*) => {
        $crate::hlist::HCons {
            head: $a,
            tail: hlist_pat![$($tok)*],
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
/// # #[macro_use] extern crate frunk_core; fn main() {
/// let h: Hlist!(f32, &str, Option<i32>) = hlist![13.5f32, "hello", Some(41)];
///
/// // Use "...Tail" to append another HList type at the end.
/// let h: Hlist!(f32, ...Hlist!(&str, Option<i32>)) = hlist![13.5f32, "hello", Some(41)];
/// # }
/// ```
#[macro_export]
macro_rules! Hlist {
    () => { $crate::hlist::HNil };
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { Hlist![$A,] };
    ($A:ty, $($tok:tt)*) => {
        $crate::hlist::HCons<$A, Hlist![$($tok)*]>
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
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
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
    ($A:ty) => { Coprod![$A,] };
    ($A:ty, $($tok:tt)*) => {
        $crate::coproduct::Coproduct<$A, Coprod![$($tok)*]>
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
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
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
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
/// # fn main() {
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
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
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
        field!( ($($repeated),*), $value, concat!( $(stringify!($repeated)),* ) )
    };
    // No name provided and type is a tuple, but with trailing commas
    (($($repeated: ty,)*), $value: expr) => {
        field!( ($($repeated),*), $value )
    };
    // We are provided any type, with no stable name
    ($name_type: ty, $value: expr) => {
        field!( $name_type, $value, stringify!($name_type) )
    };
    // We are provided any type, with a stable name
    ($name_type: ty, $value: expr, $name: expr) => {
        $crate::labelled::field_with_name::<$name_type,_>($name, $value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn trailing_commas() {
        use ::test_structs::unit_copy::{A, B};

        let hlist_pat![]:      Hlist![]      = hlist![];
        let hlist_pat![A]:     Hlist![A]     = hlist![A];
        let hlist_pat![A,]:    Hlist![A,]    = hlist![A,];
        let hlist_pat![A, B]:  Hlist![A, B]  = hlist![A, B];
        let hlist_pat![A, B,]: Hlist![A, B,] = hlist![A, B,];

        let falsum = || false;
        if falsum() { let _: Coprod![]      = panic!(); }
        if falsum() { let _: Coprod![A]     = panic!(); }
        if falsum() { let _: Coprod![A,]    = panic!(); }
        if falsum() { let _: Coprod![A, B]  = panic!(); }
        if falsum() { let _: Coprod![A, B,] = panic!(); }
    }

    #[test]
    fn ellipsis_tail() {
        use ::test_structs::unit_copy::{A, B, C};
        use ::coproduct::*;

        // hlist: accepted locations, and consistency between macros
        let hlist_pat![...hlist_pat![C]]: Hlist![...Hlist![C]] = {
            hlist![...hlist![C]]
        };
        let hlist_pat![A, ...hlist_pat![C]]: Hlist![A, ...Hlist![C]] = {
            hlist![A, ...hlist![C]]
        };
        let hlist_pat![A, B, ...hlist_pat![C]]: Hlist![A, B, ...Hlist![C]] = {
            hlist![A, B, ...hlist![C]]
        };

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
        use ::test_structs::unit_copy::{A, B, C, D, E};

        // '...' accepted locations
        let hlist_pat![...]       = hlist![A, B, C, D, E];
        let hlist_pat![A, ...]    = hlist![A, B, C, D, E];
        let hlist_pat![A, B, ...] = hlist![A, B, C, D, E];
    }
}
