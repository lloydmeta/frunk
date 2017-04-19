//! Module that holds Coproduct data structures, traits, and implementations
//!
//! Think of "Coproduct" as ad-hoc enums; allowing you to do something like this
//!
//! ```
//! # #[macro_use] extern crate frunk; use frunk::coproduct::*; fn main() {
//! type I32Bool = Coproduct!(i32, bool);
//! let co1: I32Bool = into_coproduct(3);
//! let co2: I32Bool = into_coproduct(true);
//!
//! // Getting stuff
//! let get_from_1a: Option<&i32> = co1.get();
//! let get_from_1b: Option<&bool> = co1.get();
//! assert_eq!(get_from_1a, Some(&3));
//! assert_eq!(get_from_1b, None);
//!
//! let get_from_2a: Option<&i32> = co2.get();
//! let get_from_2b: Option<&bool> = co2.get();
//! assert_eq!(get_from_2a, None);
//! assert_eq!(get_from_2b, Some(&true));
//! # }
//! ```
//!
//! Or, if you want to "fold" over all possible values of a coproduct
//!
//! ```
//! # #[macro_use] extern crate frunk;
//! # #[macro_use] extern crate frunk_core;
//! # use frunk::hlist::*;
//! # use frunk::coproduct::*; fn main() {
//! # type I32Bool = Coproduct!(i32, bool);
//! # let co1: I32Bool = into_coproduct(3);
//! # let co2: I32Bool = into_coproduct(true);
//! let folder = hlist![
//!   |&i| format!("i32 {}", i),
//!   |&b| String::from(if b { "t" } else { "f" })
//! ];
//!
//! assert_eq!(co1.as_ref().fold(&folder), "i32 3".to_string());
//! assert_eq!(co2.as_ref().fold(&folder), "t".to_string());
//!
//! // There is also a value consuming-variant of fold
//!
//! let folded = co1.fold(hlist![
//!   |i| format!("i32 {}", i),
//!   |b| String::from(if b { "t" } else { "f" })
//! ]);
//! assert_eq!(folded, "i32 3".to_string());
//! # }
//! ```

use frunk_core::hlist::*;

/// Enum type representing a Coproduct. Think of this as a Result, but capable
/// of supporting any arbitrary number of types instead of just 2.
///
/// To consctruct a Coproduct, you would typically declare a type using the `Coproduct!` type
/// macro and then use the `into_coproduct` method.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk; use frunk::coproduct::*; fn main() {
/// type I32Bool = Coproduct!(i32, bool);
/// let co1: I32Bool = into_coproduct(3);
/// let get_from_1a: Option<&i32> = co1.get();
/// let get_from_1b: Option<&bool> = co1.get();
/// assert_eq!(get_from_1a, Some(&3));
/// assert_eq!(get_from_1b, None);
/// # }
/// ```
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Coproduct<H, T> {
    /// Coproduct is either H or T, in this case, it is H
    Inl(H),
    /// Coproduct is either H or T, in this case, it is T
    Inr(T),
}

/// Phantom type for signature purposes only (has no value)
///
/// Used by the macro to terminate the Coproduct type signature
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum CNil {}

/// Returns a type signature for a Coproduct of the provided types
///
/// This is a type macro (introduced in Rust 1.13) that makes it easier
/// to write nested type signatures.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk; use frunk::coproduct::*; fn main() {
/// type I32Bool = Coproduct!(i32, bool);
/// let co1: I32Bool = into_coproduct(3);
/// # }
/// ```
#[macro_export]
macro_rules! Coproduct {
    // Nothing
    () => { $crate::coproduct::CNil };

    // Just a single item
    ($single: ty) => {
        $crate::coproduct::Coproduct<$single, CNil>
    };

    ($first: ty, $( $repeated: ty ), +) => {
        $crate::coproduct::Coproduct<$first, Coproduct!($($repeated), *)>
    };

    // <-- Forward trailing comma variants
    ($single: ty,) => {
        Coproduct![$single]
    };

    ($first: ty, $( $repeated: ty, ) +) => {
        Coproduct![$first, $($repeated),*]
    };
    // Forward trailing comma variants -->
}

// <-- For turning something into a Coproduct
pub trait IntoCoproduct<InsertType, Index> {
    fn into(to_insert: InsertType) -> Self;
}

impl<I, Tail> IntoCoproduct<I, Here> for Coproduct<I, Tail> {
    fn into(to_insert: I) -> Self {
        Coproduct::Inl(to_insert)
    }
}

impl<Head, I, Tail, TailIndex> IntoCoproduct<I, There<TailIndex>> for Coproduct<Head, Tail>
    where Tail: IntoCoproduct<I, TailIndex>
{
    fn into(to_insert: I) -> Self {
        let tail_inserted = <Tail as IntoCoproduct<I, TailIndex>>::into(to_insert);
        Coproduct::Inr(tail_inserted)
    }
}

