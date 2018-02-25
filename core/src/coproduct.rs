//! Module that holds Coproduct data structures, traits, and implementations
//!
//! Think of "Coproduct" as ad-hoc enums; allowing you to do something like this
//!
//! ```
//! # #[macro_use] extern crate frunk_core;
//! # use frunk_core::coproduct::*;
//! # fn main() {
//! // For simplicity, assign our Coproduct type to a type alias
//! // This is purely optional.
//! type I32Bool = Coprod!(i32, bool);
//! // Inject things into our Coproduct type
//! let co1 = I32Bool::inject(3);
//! let co2 = I32Bool::inject(true);
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
//!
//! // *Taking* stuff (by value)
//! let take_from_1a: Option<i32> = co1.take();
//! assert_eq!(take_from_1a, Some(3));
//!
//! // Or with a Result
//! let uninject_from_1a: Result<i32, _> = co1.uninject();
//! let uninject_from_1b: Result<bool, _> = co1.uninject();
//! assert_eq!(uninject_from_1a, Ok(3));
//! assert!(uninject_from_1b.is_err());
//! # }
//! ```
//!
//! Or, if you want to "fold" over all possible values of a coproduct
//!
//! ```
//! # #[macro_use] extern crate frunk_core;
//! # use frunk_core::hlist::*;
//! # use frunk_core::coproduct::*;
//! # fn main() {
//! # type I32Bool = Coprod!(i32, bool);
//! # let co1 = I32Bool::inject(3);
//! # let co2 = I32Bool::inject(true);
//! // In the below, we use unimplemented!() to make it obvious hat we know what type of
//! // item is inside our coproducts co1 and co2 but in real life, you should be writing
//! // complete functions for all the cases when folding coproducts
//! assert_eq!(
//!     co1.as_ref().fold(hlist![|&i| format!("i32 {}", i),
//!                              |&b| unimplemented!() /* we know this won't happen for co1 */ ]),
//!     "i32 3".to_string());
//! assert_eq!(
//!     co2.as_ref().fold(hlist![|&i| unimplemented!() /* we know this won't happen for co2 */,
//!                              |&b| String::from(if b { "t" } else { "f" })]),
//!     "t".to_string());
//!
//! // There is also a value consuming-variant of fold
//!
//! let folded = co1.fold(hlist![|i| format!("i32 {}", i),
//!                              |b| String::from(if b { "t" } else { "f" })]);
//! assert_eq!(folded, "i32 3".to_string());
//! # }
//! ```

use hlist::*;

/// Enum type representing a Coproduct. Think of this as a Result, but capable
/// of supporting any arbitrary number of types instead of just 2.
///
/// To consctruct a Coproduct, you would typically declare a type using the `Coprod!` type
/// macro and then use the `inject` method.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// type I32Bool = Coprod!(i32, bool);
/// let co1 = I32Bool::inject(3);
/// let get_from_1a: Option<&i32> = co1.get();
/// let get_from_1b: Option<&bool> = co1.get();
/// assert_eq!(get_from_1a, Some(&3));
/// assert_eq!(get_from_1b, None);
/// # }
/// ```
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Coproduct<H, T> {
    /// Coproduct is either H or T, in this case, it is H
    Inl(H),
    /// Coproduct is either H or T, in this case, it is T
    Inr(T),
}

/// Phantom type for signature purposes only (has no value)
///
/// Used by the macro to terminate the Coproduct type signature
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum CNil {}

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
/// # }
/// ```
#[macro_export]
macro_rules! Coprod {
    // Nothing
    () => { $crate::coproduct::CNil };

    // Just a single item
    ($single: ty) => {
        $crate::coproduct::Coproduct<$single, $crate::coproduct::CNil>
    };

    ($first: ty, $( $repeated: ty ), +) => {
        $crate::coproduct::Coproduct<$first, Coprod!($($repeated), *)>
    };

    // <-- Forward trailing comma variants
    ($single: ty,) => {
        Coprod![$single]
    };

    ($first: ty, $( $repeated: ty, ) +) => {
        Coprod![$first, $($repeated),*]
    };
    // Forward trailing comma variants -->
}

/// Trait for injecting something into a coproduct
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// type I32F32 = Coprod!(i32, f32);
/// let co1 = I32F32::inject(42f32);
/// let get_from_1a: Option<&i32> = co1.get();
/// let get_from_1b: Option<&f32> = co1.get();
/// assert_eq!(get_from_1a, None);
/// assert_eq!(get_from_1b, Some(&42f32));
/// # }
/// ```
pub trait CoprodInjector<InjectType, Index> {
    fn inject(to_insert: InjectType) -> Self;
}

impl<I, Tail> CoprodInjector<I, Here> for Coproduct<I, Tail> {
    fn inject(to_insert: I) -> Self {
        Coproduct::Inl(to_insert)
    }
}

