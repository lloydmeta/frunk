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
/// To construct a Coproduct, you would typically declare a type using the `Coprod!` type
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

// Inherent methods
impl<Head, Tail> Coproduct<Head, Tail> {
    /// Instantiate a coproduct from an element.
    ///
    /// This is generally much nicer than nested usage of `Coproduct::{Inl, Inr}`.
    /// The method uses a trick with type inference to automatically build the correct variant
    /// according to the input type.
    ///
    /// In standard usage, the `Index` type parameter can be ignored,
    /// as it will typically be solved for using type inference.
    ///
    /// # Rules
    ///
    /// If the type does not appear in the coproduct, the conversion is forbidden.
    ///
    /// If the type appears multiple times in the coproduct, type inference will fail.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # fn main() {
    /// use frunk_core::coproduct::Coproduct;
    ///
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// // Constructing coproducts using inject:
    /// let co1_nice: I32F32 = Coproduct::inject(1i32);
    /// let co2_nice: I32F32 = Coproduct::inject(42f32);
    ///
    /// // Compare this to the "hard way":
    /// let co1_ugly: I32F32 = Coproduct::Inl(1i32);
    /// let co2_ugly: I32F32 = Coproduct::Inr(Coproduct::Inl(42f32));
    ///
    /// assert_eq!(co1_nice, co1_ugly);
    /// assert_eq!(co2_nice, co2_ugly);
    ///
    /// // Feel free to use `inject` on a type alias, or even directly on the
    /// // `Coprod!` macro. (the latter requires wrapping the type in `<>`)
    /// let _ = I32F32::inject(42f32);
    /// let _ = <Coprod!(i32, f32)>::inject(42f32);
    ///
    /// // You can also use a turbofish to specify the type of the input when
    /// // it is ambiguous (e.g. an empty `vec![]`).
    /// // The Index parameter should be left as `_`.
    /// type Vi32Vf32 = Coprod!(Vec<i32>, Vec<f32>);
    /// let _: Vi32Vf32 = Coproduct::inject::<Vec<i32>, _>(vec![]);
    /// # }
    /// ```
    #[inline(always)]
    pub fn inject<T, Index>(to_insert: T) -> Self
    where Self: CoprodInjector<T, Index>,
    {
        CoprodInjector::inject(to_insert)
    }

    /// Borrow an element from a coproduct by type.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # fn main() {
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// // You can let type inference find the desired type:
    /// let co1 = I32F32::inject(42f32);
    /// let co1_as_i32: Option<&i32> = co1.get();
    /// let co1_as_f32: Option<&f32> = co1.get();
    /// assert_eq!(co1_as_i32, None);
    /// assert_eq!(co1_as_f32, Some(&42f32));
    ///
    /// // You can also use turbofish syntax to specify the type.
    /// // The Index parameter should be left as `_`.
    /// let co2 = I32F32::inject(1i32);
    /// assert_eq!(co2.get::<i32, _>(), Some(&1));
    /// assert_eq!(co2.get::<f32, _>(), None);
    /// # }
    /// ```
    #[inline(always)]
    pub fn get<S, Index>(&self) -> Option<&S>
    where Self: CoproductSelector<S, Index>,
    {
        CoproductSelector::get(self)
    }

    /// Retrieve an element from a coproduct by type, ignoring all others.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # fn main() {
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// // You can let type inference find the desired type:
    /// let co1 = I32F32::inject(42f32);
    /// let co1_as_i32: Option<i32> = co1.take();
    /// let co1_as_f32: Option<f32> = co1.take();
    /// assert_eq!(co1_as_i32, None);
    /// assert_eq!(co1_as_f32, Some(42f32));
    ///
    /// // You can also use turbofish syntax to specify the type.
    /// // The Index parameter should be left as `_`.
    /// let co2 = I32F32::inject(1i32);
    /// assert_eq!(co2.take::<i32, _>(), Some(1));
    /// assert_eq!(co2.take::<f32, _>(), None);
    /// # }
    /// ```
    #[inline(always)]
    pub fn take<T, Index>(self) -> Option<T>
    where Self: CoproductTaker<T, Index>,
    {
        CoproductTaker::take(self)
    }

