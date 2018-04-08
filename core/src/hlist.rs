//! Module that holds HList data structures, implementations, and typeclasses.
//!
//! Typically, you would want to use the `hlist!` macro to make it easier
//! for you to use HList.
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
//! let h = hlist![1, "hi"];
//! assert_eq!(h.len(), 2);
//! let (a, b) = h.into_tuple2();
//! assert_eq!(a, 1);
//! assert_eq!(b, "hi");
//!
//! // Reverse
//! let h1 = hlist![true, "hi"];
//! assert_eq!(h1.into_reverse(), hlist!["hi", true]);
//!
//! // foldr (foldl also available)
//! let h2 = hlist![1, false, 42f32];
//! let folded = h2.foldr(
//!             hlist![|i, acc| i + acc,
//!                    |_, acc| if acc > 42f32 { 9000 } else { 0 },
//!                    |f, acc| f + acc],
//!             1f32
//!     );
//! assert_eq!(folded, 9001);
//!
//! let h3 = hlist![9000, "joe", 41f32];
//! // Mapping over an HList with a polymorphic function,
//! // declared using the poly_fn! macro (you can choose to impl
//! // it manually)
//! let mapped = h3.map(
//!   poly_fn![
//!     |f: f32|   -> f32 { f + 1f32 },
//!     |i: isize| -> isize { i + 1 },
//!     ['a] |s: &'a str| -> &'a str { s }
//!   ]);
//! assert_eq!(mapped, hlist![9001, "joe", 42f32]);
//!
//! // Plucking a value out by type
//! let h4 = hlist![1, "hello", true, 42f32];
//! let (t, remainder): (bool, _) = h4.pluck();
//! assert!(t);
//! assert_eq!(remainder, hlist![1, "hello", 42f32]);
//!
//! // Resculpting an HList
//! let h5 = hlist![9000, "joe", 41f32, true];
//! let (reshaped, remainder2): (Hlist![f32, i32, &str], _) = h5.sculpt();
//! assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
//! assert_eq!(remainder2, hlist![true]);
//! # }
//! ```

use std::ops::Add;
use std::marker::PhantomData;

/// Typeclass for HList-y behaviour
///
/// An HList is a heterogeneous list, one that is statically typed at compile time. In simple terms,
/// it is just an arbitrarily-nested Tuple2.
pub trait HList: Sized {
    /// Returns the length of a given HList type without making use of any references, or
    /// in fact, any values at all.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::HList; fn main() {
    /// assert_eq!(<Hlist![i32, bool, f32] as HList>::LEN, 3);
    /// # }
    /// ```
    const LEN: usize;

    #[deprecated(since = "0.1.30", note = "Please use len() or static_len() instead.")]
    fn length(&self) -> u32 {
        Self::LEN as u32
    }