impl<Head, I, Tail, TailIndex> CoprodInjector<I, There<TailIndex>> for Coproduct<Head, Tail>
where
    Tail: CoprodInjector<I, TailIndex>,
{
    fn inject(to_insert: I) -> Self {
        let tail_inserted = <Tail as CoprodInjector<I, TailIndex>>::inject(to_insert);
        Coproduct::Inr(tail_inserted)
    }
}

// For turning something into a Coproduct -->

/// Trait for retrieving a coproduct element reference by type.
///
/// Returns an Option<&YourType> (notice that the inside of the option is a reference)
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// type I32F32 = Coprod!(i32, f32);
///
/// let co1 = I32F32::inject(42f32);
///
/// let get_from_1a: Option<&i32> = co1.get();
/// let get_from_1b: Option<&f32> = co1.get();
/// assert_eq!(get_from_1a, None);
/// assert_eq!(get_from_1b, Some(&42f32));
/// # }
/// ```
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
where
    Tail: CoproductSelector<FromTail, TailIndex>,
{
    fn get(&self) -> Option<&FromTail> {
        use self::Coproduct::*;
        match *self {
            Inr(ref rest) => rest.get(),
            _ => None, // Impossible
        }
    }
}

/// Trait for retrieving a coproduct element by type.
///
/// Returns an Option<YourType> (notice that the inside of the option is a value)
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// type I32F32 = Coprod!(i32, f32);
///
/// let co1 = I32F32::inject(42f32);
///
/// let get_from_1a: Option<i32> = co1.take();
/// let get_from_1b: Option<f32> = co1.take();
/// assert_eq!(get_from_1a, None);
/// assert_eq!(get_from_1b, Some(42f32));
/// # }
/// ```
pub trait CoproductTaker<S, I> {
    fn take(self) -> Option<S>;
}

impl<Head, Tail> CoproductTaker<Head, Here> for Coproduct<Head, Tail> {
    fn take(self) -> Option<Head> {
        use self::Coproduct::*;
        match self {
            Inl(thing) => Some(thing),
            _ => None, // Impossible
        }
    }
}

impl<Head, FromTail, Tail, TailIndex> CoproductTaker<FromTail, There<TailIndex>>
    for Coproduct<Head, Tail>