    /// Attempt to extract a value from a coproduct (or get the remaining possibilities).
    ///
    /// By chaining calls to this, one can exhaustively match all variants of a coproduct.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # fn main() {
    /// type I32F32 = Coprod!(i32, f32);
    /// type I32 = Coprod!(i32); // remainder after uninjecting f32
    /// type F32 = Coprod!(f32); // remainder after uninjecting i32
    ///
    /// let co1 = I32F32::inject(42f32);
    ///
    /// // You can let type inference find the desired type.
    /// let co1 = I32F32::inject(42f32);
    /// let co1_as_i32: Result<i32, F32> = co1.uninject();
    /// let co1_as_f32: Result<f32, I32> = co1.uninject();
    /// assert_eq!(co1_as_i32, Err(F32::inject(42f32)));
    /// assert_eq!(co1_as_f32, Ok(42f32));
    ///
    /// // It is not necessary to annotate the type of the remainder:
    /// let res: Result<i32, _> = co1.uninject();
    /// assert!(res.is_err());
    ///
    /// // You can also use turbofish syntax to specify the type.
    /// // The Index parameter should be left as `_`.
    /// let co2 = I32F32::inject(1i32);
    /// assert_eq!(co2.uninject::<i32, _>(), Ok(1));
    /// assert_eq!(co2.uninject::<f32, _>(), Err(I32::inject(1)));
    /// # }
    /// ```
    ///
    /// Chaining calls for an exhaustive match:
    ///
    /// ```rust
    /// # #[macro_use] extern crate frunk_core;
    /// # fn main() {
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// // Be aware that this particular example could be
    /// // written far more succinctly using `fold`.
    /// fn handle_i32_f32(co: I32F32) -> f32 {
    ///     // Remove i32 from the coproduct
    ///     let co = match co.uninject::<i32, _>() {
    ///         Ok(x) => return (2 * x) as f32,
    ///         Err(co) => co,
    ///     };
    ///
    ///     // Remove f32 from the coproduct
    ///     let co = match co.uninject::<f32, _>() {
    ///         Ok(x) => return 2.0 * x,
    ///         Err(co) => co,
    ///     };
    ///
    ///     // Now co is empty
    ///     match co { /* unreachable */ }
    /// }
    ///
    /// assert_eq!(handle_i32_f32(I32F32::inject(3)), 6.0);
    /// assert_eq!(handle_i32_f32(I32F32::inject(3.0)), 6.0);
    /// # }
    #[inline(always)]
    pub fn uninject<T, Index>(self)
    -> Result<T, <Self as CoprodUninjector<T, Index>>::Remainder>
    where Self: CoprodUninjector<T, Index>,
    {
        CoprodUninjector::uninject(self)
    }

    /// Extract a subset of the possible types in a coproduct (or get the remaining possibilities)
    ///
    /// This is basically [`uninject`] on steroids.  It lets you remove a number
    /// of types from a coproduct at once, leaving behind the remainder in an `Err`.
    /// For instance, one can extract `Coprod!(C, A)` from `Coprod!(A, B, C, D)`
    /// to produce `Result<Coprod!(C, A), Coprod!(B, D)>`.
    ///
    /// Each type in the extracted subset is required to be part of the input coproduct.
    ///
    /// [`uninject`]: #method.uninject
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # use ::frunk_core::coproduct::Coproduct;
    /// # fn main() {
    /// type I32BoolF32 = Coprod!(i32, bool, f32);
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// let co1 = I32BoolF32::inject(42_f32);
    /// let co2 = I32BoolF32::inject(true);
    ///
    /// let sub1: Result<Coprod!(i32, f32), _> = co1.subset();
    /// let sub2: Result<Coprod!(i32, f32), _> = co2.subset();
    /// assert!(sub1.is_ok());
    /// assert!(sub2.is_err());
    ///
    /// // Turbofish syntax for specifying the target subset is also supported.
    /// // The Indices parameter should be left to type inference using `_`.
    /// assert!(co1.subset::<Coprod!(i32, f32), _>().is_ok());
    /// assert!(co2.subset::<Coprod!(i32, f32), _>().is_err());
    ///
    /// // Order doesn't matter.
    /// assert!(co1.subset::<Coprod!(f32, i32), _>().is_ok());
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
    /// fn handle_all(co: Coprod!(String, u32, &'static str)) -> String {
    ///     // co is currently Coprod!(String, u32, &'static str)
    ///     let co = match co.subset().map(handle_stringly_things) {
    ///         Ok(s) => return s,
    ///         Err(co) => co,
    ///     };
    ///
    ///     // Now co is Coprod!(u32).
    ///     let co = match co.subset().map(handle_countly_things) {
    ///         Ok(s) => return s,
    ///         Err(co) => co,
    ///     };
    ///
    ///     // Now co is empty.
    ///     match co { /* unreachable */ }
    /// }
    ///
    /// assert_eq!(handle_all(Coproduct::inject("hello")), "&str hello");
    /// assert_eq!(handle_all(Coproduct::inject(String::from("World!"))), "String World!");
    /// assert_eq!(handle_all(Coproduct::inject(4)), "....");
    /// # }
    /// ```
    #[inline(always)]
    pub fn subset<Targets, Indices>(self)
    -> Result<Targets, <Self as CoproductSubsetter<Targets, Indices>>::Remainder>
    where Self: CoproductSubsetter<Targets, Indices>,
    {
        CoproductSubsetter::subset(self)
    }