    /// Returns the length of a given HList
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; fn main() {
    /// let h = hlist![1, "hi"];
    /// assert_eq!(h.len(), 2);
    /// # }
    /// ```
    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns the length of a given HList type without making use of any references, or
    /// in fact, any values at all.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::HList; fn main() {
    /// assert_eq!(<Hlist![i32, bool, f32] as HList>::static_len(), 3);
    /// # }
    /// ```
    #[inline]
    #[deprecated(since = "0.1.31", note = "Please use LEN instead")]
    fn static_len() -> usize;

    /// Prepends an item to the current HList
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; fn main() {
    /// let h1 = hlist![1, "hi"];
    /// let h2 = h1.prepend(true);
    /// let (a, (b, c)) = h2.into_tuple2();
    /// assert_eq!(a, true);
    /// assert_eq!(b, 1);
    /// assert_eq!(c, "hi");
    /// # }
    fn prepend<H>(self, h: H) -> HCons<H, Self> {
        HCons {
            head: h,
            tail: self,
        }
    }
}

/// Represents the right-most end of a heterogeneous list
///
/// # Examples
///
/// ```
/// # use frunk_core::hlist::{h_cons, HNil};
/// let h = h_cons(1, HNil);
/// let h = h.head;
/// assert_eq!(h, 1);
/// ```
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HNil;

impl HList for HNil {
    const LEN: usize = 0;
    fn static_len() -> usize {
        Self::LEN
    }
}

/// Represents the most basic non-empty HList. Its value is held in `head`
/// while its tail is another HList.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}

impl<H, T: HList> HList for HCons<H, T> {
    const LEN: usize = 1 + <T as HList>::LEN;
    fn static_len() -> usize {
        Self::LEN
    }
}

impl<H, T> HCons<H, T> {
    /// Returns the head of the list and the tail of the list as a tuple2.
    /// The original list is consumed
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::HNil; fn main() {
    /// let h = hlist!("hi");
    /// let (h, tail) = h.pop();
    /// assert_eq!(h, "hi");
    /// assert_eq!(tail, HNil);
    /// # }
    /// ```
    pub fn pop(self) -> (H, T) {
        (self.head, self.tail)
    }
}

/// Takes an element and an Hlist and returns another one with
/// the element prepended to the original list. The original list
/// is consumed
///
/// # Examples
///
/// ```
/// # use frunk_core::hlist::*;
/// let h_list = h_cons("what", h_cons(1.23f32, HNil));
/// let (h1, h2) = h_list.into_tuple2();
/// assert_eq!(h1, "what");
/// assert_eq!(h2, 1.23f32);
/// ```
pub fn h_cons<H, T: HList>(h: H, tail: T) -> HCons<H, T> {
    HCons {
        head: h,
        tail: tail,
    }
}

// Inherent methods shared by HNil and HCons.
macro_rules! gen_inherent_methods {
    (impl<$($TyPar:ident),*> $Struct:ty { ... })
    => {
        impl<$($TyPar),*> $Struct {
            /// Returns the length of a given HList
            ///
            /// # Examples
            ///
            /// ```
            /// # #[macro_use] extern crate frunk_core; fn main() {
            /// let h = hlist![1, "hi"];
            /// assert_eq!(h.len(), 2);
            /// # }
            /// ```
            #[inline(always)]
            pub fn len(&self) -> usize
            where Self: HList,
            {
                HList::len(self)
            }

            /// Prepend an item to the current HList
            ///
            /// # Examples
            ///
            /// ```
            /// # #[macro_use] extern crate frunk_core; fn main() {
            /// let h1 = hlist![1, "hi"];
            /// let h2 = h1.prepend(true);
            /// let (a, (b, c)) = h2.into_tuple2();
            /// assert_eq!(a, true);
            /// assert_eq!(b, 1);
            /// assert_eq!(c, "hi");
            /// # }
            #[inline(always)]
            pub fn prepend<H>(self, h: H) -> HCons<H, Self>
            where Self: HList,
            {
                HList::prepend(self, h)
            }

            /// Consume the current HList and return an HList with the requested shape.
            ///
            /// `sculpt` allows us to extract/reshape/scult the current HList into another shape,
            /// provided that the requested shape's types are are contained within the current HList.
            ///
            /// The `Indices` type parameter allows the compiler to figure out that `Ts`
            /// and `Self` can be morphed into each other.
            ///
            /// # Examples
            ///
            /// ```
            /// # #[macro_use] extern crate frunk_core; fn main() {
            /// let h = hlist![9000, "joe", 41f32, true];
            /// let (reshaped, remainder): (Hlist![f32, i32, &str], _) = h.sculpt();
            /// assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
            /// assert_eq!(remainder, hlist![true]);
            /// # }
            /// ```
            #[inline(always)]
            pub fn sculpt<Ts, Indices>(self) -> (Ts, <Self as Sculptor<Ts, Indices>>::Remainder)
            where Self: Sculptor<Ts, Indices>,
            {
                Sculptor::sculpt(self)
            }

            /// Reverse the HList.
            ///
            /// # Examples
            ///
            /// ```
            /// # #[macro_use] extern crate frunk_core; fn main() {
            /// assert_eq!(hlist![].into_reverse(), hlist![]);
            ///
            /// assert_eq!(
            ///     hlist![1, "hello", true, 42f32].into_reverse(),
            ///     hlist![42f32, true, "hello", 1],
            /// )
            /// # }
            /// ```
            #[inline(always)]
            pub fn into_reverse(self) -> <Self as IntoReverse>::Output
            where Self: IntoReverse,
            {
                IntoReverse::into_reverse(self)
            }

            /// Return an HList where the contents are references to
            /// the original HList on which this method was called.
            ///
            /// # Examples
            ///
            /// ```
            /// # #[macro_use] extern crate frunk_core; fn main() {
            /// assert_eq!(hlist![].to_ref(), hlist![]);
            ///
            /// assert_eq!(hlist![1, true].to_ref(), hlist![&1, &true]);
            /// # }
            /// ```
            #[inline(always)]
            pub fn to_ref<'a>(&'a self) -> <Self as ToRef<'a>>::Output
                where Self: ToRef<'a>,
            {
                ToRef::to_ref(self)
            }

            /// (XXX from trait XXX)
            ///
            /// Trait that allow for mapping over a data structure using mapping functions stored in another
            /// data structure
            ///
            /// It might be a good idea to try to re-write these using the foldr variants, but it's a
            /// wee-bit more complicated.
            ///
            /// (XXX from method XXX)
            ///
            /// Maps over the current data structure using functions stored in another
            /// data structure.
            ///
            /// # Examples
            ///
            /// ```
            /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
            /// let nil = HNil;
            ///
            /// assert_eq!(nil.map(HNil), HNil);
            ///
            /// let h = hlist![1, false, 42f32];
            ///
            /// // Sadly we need to help the compiler understand the bool type in our mapper
            ///
            /// let mapped = h.to_ref().map(hlist![
            ///     |&n| n + 1,
            ///     |b: &bool| !b,
            ///     |&f| f + 1f32]);
            /// assert_eq!(mapped, hlist![2, true, 43f32]);
            ///
            /// // There is also a value-consuming version that passes values to your functions
            /// // instead of just references:
            ///
            /// let mapped2 = h.map(hlist![
            ///     |n| n + 3,
            ///     |b: bool| !b,
            ///     |f| f + 8959f32]);
            /// assert_eq!(mapped2, hlist![4, true, 9001f32]);
            /// # }
            /// ```
            #[inline(always)]
            pub fn map<F>(self,mapper: F) -> <Self as HMappable<F>>::Output
            where Self: HMappable<F>,
            {
                HMappable::map(self, mapper)
            }

            /// (XXX from trait XXX)
            /// Left fold for a given data structure
            ///
            /// (XXX from method XXX)
            /// foldl over a data structure
            ///
            /// # Examples
            ///
            /// ```
            /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
            /// let nil = HNil;
            ///
            /// assert_eq!(nil.foldl(HNil, 0), 0);
            ///
            /// let h = hlist![1, false, 42f32];
            ///
            /// let folded = h.to_ref().foldl(
            ///     hlist![
            ///         |acc, &i| i + acc,
            ///         |acc, b: &bool| if !b && acc > 42 { 9000f32 } else { 0f32 },
            ///         |acc, &f| f + acc
            ///     ],
            ///     1
            /// );
            ///
            /// assert_eq!(42f32, folded);
            ///
            /// // There is also a value-consuming version that passes values to your folding
            /// // functions instead of just references:
            ///
            /// let folded2 = h.foldl(
            ///     hlist![
            ///         |acc, i| i + acc,
            ///         |acc, b: bool| if !b && acc > 42 { 9000f32 } else { 0f32 },
            ///         |acc, f| f + acc
            ///     ],
            ///     8918
            /// );
            ///
            /// assert_eq!(9042f32, folded2)
            /// # }
            /// ```
            #[inline(always)]
            pub fn foldl<Folder, Acc>(
                self,
                folder: Folder,
                acc: Acc,
            ) -> <Self as HFoldLeftable<Folder, Acc>>::Output
            where Self: HFoldLeftable<Folder, Acc>,
            {
                HFoldLeftable::foldl(self, folder, acc)
            }

            /// (XXX from trait XXX)
            /// Foldr for HLists
            ///
            /// (XXX from method XXX)
            /// foldr over a data structure
            ///
            /// Sadly, due to a compiler quirk, only the value-consuming (the original hlist) variant
            /// exists for foldr.
            ///
            /// # Examples
            ///
            /// ```
            /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
            /// let nil = HNil;
            ///
            /// assert_eq!(nil.foldr(HNil, 0), 0);
            ///
            /// let h = hlist![1, false, 42f32];
            ///
            /// let folded = h.foldr(
            ///     hlist![
            ///         |i, acc| i + acc,
            ///         |b: bool, acc| if !b && acc > 42f32 { 9000 } else { 0 },
            ///         |f, acc| f + acc
            ///     ],
            ///     1f32
            /// );
            ///
            /// assert_eq!(9001, folded)
            ///
            /// # }
            /// ```
            #[inline(always)]
            pub fn foldr<Folder, Init>(
                self,
                folder: Folder,
                init: Init,
            ) -> <Self as HFoldRightable<Folder, Init>>::Output
            where Self: HFoldRightable<Folder, Init>,
            {
                HFoldRightable::foldr(self, folder, init)
            }
        }
    };
}

gen_inherent_methods!{
    impl<> HNil { ... }
}
gen_inherent_methods!{
    impl<Head, Tail> HCons<Head, Tail> { ... }
}

// HCons-only inherent methods.
impl<Head, Tail> HCons<Head, Tail> {
    /// Borrow an element by type from an HList.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; fn main() {
    /// let h = hlist![1i32, 2u32, "hello", true, 42f32];
    ///
    /// // Often, type inference can figure out the type you want.
    /// // You can help guide type inference when necessary by
    /// // using type annotations.
    /// let b: &bool = h.get();
    /// if !b { panic!("no way!") };
    ///
    /// // If space is tight, you can also use turbofish syntax.
    /// // The Index is still left to type inference by using `_`.
    /// match *h.get::<u32, _>() {
    ///     2 => { },
    ///     _ => panic!("it can't be!!"),
    /// }
    /// # }
    /// ```
    #[inline(always)]
    pub fn get<T, Index>(&self) -> &T
    where
        Self: Selector<T, Index>,
    {
        Selector::get(self)
    }

    /// Remove an element by type from an HList.
    ///
    /// The remaining elements are returned along with it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; fn main() {
    /// let list = hlist![1, "hello", true, 42f32];
    ///
    /// // Often, type inference can figure out the target type.
    /// let (b, list): (bool, _) = list.pluck();
    /// assert!(b);
    ///
    /// // When type inference will not suffice, you can use a turbofish.
    /// // The Index is still left to type inference by using `_`.
    /// let (s, list) = list.pluck::<i32, _>();
    ///
    /// // Each time we plucked, we got back a remainder.
    /// // Let's check what's left:
    /// assert_eq!(list, hlist!["hello", 42.0])
    /// # }
    /// ```
    #[inline(always)]
    pub fn pluck<T, Index>(self) -> (T, <Self as Plucker<T, Index>>::Remainder)
    where
        Self: Plucker<T, Index>,
    {
        Plucker::pluck(self)
    }

    /// Turns an HList into nested Tuple2s, which are less troublesome to pattern match
    /// and have a nicer type signature.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; fn main() {
    /// let h = hlist![1, "hello", true, 42f32];
    ///
    /// // We now have a much nicer pattern matching experience
    /// let (first,(second,(third, fourth))) = h.into_tuple2();
    ///
    /// assert_eq!(first ,       1);
    /// assert_eq!(second, "hello");
    /// assert_eq!(third ,    true);
    /// assert_eq!(fourth,   42f32);
    /// # }
    /// ```
    #[inline(always)]
    pub fn into_tuple2(
        self,
    ) -> (
        <Self as IntoTuple2>::HeadType,
        <Self as IntoTuple2>::TailOutput,
    )
    where
        Self: IntoTuple2,
    {
        IntoTuple2::into_tuple2(self)
    }
}

impl<RHS> Add<RHS> for HNil
where
    RHS: HList,
{
    type Output = RHS;

    fn add(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, T, RHS> Add<RHS> for HCons<H, T>
where
    T: Add<RHS>,
    RHS: HList,
{
    type Output = HCons<H, <T as Add<RHS>>::Output>;

    fn add(self, rhs: RHS) -> Self::Output {
        HCons {
            head: self.head,
            tail: self.tail + rhs,
        }
    }
}

/// Largely lifted from https://github.com/Sgeo/hlist/blob/master/src/lib.rs#L30

/// Used as an index into an `HList`.
///
/// `Here` is 0, pointing to the head of the HList.
///
/// Users should normally allow type inference to create this type
#[allow(dead_code)]
pub enum Here {}

/// Used as an index into an `HList`.
///
/// `There<T>` is 1 + `T`.
///
/// Users should normally allow type inference to create this type.
#[allow(dead_code)]
pub struct There<T>(PhantomData<T>);

/// Trait for borrowing an HList element by type
///
/// This trait is part of the implementation of the inherent method
/// [`HCons::get`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// HLists of unknown type. If you have an HList of known type,
/// then `list.get()` should "just work" even without the trait.
///
/// [`HCons::get`]: struct.HCons.html#method.get
pub trait Selector<S, I> {
    /// Borrow an element by type from an HList.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: struct.HCons.html#method.get
    fn get(&self) -> &S;
}

impl<T, Tail> Selector<T, Here> for HCons<T, Tail> {
    fn get(&self) -> &T {
        &self.head
    }
}

impl<Head, Tail, FromTail, TailIndex> Selector<FromTail, There<TailIndex>> for HCons<Head, Tail>
where
    Tail: Selector<FromTail, TailIndex>,
{
    fn get(&self) -> &FromTail {
        self.tail.get()
    }
}

/// Trait defining extraction from a given HList
///
/// This trait is part of the implementation of the inherent method
/// [`HCons::pluck`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// HLists of unknown type. If you have an HList of known type,
/// then `list.pluck()` should "just work" even without the trait.
///
/// [`HCons::pluck`]: struct.HCons.html#method.pluck
pub trait Plucker<Target, Index> {
    /// What is left after you pluck the target from the Self
    type Remainder;

    /// Remove an element by type from an HList.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: struct.HCons.html#method.pluck
    fn pluck(self) -> (Target, Self::Remainder);
}

/// Implementation when the pluck target is in head
impl<T, Tail> Plucker<T, Here> for HCons<T, Tail> {
    type Remainder = Tail;

    fn pluck(self) -> (T, Self::Remainder) {
        (self.head, self.tail)
    }
}

/// Implementation when the pluck target is in the tail
impl<Head, Tail, FromTail, TailIndex> Plucker<FromTail, There<TailIndex>> for HCons<Head, Tail>
where
    Tail: Plucker<FromTail, TailIndex>,
{
    type Remainder = HCons<Head, <Tail as Plucker<FromTail, TailIndex>>::Remainder>;

    fn pluck(self) -> (FromTail, Self::Remainder) {
        let (target, tail_remainder): (
            FromTail,
            <Tail as Plucker<FromTail, TailIndex>>::Remainder,
        ) = <Tail as Plucker<FromTail, TailIndex>>::pluck(self.tail);
        (
            target,
            HCons {
                head: self.head,
                tail: tail_remainder,
            },
        )
    }
}

/// Trait for pulling out some subset of an HList, using type inference.
///
/// This trait is part of the implementation of the inherent method
/// [`HCons::sculpt`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// HLists of unknown type. If you have an HList of known type,
/// then `list.sculpt()` should "just work" even without the trait.
///
/// [`HCons::sculpt`]: struct.HCons.html#method.sculpt
pub trait Sculptor<Target, Indices> {
    type Remainder;

    /// Consumes the current HList and returns an HList with the requested shape.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: struct.HCons.html#method.sculpt
    fn sculpt(self) -> (Target, Self::Remainder);
}

/// Implementation for when the target is an empty HList (HNil)
///
/// Index type is HNil because we don't need an index for finding HNil
impl<Source> Sculptor<HNil, HNil> for Source {
    type Remainder = Source;

    #[inline(always)]
    fn sculpt(self) -> (HNil, Self::Remainder) {
        (HNil, self)
    }
}

/// Implementation for when we have a non-empty HCons target
///
/// Indices is HCons<IndexHead, IndexTail> here because the compiler is being asked to figure out the
/// Index for Plucking the first item of type THead out of Self and the rest (IndexTail) is for the
/// Plucker's remainder induce.
impl<
    THead,
    TTail,
    SHead,
    STail,
    IndexHead,
    IndexTail,
> Sculptor<HCons<THead, TTail>, HCons<IndexHead, IndexTail>> for HCons<SHead, STail>
where
    HCons<SHead, STail>: Plucker<THead, IndexHead>,
    <HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder: Sculptor<TTail, IndexTail>,
{
    type Remainder =
        <<HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder as Sculptor<
            TTail,
            IndexTail,
        >>::Remainder;

    #[inline(always)]
    fn sculpt(self) -> (HCons<THead, TTail>, Self::Remainder) {
        let (p, r): (
            THead,
            <HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder,
        ) = self.pluck();
        let (tail, tail_remainder): (TTail, Self::Remainder) = r.sculpt();
        (
            HCons {
                head: p,
                tail: tail,
            },
            tail_remainder,
        )
    }
}

/// Trait that allows for reversing a given data structure.
///
/// Implemented for HLists.
///
/// This functionality is also provided as an [inherent method].
/// However, you may find this trait useful in generic contexts.
///
/// [inherent method]: struct.HCons.html#method.into_reverse
pub trait IntoReverse {
    type Output;

    /// Reverses a given data structure.
    fn into_reverse(self) -> Self::Output;
}

impl IntoReverse for HNil {
    type Output = HNil;
    fn into_reverse(self) -> Self::Output {
        self
    }
}

impl<H, Tail> IntoReverse for HCons<H, Tail>
where
    Tail: IntoReverse,
    <Tail as IntoReverse>::Output: Add<HCons<H, HNil>>,
{
    type Output = <<Tail as IntoReverse>::Output as Add<HCons<H, HNil>>>::Output;

    fn into_reverse(self) -> Self::Output {
        self.tail.into_reverse() + HCons {
            head: self.head,
            tail: HNil,
        }
    }
}

/// This is a thin generic wrapper type that is used to differentiate
/// between single-typed generic closure F that implements, say, Fn<i8> -> bool,
/// and a Poly-typed F that implements multiple Function types, say
/// Func<i8, bool>, Fun<bool, f32> etc.
///
/// This is needed because there are completely generic impls for many of the
/// HList traits that take a simple unwrapped closure.
pub struct Poly<T>(pub T);

/// This is a simple, user-implementable version of Fn.
///
/// Might not be necessary if/when Fn(Once, Mut) traits are implementable
/// in stable Rust
pub trait Func<Input> {
    type Output;

    fn call(i: Input) -> Self::Output;
}

impl<P, H, Tail> HMappable<Poly<P>> for HCons<H, Tail>
where
    P: Func<H>,
    Tail: HMappable<Poly<P>>,
{
    type Output = HCons<<P as Func<H>>::Output, <Tail as HMappable<Poly<P>>>::Output>;
    fn map(self, poly: Poly<P>) -> Self::Output {
        HCons {
            head: P::call(self.head),
            tail: self.tail.map(poly),
        }
    }
}

// TODO
pub trait HMappable<Mapper> {
    type Output;

    // TODO
    fn map(self, folder: Mapper) -> Self::Output;
}

impl<F> HMappable<F> for HNil {
    type Output = HNil;

    fn map(self, _: F) -> Self::Output {
        HNil
    }
}

impl<F, R, H, Tail> HMappable<F> for HCons<H, Tail>
where
    F: Fn(H) -> R,
    Tail: HMappable<F>,
{
    type Output = HCons<R, <Tail as HMappable<F>>::Output>;

    fn map(self, f: F) -> Self::Output {
        let HCons { head, tail } = self;
        HCons {
            head: f(head),
            tail: tail.map(f),
        }
    }
}

impl<F, R, MapperTail, H, Tail> HMappable<HCons<F, MapperTail>> for HCons<H, Tail>
where
    F: FnOnce(H) -> R,
    Tail: HMappable<MapperTail>,
{
    type Output = HCons<R, <Tail as HMappable<MapperTail>>::Output>;

    fn map(self, mapper: HCons<F, MapperTail>) -> Self::Output {
        let HCons { head, tail } = self;
        HCons {
            head: (mapper.head)(head),
            tail: tail.map(mapper.tail),
        }
    }
}

// TODO
pub trait HFoldRightable<Folder, Init> {
    type Output;

    // TODO
    fn foldr(self, folder: Folder, i: Init) -> Self::Output;
}

impl<F, Init> HFoldRightable<F, Init> for HNil {
    type Output = Init;

    fn foldr(self, _: F, i: Init) -> Self::Output {
        i
    }
}

impl<F, FolderHeadR, FolderTail, H, Tail, Init> HFoldRightable<HCons<F, FolderTail>, Init>
    for HCons<H, Tail>
where
    Tail: HFoldRightable<FolderTail, Init>,
    F: FnOnce(H, <Tail as HFoldRightable<FolderTail, Init>>::Output) -> FolderHeadR,
{
    type Output = FolderHeadR;

    fn foldr(self, folder: HCons<F, FolderTail>, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder.tail, init);
        (folder.head)(self.head, folded_tail)
    }
}

impl<'a, F, R, H, Tail, Init> HFoldRightable<&'a F, Init> for HCons<H, Tail>
where
    Tail: HFoldRightable<&'a F, Init>,
    F: Fn(H, <Tail as HFoldRightable<&'a F, Init>>::Output) -> R,
{
    type Output = R;

    fn foldr(self, folder: &'a F, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder, init);
        (folder)(self.head, folded_tail)
    }
}

/// An alternative to AsRef that does not force the reference type to be a pointer itself.
///
/// This lets us create implementations for our recursive traits that take the resulting
/// Output reference type, without having to deal with strange, spurious overflows
/// that sometimes occur when trying to implement a trait for &'a T (see this comment:
/// https://github.com/lloydmeta/frunk/pull/106#issuecomment-377927198)
///
/// This is essentially From, but the more specific nature of it means it's more ergonomic
/// in actual usage.
///
/// Implemented for HLists.
///
/// This functionality is also provided as an [inherent method].
/// However, you may find this trait useful in generic contexts.
///
/// [inherent method]: struct.HCons.html#method.to_ref
pub trait ToRef<'a> {
    type Output;

    #[inline(always)]
    fn to_ref(&'a self) -> Self::Output;
}

impl<'a> ToRef<'a> for HNil {
    type Output = HNil;

    #[inline(always)]
    fn to_ref(&'a self) -> Self::Output {
        HNil
    }
}

impl<'a, H, Tail> ToRef<'a> for HCons<H, Tail>
where
    H: 'a,
    Tail: ToRef<'a>,
{
    type Output = HCons<&'a H, <Tail as ToRef<'a>>::Output>;

    #[inline(always)]
    fn to_ref(&'a self) -> Self::Output {
        HCons {
            head: &self.head,
            tail: (&self.tail).to_ref(),
        }
    }
}

// TODO
pub trait HFoldLeftable<Folder, Acc> {
    type Output;

    // TODO
    fn foldl(self, folder: Folder, acc: Acc) -> Self::Output;
}

impl<F, Acc> HFoldLeftable<F, Acc> for HNil {
    type Output = Acc;

    fn foldl(self, _: F, acc: Acc) -> Self::Output {
        acc
    }
}

impl<F, R, FTail, H, Tail, Acc> HFoldLeftable<HCons<F, FTail>, Acc> for HCons<H, Tail>
where
    Tail: HFoldLeftable<FTail, R>,
    F: FnOnce(Acc, H) -> R,
{
    type Output = <Tail as HFoldLeftable<FTail, R>>::Output;

    fn foldl(self, folder: HCons<F, FTail>, acc: Acc) -> Self::Output {
        let HCons { head, tail } = self;
        tail.foldl(folder.tail, (folder.head)(acc, head))
    }
}

/// Implementation for folding over an HList using a single function that
/// can handle all cases
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
/// let h = hlist![1, 2, 3, 4, 5];
///
/// let r: isize = h.foldl(|acc, next| acc + next, 0);
/// assert_eq!(r, 15);
/// # }
/// ```
impl<F, H, Tail, Acc> HFoldLeftable<F, Acc> for HCons<H, Tail>
where
    Tail: HFoldLeftable<F, Acc>,
    F: Fn(Acc, H) -> Acc,
{
    type Output = <Tail as HFoldLeftable<F, Acc>>::Output;

    fn foldl(self, f: F, acc: Acc) -> Self::Output {
        let HCons { head, tail } = self;
        let acc = f(acc, head);
        tail.foldl(f, acc)
    }
}

/// Trait for transforming an HList into a nested tuple.
///
/// This trait is part of the implementation of the inherent method
/// [`HCons::into_tuple2`]. Please see that method for more information.
///
/// This operation is not useful in generic contexts, so it is unlikely
/// that you should ever need to import this trait. Do not worry;
/// if you have an HList of known type, then `list.into_tuple2()`
/// should "just work," even without the trait.
///
/// [`HCons::into_tuple2`]: struct.HCons.html#method.into_tuple2
pub trait IntoTuple2 {
    /// The 0 element in the output tuple
    type HeadType;

    /// The 1 element in the output tuple
    type TailOutput;

    /// Turns an HList into nested Tuple2s, which are less troublesome to pattern match
    /// and have a nicer type signature.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// [inherent method]: struct.HCons.html#method.into_tuple2
    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput);
}

impl<T1, T2> IntoTuple2 for HCons<T1, HCons<T2, HNil>> {
    type HeadType = T1;
    type TailOutput = T2;

    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.head)
    }
}

impl<T, Tail> IntoTuple2 for HCons<T, Tail>
where
    Tail: IntoTuple2,
{
    type HeadType = T;
    type TailOutput = (
        <Tail as IntoTuple2>::HeadType,
        <Tail as IntoTuple2>::TailOutput,
    );

    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.into_tuple2())
    }
}