where
    Tail: CoproductTaker<FromTail, TailIndex>,
{
    fn take(self) -> Option<FromTail> {
        use self::Coproduct::*;
        match self {
            Inr(rest) => rest.take(),
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
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # use frunk_core::hlist::*;
/// # fn main() {
/// type I32F32StrBool = Coprod!(i32, f32, bool);
///
/// let co1 = I32F32StrBool::inject(3);
/// let co2 = I32F32StrBool::inject(true);
/// let co3 = I32F32StrBool::inject(42f32);
///
/// let folder = hlist![|&i| format!("int {}", i),
///                     |&f| format!("float {}", f),
///                     |&b| (if b { "t" } else { "f" }).to_string()];
///
/// assert_eq!(co1.as_ref().fold(folder), "int 3".to_string());
/// # }
/// ```
pub trait CoproductFoldable<Folder, Output> {
    fn fold(self, f: Folder) -> Output;
}

impl<F, R, FTail, CH, CTail> CoproductFoldable<HCons<F, FTail>, R> for Coproduct<CH, CTail>
where
    F: FnOnce(CH) -> R,
    CTail: CoproductFoldable<FTail, R>,
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

impl<'a, F, R, FTail, CH, CTail> CoproductFoldable<HCons<F, FTail>, R> for &'a Coproduct<CH, CTail>
where
    F: FnOnce(&'a CH) -> R,
    &'a CTail: CoproductFoldable<FTail, R>,
{
    fn fold(self, f: HCons<F, FTail>) -> R {
        use self::Coproduct::*;
        let f_head = f.head;
        let f_tail = f.tail;
        match *self {
            Inl(ref r) => (f_head)(r),
            Inr(ref rest) => <&'a CTail as CoproductFoldable<FTail, R>>::fold(rest, f_tail),
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
impl<'a, F, R> CoproductFoldable<F, R> for &'a CNil {
    fn fold(self, _: F) -> R {
        unreachable!()
    }
}

impl<CH, CTail> AsRef<Coproduct<CH, CTail>> for Coproduct<CH, CTail> {
    fn as_ref(&self) -> &Coproduct<CH, CTail> {
        self
    }
}

/// Trait for extracting a value from a coproduct in an exhaustive way.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// type I32F32 = Coprod!(i32, f32);
/// let co1 = I32F32::inject(42f32);
/// let get_from_1a: Result<i32, _> = co1.uninject();
/// let get_from_1b: Result<f32, _> = co1.uninject();
/// assert!(get_from_1a.is_err());
/// assert_eq!(get_from_1b, Ok(42f32));
/// # }
/// ```
pub trait CoprodUninjector<T, U, Idx>: CoprodInjector<T, Idx> {
    /// Attempts to get a value from the union.
    fn uninject(self) -> Result<T, U>;
}

impl<Hd, Tl> CoprodUninjector<Hd, Tl, Here> for Coproduct<Hd, Tl> {
    fn uninject(self) -> Result<Hd, Tl> {
        match self {
            Coproduct::Inl(h) => Ok(h),
            Coproduct::Inr(t) => Err(t),
        }
    }
}

impl<Hd, Tl, T, U, N> CoprodUninjector<T, Coproduct<Hd, U>, There<N>> for Coproduct<Hd, Tl>
where
    Tl: CoprodUninjector<T, U, N>,
{
    fn uninject(self) -> Result<T, Coproduct<Hd, U>> {
        match self {
            Coproduct::Inl(h) => Err(Coproduct::Inl(h)),
            Coproduct::Inr(t) => t.uninject().map_err(Coproduct::Inr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Coproduct::*;

    #[test]
    fn test_coproduct_inject() {
        type I32StrBool = Coprod!(i32, &'static str, bool);

        let co1 = I32StrBool::inject(3);
        assert_eq!(co1, Inl(3));
        let get_from_1a: Option<&i32> = co1.get();
        let get_from_1b: Option<&bool> = co1.get();
        assert_eq!(get_from_1a, Some(&3));
        assert_eq!(get_from_1b, None);

        let co2 = I32StrBool::inject(false);
        assert_eq!(co2, Inr(Inr(Inl(false))));
        let get_from_2a: Option<&i32> = co2.get();
        let get_from_2b: Option<&bool> = co2.get();
        assert_eq!(get_from_2a, None);
        assert_eq!(get_from_2b, Some(&false));
    }

    #[test]
    fn test_coproduct_fold_consuming() {
        type I32F32StrBool = Coprod!(i32, f32, bool);

        let co1 = I32F32StrBool::inject(3);
        let folded = co1.fold(hlist![
            |i| format!("int {}", i),
            |f| format!("float {}", f),
            |b| (if b { "t" } else { "f" }).to_string(),
        ]);

        assert_eq!(folded, "int 3".to_string());
    }

    #[test]
    fn test_coproduct_fold_non_consuming() {
        type I32F32Bool = Coprod!(i32, f32, bool);

        let co1 = I32F32Bool::inject(3);
        let co2 = I32F32Bool::inject(true);
        let co3 = I32F32Bool::inject(42f32);

        assert_eq!(
            co1.as_ref().fold(hlist![
                |&i| format!("int {}", i),
                |&f| format!("float {}", f),
                |&b| (if b { "t" } else { "f" }).to_string(),
            ]),
            "int 3".to_string()
        );
        assert_eq!(
            co2.as_ref().fold(hlist![
                |&i| format!("int {}", i),
                |&f| format!("float {}", f),
                |&b| (if b { "t" } else { "f" }).to_string(),
            ]),
            "t".to_string()
        );
        assert_eq!(
            co3.as_ref().fold(hlist![
                |&i| format!("int {}", i),
                |&f| format!("float {}", f),
                |&b| (if b { "t" } else { "f" }).to_string(),
            ]),
            "float 42".to_string()
        );
    }

    #[test]
    fn test_coproduct_uninject() {
        type I32StrBool = Coprod!(i32, &'static str, bool);

        let co1 = I32StrBool::inject(3);
        let co2 = I32StrBool::inject("hello");
        let co3 = I32StrBool::inject(false);

        let uninject_i32_co1: Result<i32, _> = co1.uninject();
        let uninject_str_co1: Result<&'static str, _> = co1.uninject();
        let uninject_bool_co1: Result<bool, _> = co1.uninject();
        assert_eq!(uninject_i32_co1, Ok(3));
        assert!(uninject_str_co1.is_err());
        assert!(uninject_bool_co1.is_err());

        let uninject_i32_co2: Result<i32, _> = co2.uninject();
        let uninject_str_co2: Result<&'static str, _> = co2.uninject();
        let uninject_bool_co2: Result<bool, _> = co2.uninject();
        assert!(uninject_i32_co2.is_err());
        assert_eq!(uninject_str_co2, Ok("hello"));
        assert!(uninject_bool_co2.is_err());

        let uninject_i32_co3: Result<i32, _> = co3.uninject();
        let uninject_str_co3: Result<&'static str, _> = co3.uninject();
        let uninject_bool_co3: Result<bool, _> = co3.uninject();
        assert!(uninject_i32_co2.is_err());
        assert!(uninject_str_co3.is_err());
        assert_eq!(uninject_bool_co3, Ok(false));
    }
}