    /// Convert a coproduct into another that can hold its variants.
    ///
    /// This converts a coproduct into another one which is capable of holding each
    /// of its types. The most well-supported use-cases (i.e. those where type inference
    /// is capable of solving for the indices) are:
    ///
    /// * Reordering variants: `Coprod!(C, A, B) -> Coprod!(A, B, C)`
    /// * Embedding into a superset: `Coprod!(B, D) -> Coprod!(A, B, C, D, E)`
    /// * Coalescing duplicate inputs: `Coprod!(B, B, B, B) -> Coprod!(A, B, C)`
    ///
    /// and of course any combination thereof.
    ///
    /// # Rules
    ///
    /// If any type in the input does not appear in the output, the conversion is forbidden.
    ///
    /// If any type in the input appears multiple times in the output, type inference will fail.
    ///
    /// All of these rules fall naturally out of its fairly simple definition,
    /// which is equivalent to:
    ///
    /// ```text
    /// coprod.fold(hlist![
    ///     |x| Coproduct::inject(x),
    ///     |x| Coproduct::inject(x),
    ///             ...
    ///     |x| Coproduct::inject(x),
    /// ])
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # fn main() {
    /// type I32BoolF32 = Coprod!(i32, bool, f32);
    /// type BoolI32 = Coprod!(bool, i32);
    ///
    /// let co = BoolI32::inject(true);
    /// let embedded: I32BoolF32 = co.embed();
    /// assert_eq!(embedded, I32BoolF32::inject(true));
    ///
    /// // Turbofish syntax for specifying the output type is also supported.
    /// // The Indices parameter should be left to type inference using `_`.
    /// let embedded = co.embed::<I32BoolF32, _>();
    /// assert_eq!(embedded, I32BoolF32::inject(true));
    /// # }
    /// ```
    #[inline(always)]
    pub fn embed<Targets, Indices>(self) -> Targets
    where Self: CoproductEmbedder<Targets, Indices>,
    {
        CoproductEmbedder::embed(self)
    }
}

/// Trait for instantiating a coproduct from an element
///
/// This trait is part of the implementation of the inherent static method
/// [`Coproduct::inject`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. In most code, `Coproduct::inject` will
/// "just work," with or without this trait.
///
/// [`Coproduct::inject`]: enum.Coproduct.html#method.inject
pub trait CoprodInjector<InjectType, Index> {
    /// Instantiate a coproduct from an element.
    ///
    /// Please see the [inherent static method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent static method]: enum.Coproduct.html#method.inject
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

/// Trait for borrowing a coproduct element by type
///
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::get`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. If you have a Coproduct of known type,
/// then `co.get()` should "just work" even without the trait.
///
/// [`Coproduct::get`]: enum.Coproduct.html#method.get
pub trait CoproductSelector<S, I> {
    /// Borrow an element from a coproduct by type.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.get
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

/// Trait for retrieving a coproduct element by type
///
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::take`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. If you have a Coproduct of known type,
/// then `co.take()` should "just work" even without the trait.
///
/// [`Coproduct::take`]: enum.Coproduct.html#method.take
pub trait CoproductTaker<S, I> {
    /// Retrieve an element from a coproduct by type, ignoring all others.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.take
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
impl<F, R> CoproductFoldable<F, R> for CNil {
    fn fold(self, _: F) -> R {
        unreachable!()
    }
}

/// This is literally impossible; &CNil is not instantiable
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
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::uninject`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. If you have a Coproduct of known type,
/// then `co.uninject()` should "just work" even without the trait.
///
/// [`Coproduct::uninject`]: enum.Coproduct.html#method.uninject
pub trait CoprodUninjector<T, Idx>: CoprodInjector<T, Idx> {
    type Remainder;

    /// Attempt to extract a value from a coproduct (or get the remaining possibilities).
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.uninject
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
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::subset`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. If you have a Coproduct of known type,
/// then `co.subset()` should "just work" even without the trait.
///
/// [`Coproduct::subset`]: enum.Coproduct.html#method.subset
pub trait CoproductSubsetter<Targets, Indices>: Sized {
    type Remainder;