impl<H, Tail> Into<Vec<H>> for HCons<H, Tail>
where
    Tail: Into<Vec<H>> + HList,
{
    fn into(self) -> Vec<H> {
        let h = self.head;
        let t = self.tail;
        let mut v = Vec::with_capacity(<Self as HList>::LEN);
        v.push(h);
        let mut t_vec: Vec<H> = t.into();
        v.append(&mut t_vec);
        v
    }
}

impl<T> Into<Vec<T>> for HNil {
    fn into(self) -> Vec<T> {
        Vec::with_capacity(0)
    }
}

impl Default for HNil {
    fn default() -> Self {
        HNil
    }
}

impl<T: Default, Tail: Default + HList> Default for HCons<T, Tail> {
    fn default() -> Self {
        h_cons(T::default(), Tail::default())
    }
}

/// Indexed type conversions of `T -> Self` with index `I`.
/// This is a generalized version of `From` which for example allows the caller
/// to use default values for parts of `Self` and thus "fill in the blanks".
///
/// `LiftFrom` is the reciprocal of `LiftInto`.
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
/// type H = Hlist![(), usize, f64, (), bool];
///
/// let x = H::lift_from(42.0);
/// assert_eq!(x, hlist![(), 0, 42.0, (), false]);
///
/// let x: H = lift_from(true);
/// assert_eq!(x, hlist![(), 0, 0.0, (), true]);
/// # }
/// ```
pub trait LiftFrom<T, I> {
    /// Performs the indexed conversion.
    fn lift_from(part: T) -> Self;
}

