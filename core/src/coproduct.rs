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
/// # Examples
///
/// Basic usage:
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// type I32F32 = Coprod!(i32, f32);
/// let co1 = I32F32::inject(42f32);
///
/// let uninject_1a: Result<i32, _> = co1.uninject();
/// let uninject_1b: Result<f32, _> = co1.uninject();
/// assert!(uninject_1a.is_err());
/// assert_eq!(uninject_1b, Ok(42f32));
/// # }
/// ```
///
/// Chaining calls for an exhaustive match:
///
/// ```rust
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// type I32F32 = Coprod!(i32, f32);
///
/// // be aware that this particular example could be
/// // written far more succinctly using `fold`.
/// fn handle_i32_f32(co: I32F32) -> f32 {
///     // Remove i32 from the coproduct
///     let res: Result<i32, _> = co.uninject();
///     let co = match res {
///         Ok(x) => return (2 * x) as f32,
///         Err(co) => co,
///     };
///
///     // Remove f32 from the coproduct
///     let res: Result<f32, _> = co.uninject();
///     let co = match res {
///         Ok(x) => return 2.0 * x,
///         Err(co) => co
///     };
///
///     // now co is empty;
///     match co { /* unreachable */ }
/// }
///
/// assert_eq!(handle_i32_f32(I32F32::inject(3)), 6.0);
/// assert_eq!(handle_i32_f32(I32F32::inject(3.0)), 6.0);
/// # }
pub trait CoprodUninjector<T, Idx>: CoprodInjector<T, Idx> {
    type Remainder;

    /// Attempts to get a value from the union.
    fn uninject(self) -> Result<T, Self::Remainder>;
}

impl<Hd, Tl> CoprodUninjector<Hd, Here> for Coproduct<Hd, Tl> {
    type Remainder = Tl;

    fn uninject(self) -> Result<Hd, Tl> {
        match self {
            Coproduct::Inl(h) => Ok(h),
            Coproduct::Inr(t) => Err(t),
        }
    }
}

impl<Hd, Tl, T, N> CoprodUninjector<T, There<N>> for Coproduct<Hd, Tl>
where
    Tl: CoprodUninjector<T, N>,
{
    type Remainder = Coproduct<Hd, Tl::Remainder>;

    fn uninject(self) -> Result<T, Self::Remainder> {
        match self {
            Coproduct::Inl(h) => Err(Coproduct::Inl(h)),
            Coproduct::Inr(t) => t.uninject().map_err(Coproduct::Inr),
        }
    }
}

/// Trait for extracting a subset of the possible types in a coproduct.
///
/// This is basically [`uninject`] on steroids.  It lets you remove a number
/// of types from a coproduct at once, leaving behind the remainder in an `Err`.
/// For instance, one can extract `Coprod!(C, A)` from `Coprod!(A, B, C, D)`
/// to produce `Result<Coprod!(C, A), Coprod!(B, D)>`.
///
/// Each type in the extracted subset is required to be part of the input coproduct.
///
/// [`uninject`]: ../traits.CoprodUninjector.html
///
/// # Example
///
/// Basic usage:
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// type I32BoolF32 = Coprod!(i32, bool, f32);
///
/// let co: Result<Coprod!(i32, f32), _> = I32BoolF32::inject(42_f32).subset();
/// assert!(co.is_ok());
///
/// let co: Result<Coprod!(i32, f32), _> = I32BoolF32::inject(true).subset();
/// assert!(co.is_err());
/// # }
/// ```
///
/// Like `uninject`, `subset` can be used for exhaustive matching,
/// with the advantage that it can remove more than one type at a time:
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::coproduct::*;
/// # fn main() {
/// fn handle_stringly_things(co: Coprod!(&'static str, String)) -> String {
///     co.fold(hlist![
///         |s| format!("&str {}", s),
///         |s| format!("String {}", s),
///     ])
/// }
///
/// fn handle_countly_things(co: Coprod!(u32)) -> String {
///     co.fold(hlist![
///         |n| vec!["."; n as usize].concat(),
///     ])
/// }
///
/// type Anything = Coprod!(String, u32, &'static str);
/// fn handle_anything(co: Anything) -> String {
///
///     // co is currently Coprod!(String, u32, &'static str)
///     let co = match co.subset().map(handle_stringly_things) {
///         Ok(s) => return s,
///         Err(co) => co,
///     };
///
///     // now co is Coprod!(u32)
///     let co = match co.subset().map(handle_countly_things) {
///         Ok(s) => return s,
///         Err(co) => co,
///     };
///
///     // now co is empty;
///     match co { /* unreachable */ }
/// }
///
/// assert_eq!(handle_anything(Anything::inject("hello")), "&str hello");
/// assert_eq!(handle_anything(Anything::inject(String::from("World!"))), "String World!");
/// assert_eq!(handle_anything(Anything::inject(4)), "....");
/// # }
/// ```
pub trait CoproductSubsetter<Targets, Indices>: Sized {
    type Remainder;

    fn subset(self) -> Result<Targets, Self::Remainder>;
}

impl<Choices, THead, TTail, NHead, NTail, Rem>
    CoproductSubsetter<Coproduct<THead, TTail>, HCons<NHead, NTail>> for Choices
where
    Self: CoprodUninjector<THead, NHead, Remainder=Rem>,
    Rem: CoproductSubsetter<TTail, NTail>,
{
    type Remainder = <Rem as CoproductSubsetter<TTail, NTail>>::Remainder;

    /// Attempt to extract a value from a subset of the types.
    fn subset(self) -> Result<Coproduct<THead, TTail>, Self::Remainder>
    { match self.uninject() {
        Ok(good) => Ok(Coproduct::Inl(good)),
        Err(bads) => match bads.subset() {
            Ok(goods) => Ok(Coproduct::Inr(goods)),
            Err(bads) => Err(bads),
        }
    }}
}

impl<Choices> CoproductSubsetter<CNil, HNil> for Choices {
    type Remainder = Self;

    #[inline(always)]
    fn subset(self) -> Result<CNil, Self::Remainder>
    { Err(self) }
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
        assert!(uninject_i32_co3.is_err());
        assert!(uninject_str_co3.is_err());
        assert_eq!(uninject_bool_co3, Ok(false));
    }


    #[test]
    fn test_coproduct_subset() {
        type I32StrBool = Coprod!(i32, &'static str, bool);

        // CNil can be extracted from anything.
        let res: Result<CNil, _> = I32StrBool::inject(3).subset();
        assert!(res.is_err());

        if false {
            #[allow(unreachable_code)] {
                // ...including CNil.
                #[allow(unused)]
                let cnil: CNil = panic!();
                let res: Result<CNil, _> = cnil.subset();
                let _ = res;
            }
        }

        { // Order does not matter.
            let co = I32StrBool::inject(3);
            let res: Result<Coprod!(bool, i32), _> = co.subset();
            assert_eq!(res, Ok(Coproduct::Inr(Coproduct::Inl(3))));

            let co = I32StrBool::inject("4");
            let res: Result<Coprod!(bool, i32), _> = co.subset();
            assert_eq!(res, Err(Coproduct::Inl("4")));
        }
    }
}