    /// Extract a subset of the possible types in a coproduct (or get the remaining possibilities)
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.subset
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

/// Trait for converting a coproduct into another that can hold its variants.
///
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::embed`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. If you have a Coproduct of known type,
/// then `co.embed()` should "just work" even without the trait.
///
/// [`Coproduct::embed`]: enum.Coproduct.html#method.embed
pub trait CoproductEmbedder<Out, Indices> {
    /// Convert a coproduct into another that can hold its variants.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.embed
    fn embed(self) -> Out;
}

impl CoproductEmbedder<CNil, HNil> for CNil {
    fn embed(self) -> CNil
    { match self {
        // impossible!
    }}
}

impl<Head, Tail> CoproductEmbedder<Coproduct<Head, Tail>, HNil> for CNil
where CNil: CoproductEmbedder<Tail, HNil>,
{
    fn embed(self) -> Coproduct<Head, Tail>
    { match self {
        // impossible!
    }}
}

impl<Head, Tail, Out, NHead, NTail>
    CoproductEmbedder<Out, HCons<NHead, NTail>>
    for Coproduct<Head, Tail>
where
    Out: CoprodInjector<Head, NHead>,
    Tail: CoproductEmbedder<Out, NTail>,
{
    fn embed(self) -> Out
    { match self {
        Coproduct::Inl(this) => Out::inject(this),
        Coproduct::Inr(those) => those.embed(),
    }}
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

    #[test]
    fn test_coproduct_embed() {
        // CNil can be embedded into any coproduct.
        if false {
            #[allow(unreachable_code)] {
                #[allow(unused)]
                let cnil: CNil = panic!();
                let _: CNil = cnil.embed();

                #[allow(unused)]
                let cnil: CNil = panic!();
                let _: Coprod!(i32, bool) = cnil.embed();
            }
        }

        #[derive(Debug, PartialEq)] struct A;
        #[derive(Debug, PartialEq)] struct B;
        #[derive(Debug, PartialEq)] struct C;

        { // Order does not matter.
            let co_a = <Coprod!(C, A, B)>::inject(A);
            let co_b = <Coprod!(C, A, B)>::inject(B);
            let co_c = <Coprod!(C, A, B)>::inject(C);
            let out_a: Coprod!(A, B, C) = co_a.embed();
            let out_b: Coprod!(A, B, C) = co_b.embed();
            let out_c: Coprod!(A, B, C) = co_c.embed();
            assert_eq!(out_a, Coproduct::Inl(A));
            assert_eq!(out_b, Coproduct::Inr(Coproduct::Inl(B)));
            assert_eq!(out_c, Coproduct::Inr(Coproduct::Inr(Coproduct::Inl(C))));
        }

        { // Multiple variants can resolve to the same output w/o type annotations
            type ABC = Coprod!(A, B, C);
            type BBB = Coprod!(B, B, B);

            let b1 = BBB::inject::<_, Here>(B);
            let b2 = BBB::inject::<_, There<Here>>(B);
            let out1: ABC = b1.embed();
            let out2: ABC = b2.embed();
            assert_eq!(out1, Coproduct::Inr(Coproduct::Inl(B)));
            assert_eq!(out2, Coproduct::Inr(Coproduct::Inl(B)));
        }
    }
}