/// Free function version of `LiftFrom::lift_from`.
pub fn lift_from<I, T, PF: LiftFrom<T, I>>(part: T) -> PF {
    PF::lift_from(part)
}

/// An indexed conversion that consumes `self`, and produces a `T`. To produce
/// `T`, the index `I` may be used to for example "fill in the blanks".
/// `LiftInto` is the reciprocal of `LiftFrom`.
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
/// type H = Hlist![(), usize, f64, (), bool];
///
/// // Type inference works as expected:
/// let x: H = 1337.lift_into();
/// assert_eq!(x, hlist![(), 1337, 0.0, (), false]);
///
/// // Sublists:
/// let x: H = hlist![(), true].lift_into();
/// assert_eq!(x, hlist![(), 0, 0.0, (), true]);
///
/// let x: H = hlist![3.0, ()].lift_into();
/// assert_eq!(x, hlist![(), 0, 3.0, (), false]);
///
/// let x: H = hlist![(), 1337].lift_into();
/// assert_eq!(x, hlist![(), 1337, 0.0, (), false]);
///
/// let x: H = hlist![(), 1337, 42.0, (), true].lift_into();
/// assert_eq!(x, hlist![(), 1337, 42.0, (), true]);
/// # }
/// ```
pub trait LiftInto<T, I> {
    /// Performs the indexed conversion.
    fn lift_into(self) -> T;
}