/// Function for returning a Coproduct from a given type
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate frunk; use frunk::coproduct::*; fn main() {
/// type I32Bool = Coproduct!(i32, f32);
/// let co1: I32Bool = into_coproduct(42f32);
/// let get_from_1a: Option<&i32> = co1.get();
/// let get_from_1b: Option<&f32> = co1.get();
/// assert_eq!(get_from_1a, None);
/// assert_eq!(get_from_1b, Some(&42f32));
/// # }
/// ```
pub fn into_coproduct<C, I, Index>(to_into: I) -> C
    where C: IntoCoproduct<I, Index>
{
    <C as IntoCoproduct<I, Index>>::into(to_into)
}
// For turning something into a Coproduct -->

/// Trait for retrieving a coproduct element by type
pub trait CoproductSelector<S, I> {
    fn get(&self) -> Option<&S>;
}

impl<Head, Tail> CoproductSelector<Head, Here> for Coproduct<Head, Tail> {
    fn get(&self) -> Option<&Head> {
        use self::Coproduct::*;
        match *self {
            Inl(ref thing) => Some(thing),
            _ => None, // Impossible
        }
    }
}

impl<Head, FromTail, Tail, TailIndex> CoproductSelector<FromTail, There<TailIndex>>
    for Coproduct<Head, Tail>
    where Tail: CoproductSelector<FromTail, TailIndex>
{
    fn get(&self) -> Option<&FromTail> {
        use self::Coproduct::*;
        match *self {
            Inr(ref rest) => rest.get(),
            _ => None, // Impossible
        }
    }
}

/// Trait for implementing "folding" a Coproduct into a value.
///
/// The Folder should be an HList of closures that correspond (in order, for now..) to the
/// types used in declaring the Coproduct type.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate frunk;
/// # use frunk::coproduct::*;
/// # use frunk::hlist::*; fn main() {
/// type I32StrBool = Coproduct!(i32, f32, bool);
///
/// let co1: I32StrBool = into_coproduct(3);
/// let co2: I32StrBool = into_coproduct(true);
/// let co3: I32StrBool = into_coproduct(42f32);
///
/// let folder = hlist![|&i| format!("int {}", i),
///                     |&f| format!("float {}", f),
///                     |&b| (if b { "t" } else { "f" }).to_string()];
///
/// assert_eq!(co1.as_ref().fold(&folder), "int 3".to_string());
/// assert_eq!(co2.as_ref().fold(&folder), "t".to_string());
/// assert_eq!(co3.as_ref().fold(&folder), "float 42".to_string());
/// # }
/// ```
pub trait CoproductFoldable<Folder, Output> {
    fn fold(self, f: Folder) -> Output;
}

impl<F, R, FTail, CH, CTail> CoproductFoldable<HCons<F, FTail>, R> for Coproduct<CH, CTail>
    where F: FnOnce(CH) -> R,
          CTail: CoproductFoldable<FTail, R>
{
    fn fold(self, f: HCons<F, FTail>) -> R {
        use self::Coproduct::*;
        let f_head = f.head;
        let f_tail = f.tail;
        match self {
            Inl(r) => (f_head)(r),
            Inr(rest) => rest.fold(f_tail),
        }
    }
}

impl<'a, F, R, FTail, CH, CTail> CoproductFoldable<&'a HCons<F, FTail>, R> for &'a Coproduct<CH, CTail>
    where F: Fn(&'a CH) -> R,
          &'a CTail: CoproductFoldable<&'a FTail, R>
{
    fn fold(self, f: &'a HCons<F, FTail>) -> R {
        use self::Coproduct::*;
        let ref f_head = f.head;
        let ref f_tail = f.tail;
        match *self {
            Inl(ref r) => (f_head)(r),
            Inr(ref rest) => <&'a CTail as CoproductFoldable<&'a FTail, R>>::fold(rest, f_tail),
        }
    }
}

/// This is literally impossible; CNil is not instantiable
#[doc(hidden)]
impl<F, R> CoproductFoldable<F, R> for CNil {
    fn fold(self, _: F) -> R {
        unreachable!()
    }
}

/// This is literally impossible; &CNil is not instantiable
#[doc(hidden)]
impl<'a, F, R> CoproductFoldable<&'a F, R> for &'a CNil {
    fn fold(self, _: &'a F) -> R {
        unreachable!()
    }
}

impl <CH, CTail> AsRef<Coproduct<CH, CTail>> for Coproduct<CH, CTail> {
    fn as_ref(&self) -> &Coproduct<CH, CTail> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Coproduct::*;

    #[test]
    fn test_into_coproduct() {
        type I32StrBool = Coproduct!(i32, &'static str, bool);

        let co1: I32StrBool = into_coproduct(3);
        assert_eq!(co1, Inl(3));
        let get_from_1a: Option<&i32> = co1.get();
        let get_from_1b: Option<&bool> = co1.get();
        assert_eq!(get_from_1a, Some(&3));
        assert_eq!(get_from_1b, None);


        let co2: I32StrBool = into_coproduct(false);
        assert_eq!(co2, Inr(Inr(Inl(false))));
        let get_from_2a: Option<&i32> = co2.get();
        let get_from_2b: Option<&bool> = co2.get();
        assert_eq!(get_from_2a, None);
        assert_eq!(get_from_2b, Some(&false));
    }

    #[test]
    fn test_coproduct_fold_consuming() {
        type I32StrBool = Coproduct!(i32, f32, bool);

        let co1: I32StrBool = into_coproduct(3);
        let folded = co1.fold(hlist![|i| format!("int {}", i),
                                      |f| format!("float {}", f),
                                      |b| (if b { "t" } else { "f" }).to_string()]);

        assert_eq!(folded, "int 3".to_string());
    }

    #[test]
    fn test_coproduct_fold_non_consuming() {
        type I32StrBool = Coproduct!(i32, f32, bool);

        let co1: I32StrBool = into_coproduct(3);
        let co2: I32StrBool = into_coproduct(true);
        let co3: I32StrBool = into_coproduct(42f32);

        let folder = hlist![|&i| format!("int {}", i),
                            |&f| format!("float {}", f),
                            |&b| (if b { "t" } else { "f" }).to_string()];

        assert_eq!(co1.as_ref().fold(&folder), "int 3".to_string());
        assert_eq!(co2.as_ref().fold(&folder), "t".to_string());
        assert_eq!(co3.as_ref().fold(&folder), "float 42".to_string());
    }
}