impl<T, U, I> LiftInto<U, I> for T
where
    U: LiftFrom<T, I>,
{
    fn lift_into(self) -> U {
        LiftFrom::lift_from(self)
    }
}

impl<T, Tail> LiftFrom<T, Here> for HCons<T, Tail>
where
    Tail: Default + HList,
{
    fn lift_from(part: T) -> Self {
        h_cons(part.into(), Tail::default())
    }
}

impl<Head, Tail, ValAtIx, TailIx> LiftFrom<ValAtIx, There<TailIx>> for HCons<Head, Tail>
where
    Head: Default,
    Tail: HList + LiftFrom<ValAtIx, TailIx>,
{
    fn lift_from(part: ValAtIx) -> Self {
        h_cons(Head::default(), Tail::lift_from(part))
    }
}

/// An index denoting that `Suffix` is just that.
pub struct Suffixed<Suffix>(PhantomData<Suffix>);

impl<Prefix, Suffix> LiftFrom<Prefix, Suffixed<Suffix>> for <Prefix as Add<Suffix>>::Output
where
    Prefix: HList + Add<Suffix>,
    Suffix: Default,
{
    fn lift_from(part: Prefix) -> Self {
        part + Suffix::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hcons() {
        let hlist1 = h_cons(1, HNil);
        let (h, _) = hlist1.pop();
        assert_eq!(h, 1);

        let hlist2 = h_cons("hello", h_cons(1, HNil));
        let (h2, tail2) = hlist2.pop();
        let (h1, _) = tail2.pop();
        assert_eq!(h2, "hello");
        assert_eq!(h1, 1);
    }

    struct HasHList<T: HList>(T);

    #[test]
    fn test_contained_list() {
        let c = HasHList(h_cons(1, HNil));
        let retrieved = c.0;
        assert_eq!(retrieved.len(), 1);
        let new_list = h_cons(2, retrieved);
        assert_eq!(new_list.len(), 2);
    }

    #[test]
    fn test_pluck() {
        let h = hlist![1, "hello", true, 42f32];
        let (t, r): (f32, _) = h.pluck();
        assert_eq!(t, 42f32);
        assert_eq!(r, hlist![1, "hello", true])
    }

    #[test]
    fn test_hlist_macro() {
        assert_eq!(hlist![], HNil);
        let h: Hlist!(i32, &str, i32) = hlist![1, "2", 3];
        let (h1, tail1) = h.pop();
        assert_eq!(h1, 1);
        assert_eq!(tail1, hlist!["2", 3]);
        let (h2, tail2) = tail1.pop();
        assert_eq!(h2, "2");
        assert_eq!(tail2, hlist![3]);
        let (h3, tail3) = tail2.pop();
        assert_eq!(h3, 3);
        assert_eq!(tail3, HNil);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Hlist_macro() {
        let h1: Hlist!(i32, &str, i32) = hlist![1, "2", 3];
        let h2: Hlist!(i32, &str, i32,) = hlist![1, "2", 3];
        let h3: Hlist!(i32) = hlist![1];
        let h4: Hlist!(i32,) = hlist![1,];
        assert_eq!(h1, h2);
        assert_eq!(h3, h4);
    }

    #[test]
    fn test_pattern_matching() {
        let hlist_pat!(one1) = hlist!["one"];
        assert_eq!(one1, "one");
        let hlist_pat!(one2,) = hlist!["one"];
        assert_eq!(one2, "one");

        let h = hlist![5, 3.2f32, true, "blue".to_owned()];
        let hlist_pat!(five, float, right, s) = h;
        assert_eq!(five, 5);
        assert_eq!(float, 3.2f32);
        assert_eq!(right, true);
        assert_eq!(s, "blue".to_owned());

        let h2 = hlist![13.5f32, "hello", Some(41)];
        let hlist_pat![a, b, c,] = h2;
        assert_eq!(a, 13.5f32);
        assert_eq!(b, "hello");
        assert_eq!(c, Some(41));
    }

    #[test]
    fn test_add() {
        let h1 = hlist![true, "hi"];
        let h2 = hlist![1, 32f32];
        let combined = h1 + h2;
        assert_eq!(combined, hlist![true, "hi", 1, 32f32])
    }

    #[test]
    fn test_into_reverse() {
        let h1 = hlist![true, "hi"];
        let h2 = hlist![1, 32f32];
        assert_eq!(h1.into_reverse(), hlist!["hi", true]);
        assert_eq!(h2.into_reverse(), hlist![32f32, 1]);
    }

    #[test]
    fn test_foldr_consuming() {
        let h = hlist![1, false, 42f32];
        let folded = h.foldr(
            hlist![
                |i, acc| i + acc,
                |_, acc| if acc > 42f32 { 9000 } else { 0 },
                |f, acc| f + acc,
            ],
            1f32,
        );
        assert_eq!(folded, 9001)
    }

    #[test]
    fn test_single_func_foldr_consuming() {
        let h = hlist![1, 2, 3];
        let folded = h.foldr(&|i, acc| i * acc, 1);
        assert_eq!(folded, 6)
    }

    #[test]
    fn test_foldr_non_consuming() {
        let h = hlist![1, false, 42f32];
        let folder = hlist![
            |&i, acc| i + acc,
            |&_, acc| if acc > 42f32 { 9000 } else { 0 },
            |&f, acc| f + acc
        ];
        let folded = h.to_ref().foldr(folder, 1f32);
        assert_eq!(folded, 9001)
    }

    #[test]
    fn test_foldl_consuming() {
        let h = hlist![1, false, 42f32];
        let folded = h.foldl(
            hlist![
                |acc, i| i + acc,
                |acc, b: bool| if !b && acc > 42 { 9000f32 } else { 0f32 },
                |acc, f| f + acc,
            ],
            1,
        );
        assert_eq!(42f32, folded)
    }

    #[test]
    fn test_foldl_non_consuming() {
        let h = hlist![1, false, 42f32];
        let folded = h.to_ref().foldl(
            hlist![
                |acc, &i| i + acc,
                |acc, b: &bool| if !b && acc > 42 { 9000f32 } else { 0f32 },
                |acc, &f| f + acc,
            ],
            1,
        );
        assert_eq!(42f32, folded);
        assert_eq!((&h.head), &1);
    }

    #[test]
    fn test_map_consuming() {
        let h = hlist![9000, "joe", 41f32];
        let mapped = h.map(hlist![|n| n + 1, |s| s, |f| f + 1f32]);
        assert_eq!(mapped, hlist![9001, "joe", 42f32]);
    }

    #[test]
    fn test_poly_map_consuming() {
        let h = hlist![9000, "joe", 41f32, "schmoe", 50];
        impl Func<i32> for P {
            type Output = bool;
            fn call(args: i32) -> Self::Output {
                args > 100
            }
        }
        impl<'a> Func<&'a str> for P {
            type Output = usize;
            fn call(args: &'a str) -> Self::Output {
                args.len()
            }
        }
        impl Func<f32> for P {
            type Output = String;
            fn call(args: f32) -> Self::Output {
                format!("{}", args)
            }
        }
        struct P;
        assert_eq!(h.map(Poly(P)), hlist![true, 3, "41".to_string(), 6, false]);
    }

    #[test]
    fn test_poly_map_non_consuming() {
        let h = hlist![9000, "joe", 41f32, "schmoe", 50];
        impl<'a> Func<&'a i32> for P {
            type Output = bool;
            fn call(args: &'a i32) -> Self::Output {
                *args > 100
            }
        }
        impl<'a> Func<&'a &'a str> for P {
            type Output = usize;
            fn call(args: &'a &'a str) -> Self::Output {
                args.len()
            }
        }
        impl<'a> Func<&'a f32> for P {
            type Output = String;
            fn call(args: &'a f32) -> Self::Output {
                format!("{}", args)
            }
        }
        struct P;
        assert_eq!(
            h.to_ref().map(Poly(P)),
            hlist![true, 3, "41".to_string(), 6, false]
        );
    }

    #[test]
    fn test_map_single_func_consuming() {
        let h = hlist![9000, 9001, 9002];
        let mapped = h.map(|v| v + 1);
        assert_eq!(mapped, hlist![9001, 9002, 9003]);
    }

    #[test]
    fn test_map_single_func_non_consuming() {
        let h = hlist![9000, 9001, 9002];
        let mapped = h.to_ref().map(|v| v + 1);
        assert_eq!(mapped, hlist![9001, 9002, 9003]);
    }

    #[test]
    fn test_map_non_consuming() {
        let h = hlist![9000, "joe", 41f32];
        let mapped = h.to_ref().map(hlist![|&n| n + 1, |&s| s, |&f| f + 1f32]);
        assert_eq!(mapped, hlist![9001, "joe", 42f32]);
    }

    #[test]
    fn test_sculpt() {
        let h = hlist![9000, "joe", 41f32];
        let (reshaped, remainder): (Hlist!(f32, i32), _) = h.sculpt();
        assert_eq!(reshaped, hlist![41f32, 9000]);
        assert_eq!(remainder, hlist!["joe"])
    }

    #[test]
    fn test_len_const() {
        assert_eq!(<Hlist![usize, &str, f32] as HList>::LEN, 3);
    }

    #[test]
    fn test_single_func_foldl_consuming() {
        use std::collections::HashMap;
        let h = hlist![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
        ];
        let r = h.foldl(
            |mut acc: HashMap<&'static str, isize>, (k, v)| {
                acc.insert(k, v);
                acc
            },
            HashMap::with_capacity(5),
        );
        let expected = {
            let mut m = HashMap::with_capacity(5);
            let vec = vec![
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
            ];
            for (k, v) in vec {
                m.insert(k, v);
            }
            m
        };
        assert_eq!(r, expected);
    }

    #[test]
    fn test_single_func_foldl_non_consuming() {
        let h = hlist![1, 2, 3, 4, 5];
        let r: isize = h.to_ref().foldl(|acc, &next| acc + next, 0isize);
        assert_eq!(r, 15);
    }

    #[test]
    fn test_into_vec() {
        let h = hlist![1, 2, 3, 4, 5];
        let as_vec: Vec<_> = h.into();
        assert_eq!(as_vec, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn test_lift() {
        type H = Hlist![(), usize, f64, (), bool];

        // Ensure type inference works as expected first:
        let x: H = 1337.lift_into();
        assert_eq!(x, hlist![(), 1337, 0.0, (), false]);

        let x = H::lift_from(42.0);
        assert_eq!(x, hlist![(), 0, 42.0, (), false]);

        let x: H = lift_from(true);
        assert_eq!(x, hlist![(), 0, 0.0, (), true]);

        // Sublists:
        let x: H = hlist![(), true].lift_into();
        assert_eq!(x, hlist![(), 0, 0.0, (), true]);

        let x: H = hlist![3.0, ()].lift_into();
        assert_eq!(x, hlist![(), 0, 3.0, (), false]);

        let x: H = hlist![(), 1337].lift_into();
        assert_eq!(x, hlist![(), 1337, 0.0, (), false]);

        let x: H = hlist![(), 1337, 42.0, (), true].lift_into();
        assert_eq!(x, hlist![(), 1337, 42.0, (), true]);
    }
}
