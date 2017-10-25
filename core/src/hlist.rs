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
//! // Mapping over an HList (we use as_ref() to map over the HList without consuming it,
//! // but you can use the value-consuming version by leaving it off.)
//! let mapped = h3.as_ref().map(hlist![|&n| n + 1,
//!                                     |&s| s,
//!                                     |&f| f + 1f32]);
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
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
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
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
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
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
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
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
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
/// # use frunk_core::hlist::*;
///
/// let h = h_cons(1, HNil);
/// let h = h.head;
/// assert_eq!(h, 1);
/// ```
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub struct HNil;

impl HList for HNil {
    const LEN: usize = 0;
    fn static_len() -> usize {
        Self::LEN
    }
}

impl AsRef<HNil> for HNil {
    fn as_ref(&self) -> &HNil {
        self
    }
}

/// Represents the most basic non-empty HList. Its value is held in `head`
/// while its tail is another HList.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
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

impl<H, T> AsRef<HCons<H, T>> for HCons<H, T> {
    fn as_ref(&self) -> &HCons<H, T> {
        self
    }
}

impl<H, T> HCons<H, T> {
    /// Returns the head of the list and the tail of the list as a tuple2.
    /// The original list is consumed
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
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

/// Returns an `HList` based on the values passed in.
///
/// Helps to avoid having to write nested `HCons`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
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
/// # }
/// ```
#[macro_export]
macro_rules! hlist {

    // Nothing
    () => { $crate::hlist::HNil };

    // Just a single item
    ($single: expr) => {
        $crate::hlist::HCons { head: $single, tail: $crate::hlist::HNil }
    };

    ($first: expr, $( $repeated: expr ), +) => {
        $crate::hlist::HCons { head: $first, tail: hlist!($($repeated), *)}
    };

    // <-- Forwarding of trailing comma variants
    ($first: expr, $( $repeated: expr, ) +) => {
        hlist!($first, $($repeated),*)
    };

    ($first: expr, ) => {
        hlist!($first)
    };
    // Forwarding of trailing comma variants -->

}

/// Macro for pattern-matching on HLists.
///
/// Taken from https://github.com/tbu-/rust-rfcs/blob/master/text/0873-type-macros.md
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
/// let h = hlist![13.5f32, "hello", Some(41)];
/// let hlist_pat![h1, h2, h3] = h;
/// assert_eq!(h1, 13.5f32);
/// assert_eq!(h2, "hello");
/// assert_eq!(h3, Some(41));
/// # }
/// ```
#[macro_export]
macro_rules! hlist_pat {
    {} => { $crate::hlist::HNil };
    { $head:pat, $($tail:tt), +} => { $crate::hlist::HCons{ head: $head, tail: hlist_pat!($($tail),*) } };
    { $head:pat } => { $crate::hlist::HCons { head: $head, tail: $crate::hlist::HNil } };

    // <-- Forward trailing comma variants
    { $head:pat, $($tail:tt,) +} => { hlist_pat!($head, $($tail),*) };
    { $head:pat, } => { hlist_pat!($head) };
    // Forward trailing comma variants -->
}

/// Returns a type signature for an HList of the provided types
///
/// This is a type macro (introduced in Rust 1.13) that makes it easier
/// to write nested type signatures.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
/// let h: Hlist!(f32, &str, Option<i32>) = hlist![13.5f32, "hello", Some(41)];
/// # }
/// ```
#[macro_export]
macro_rules! Hlist {
    // Nothing
    () => { $crate::hlist::HNil };

    // Just a single item
    ($single: ty) => {
        $crate::hlist::HCons<$single, $crate::hlist::HNil>
    };

    ($first: ty, $( $repeated: ty ), +) => {
        $crate::hlist::HCons<$first, Hlist!($($repeated), *)>
    };

    // <-- Forward trailing comma variants
    ($single: ty,) => {
        Hlist![$single]
    };

    ($first: ty, $( $repeated: ty, ) +) => {
        Hlist![$first, $($repeated),*]
    };
    // Forward trailing comma variants -->
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

/// Trait for retrieving an HList element by type
pub trait Selector<S, I> {
    /// Allows you to retrieve a unique type from an HList
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    /// let h = hlist![1, "hello", true, 42f32];
    ///
    /// let f: &f32 = h.get();
    /// let b: &bool = h.get();
    /// assert_eq!(*f, 42f32);
    /// assert!(b)
    /// # }
    /// ```
    fn get(&self) -> &S;
}

impl<T, Tail> Selector<T, Here> for HCons<T, Tail> {
    fn get(&self) -> &T {
        &self.head
    }
}

impl<Head, Tail, FromTail, TailIndex> Selector<FromTail, There<TailIndex>> for HCons<Head, Tail>
    where Tail: Selector<FromTail, TailIndex>
{
    fn get(&self) -> &FromTail {
        self.tail.get()
    }
}

/// Trait defining extraction from a given HList
///
/// Similar to Selector, but returns the target and the remainder of the list (w/o target)
/// in a pair.
pub trait Plucker<Target, Index> {
    /// What is left after you pluck the target from the Self
    type Remainder;

    /// Returns the target with the remainder of the list in a pair
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    /// let h = hlist![1, "hello", true, 42f32];
    /// let (t, r): (bool, _) = h.pluck();
    /// assert!(t);
    /// assert_eq!(r, hlist![1, "hello", 42f32])
    /// # }
    /// ```
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
    where Tail: Plucker<FromTail, TailIndex>
{
    type Remainder = HCons<Head, <Tail as Plucker<FromTail, TailIndex>>::Remainder>;

    fn pluck(self) -> (FromTail, Self::Remainder) {
        let (target, tail_remainder): (FromTail,
                                       <Tail as Plucker<FromTail, TailIndex>>::Remainder) =
            <Tail as Plucker<FromTail, TailIndex>>::pluck(self.tail);
        (target,
         HCons {
             head: self.head,
             tail: tail_remainder,
         })
    }
}

/// An Sculptor trait, that allows us to extract/reshape/scult the current HList into another shape,
/// provided that the requested shape's types are are contained within the current HList.
///
/// The "Indices" type parameter allows the compiler to figure out that the Target and Self
/// can be morphed into each other
pub trait Sculptor<Target, Indices> {
    type Remainder;

    /// Consumes the current HList and returns an HList with the requested shape.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    /// let h = hlist![9000, "joe", 41f32, true];
    /// let (reshaped, remainder): (Hlist![f32, i32, &str], _) = h.sculpt();
    /// assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
    /// assert_eq!(remainder, hlist![true]);
    /// # }
    /// ```
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
    type Remainder = <<HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder as Sculptor<TTail, IndexTail>>::Remainder;

    #[inline(always)]
    fn sculpt(self) -> (HCons<THead, TTail>, Self::Remainder) {
        let (p, r): (THead,
                     <HCons<SHead, STail> as Plucker<
            THead,
            IndexHead,
        >>::Remainder) = self.pluck();
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
/// Implemented for HCons and HNil.
pub trait IntoReverse {
    type Output;

    /// Reverses a given data structure.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    /// let nil = HNil;
    ///
    /// assert_eq!(nil.into_reverse(), nil);
    ///
    /// let h = hlist![1, "hello", true, 42f32];
    /// assert_eq!(h.into_reverse(), hlist![42f32, true, "hello", 1])
    /// # }
    /// ```
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
        self.tail.into_reverse() +
            HCons {
                head: self.head,
                tail: HNil,
            }
    }
}

/// Trail that allow for mapping over a data structure using mapping functions stored in another
/// data structure
///
/// It might be a good idea to try to re-write these using the foldr variants, but it's a
/// wee-bit more complicated.
pub trait HMappable<Mapper, Index> {
    type Output;


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
    /// let mapped = h.as_ref().map(hlist![
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
    fn map(self, folder: Mapper) -> Self::Output;
}

impl<F> HMappable<F, Here> for HNil {
    type Output = HNil;

    fn map(self, _: F) -> Self::Output {
        self
    }
}

impl<'a, F, R, H> HMappable<HCons<F, HNil>, Here> for &'a HCons<H, HNil>
where
    F: FnOnce(&'a H) -> R,
{
    type Output = HCons<R, HNil>;

    fn map(self, f: HCons<F, HNil>) -> Self::Output {
        let ref h = self.head;
        let f = f.head;
        HCons {
            head: f(h),
            tail: HNil,
        }
    }
}


impl<'a, F, R, H> HMappable<F, Here> for &'a HCons<H, HNil>
where
    F: Fn(&'a H) -> R,
{
    type Output = HCons<R, HNil>;

    fn map(self, f: F) -> Self::Output {
        let ref h = self.head;
        HCons {
            head: f(h),
            tail: HNil,
        }
    }
}

impl<F, MapperHeadR, MapperTail, H, Tail, Index> HMappable<HCons<F, MapperTail>, There<Index>>
    for HCons<H, Tail>
where
    F: FnOnce(H) -> MapperHeadR,
    Tail: HMappable<MapperTail, Index>,
{
    type Output = HCons<MapperHeadR, <Tail as HMappable<MapperTail, Index>>::Output>;
    fn map(self, mapper: HCons<F, MapperTail>) -> Self::Output {
        let f = mapper.head;
        HCons {
            head: f(self.head),
            tail: self.tail.map(mapper.tail),
        }
    }
}

impl<F, R, H, Tail, Index> HMappable<F, There<Index>> for HCons<H, Tail>
where
    F: Fn(H) -> R,
    Tail: HMappable<F, Index>,
{
    type Output = HCons<R, <Tail as HMappable<F, Index>>::Output>;
    fn map(self, f: F) -> Self::Output {
        let r = f(self.head);
        HCons {
            head: r,
            tail: self.tail.map(f),
        }
    }
}

impl<'a, F, MapperHeadR, MapperTail, H, Tail, Index> HMappable<HCons<F, MapperTail>, There<Index>>
    for &'a HCons<H, Tail>
where
    F: FnOnce(&'a H) -> MapperHeadR,
    &'a Tail: HMappable<MapperTail, Index>,
{
    type Output = HCons<MapperHeadR, <&'a Tail as HMappable<MapperTail, Index>>::Output>;
    fn map(self, mapper: HCons<F, MapperTail>) -> Self::Output {
        let f = mapper.head;
        let mapper_tail = mapper.tail;
        let ref self_head = self.head;
        let ref self_tail = self.tail;
        HCons {
            head: f(self_head),
            tail: self_tail.map(mapper_tail),
        }
    }
}

impl<'a, F, R, H, Tail, Index> HMappable<F, There<Index>> for &'a HCons<H, Tail>
    where F: Fn(&'a H) -> R,
          &'a Tail: HMappable<F, Index>
{
    type Output = HCons<R, <&'a Tail as HMappable<F, Index>>::Output>;
    fn map(self, f: F) -> Self::Output {
        let ref self_head = self.head;
        let ref self_tail = self.tail;
        HCons {
            head: f(self_head),
            tail: self_tail.map(f),
        }
    }
}

/// Foldr for HLists
pub trait HFoldRightable<Folder, Init, Index> {
    type Output;

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
    fn foldr(self, folder: Folder, i: Init) -> Self::Output;
}

impl<F, Init> HFoldRightable<F, Init, Here> for HNil {
    type Output = Init;

    fn foldr(self, _: F, i: Init) -> Self::Output {
        i
    }
}


impl<
    F,
    FolderHeadR,
    FolderTail,
    H,
    Tail,
    Init,
    Index,
> HFoldRightable<HCons<F, FolderTail>, Init, There<Index>> for HCons<H, Tail>
where
    Tail: HFoldRightable<FolderTail, Init, Index>,
    F: FnOnce(H, <Tail as HFoldRightable<FolderTail, Init, Index>>::Output) -> FolderHeadR,
{
    type Output = FolderHeadR;

    fn foldr(self, folder: HCons<F, FolderTail>, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder.tail, init);
        (folder.head)(self.head, folded_tail)
    }
}


impl<
    'a,
    F,
    R,
    H,
    Tail,
    Init,
    Index,
> HFoldRightable<&'a F, Init, There<Index>> for HCons<H, Tail>
where
    Tail: HFoldRightable<&'a F, Init, Index>,
    F: Fn(H, <Tail as HFoldRightable<&'a F, Init, Index>>::Output) -> R,
{
    type Output = R;

    fn foldr(self, folder: &'a F, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder, init);
        (folder)(self.head, folded_tail)
    }
}

// TODO: enable this when the compiler stops smoking crack
// Likely same as https://github.com/rust-lang/rust/issues/39959
//
// At the moment, we get a diverging requirement evaluation a la
// overflow evaluating the requirement `<_ as std::ops::FnOnce<(&_, _)>>::Output`
//
// or
//
// overflow evaluating the requirement `<&_ as hlist::HFoldRightable<_, _, _>>::Output`
//
// Depending on the exit-case implementation
//
//impl<'a, F, Init> HFoldRightable<F, Init, Here> for &'a HNil {
//    type Output = Init;
//
//    fn foldr(self, _: F, i: Init) -> Self::Output {
//        i
//    }
//}
//
//
//impl<'a, F, FolderHeadR, FolderTail, H, Tail, Init, Index> HFoldRightable<HCons<F, FolderTail>, Init, There<Index>> for &'a HCons<H, Tail>
//    where
//        F: Fn(&'a H, <&'a Tail as HFoldRightable<FolderTail, Init, Index>>::Output) -> FolderHeadR,
//        &'a Tail: HFoldRightable<FolderTail, Init, Index>
//
//{
//    type Output = FolderHeadR;
//
//    fn foldr(self, folder: HCons<F, FolderTail>, init: Init) -> Self::Output {
//        let f_h = folder.head;
//        let f_tail = folder.tail;
//        let ref h = self.head;
//        let ref tail = self.tail;
//        let folded_tail = tail.foldr(f_tail, init);
//        (f_h)(h, folded_tail)
//    }
//}

/// Left fold for a given data structure
pub trait HFoldLeftable<Folder, Init, Index> {
    type Output;

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
    /// let folded = h.as_ref().foldl(
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
    fn foldl(self, folder: Folder, i: Init) -> Self::Output;
}

impl<F, Acc> HFoldLeftable<F, Acc, Here> for HNil {
    type Output = Acc;

    fn foldl(self, _: F, acc: Acc) -> Self::Output {
        acc
    }
}

impl<'a, F, R, H, Acc> HFoldLeftable<F, Acc, Here> for &'a HCons<H, HNil>
where
    F: FnOnce(Acc, &'a H) -> R,
{
    type Output = R;

    fn foldl(self, folder: F, acc: Acc) -> Self::Output {
        let ref h = self.head;
        folder(acc, h)
    }
}

impl<'a, F, R, H, Acc> HFoldLeftable<HCons<F, HNil>, Acc, Here> for &'a HCons<H, HNil>
    where F: FnOnce(Acc, &'a H) -> R
{
    type Output = R;

    fn foldl(self, folder: HCons<F, HNil>, acc: Acc) -> Self::Output {
        let f = folder.head;
        let ref h = self.head;
        f(acc, h)
    }
}

impl<
    F,
    FolderHeadR,
    FolderTail,
    H,
    Tail,
    Acc,
    Index,
> HFoldLeftable<HCons<F, FolderTail>, Acc, There<Index>> for HCons<H, Tail>
where
    Tail: HFoldLeftable<FolderTail, FolderHeadR, Index>,
    F: FnOnce(Acc, H) -> FolderHeadR,
{
    type Output = <Tail as HFoldLeftable<FolderTail, FolderHeadR, Index>>::Output;

    fn foldl(self, folder: HCons<F, FolderTail>, acc: Acc) -> Self::Output {
        self.tail.foldl(folder.tail, (folder.head)(acc, self.head))
    }
}

impl<
    'a,
    F,
    FolderHeadR,
    FolderTail,
    H,
    Tail,
    Acc,
    Index,
> HFoldLeftable<HCons<F, FolderTail>, Acc, There<Index>> for &'a HCons<H, Tail>
where
    &'a Tail: HFoldLeftable<FolderTail, FolderHeadR, Index>,
    F: FnOnce(Acc, &'a H) -> FolderHeadR,
{
    type Output = <&'a Tail as HFoldLeftable<FolderTail, FolderHeadR, Index>>::Output;

    fn foldl(self, folder: HCons<F, FolderTail>, acc: Acc) -> Self::Output {
        let ref h = self.head;
        let ref t = self.tail;
        let f_head = folder.head;
        let f_tail = folder.tail;
        t.foldl(f_tail, (f_head)(acc, h))
    }
}

/// Trait for things that can be turned into a Tuple 2 (pair)
pub trait IntoTuple2 {
    /// The 0 element in the output tuple
    type HeadType;

    /// The 1 element in the output tuple
    type TailOutput;

    /// Turns an HList into nested Tuple2s, which are less troublesome to pattern match
    /// and have a nicer type signature.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
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
    type TailOutput = (<Tail as IntoTuple2>::HeadType, <Tail as IntoTuple2>::TailOutput);

    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.into_tuple2())
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
impl<
    F,
    H,
    Tail,
    Acc,
    Index,
> HFoldLeftable<F, Acc, There<Index>> for HCons<H, Tail>
where
    Tail: HFoldLeftable<F, Acc, Index>,
    F: Fn(Acc, H) -> Acc,
{
    type Output = <Tail as HFoldLeftable<F, Acc, Index>>::Output;

    fn foldl(self, folder: F, acc: Acc) -> Self::Output {
        let acc = folder(acc, self.head);
        self.tail.foldl(folder, acc)
    }
}

impl<'a, F, H, Tail, Acc, Index> HFoldLeftable<F, Acc, There<Index>> for &'a HCons<H, Tail>
    where
        F: Fn(Acc, &'a H) -> Acc,
        &'a Tail: HFoldLeftable<F, Acc, Index>,
{
    type Output = <&'a Tail as HFoldLeftable<F, Acc, Index>>::Output;

    fn foldl(self, f: F, acc: Acc) -> Self::Output {
        let ref h = self.head;
        let ref t = self.tail;
        let result = f(acc, h);
        t.foldl(f, result)
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

impl<Head, Tail, ValAtIx, TailIx> LiftFrom<ValAtIx, There<TailIx>>
for HCons<Head, Tail>
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

impl<Prefix, Suffix> LiftFrom<Prefix, Suffixed<Suffix>>
for <Prefix as Add<Suffix>>::Output
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
        let h: Hlist
        !(i32, &str, i32) = hlist![1, "2", 3];
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
        let h1: Hlist
        !(i32, &str, i32) = hlist![1, "2", 3];
        let h2: Hlist
        !(i32, &str, i32, ) = hlist![1, "2", 3];
        let h3: Hlist
        !(i32) = hlist![1];
        let h4: Hlist
        !(i32, ) = hlist![1,];
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

    // Todo enable when compiler is fixed
    //    #[test]
    //    fn test_foldr_non_consuming() {
    //        let h = hlist![1, false, 42f32];
    //        let folder = hlist![|&i, acc| i + acc,
    //                                             |&_, acc| if acc > 42f32 { 9000 } else { 0 },
    //                                             |&f, acc| f + acc];
    //        let folded = h.as_ref().foldr(folder,
    //                                      1f32);
    //        assert_eq!(folded, 9001)
    //    }

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
        let folded = h.as_ref().foldl(
            hlist![
                |acc, &i| i + acc,
                |acc, b: &bool| if !b && acc > 42 { 9000f32 } else { 0f32 },
                |acc, &f| f + acc,
            ],
            1,
        );
        assert_eq!(42f32, folded)
    }

    #[test]
    fn test_map_consuming() {
        let h = hlist![9000, "joe", 41f32];
        let mapped = h.map(hlist![|n| n + 1, |s| s, |f| f + 1f32]);
        assert_eq!(mapped, hlist![9001, "joe", 42f32]);
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
        let mapped = h.as_ref().map(|v| v + 1);
        assert_eq!(mapped, hlist![9001, 9002, 9003]);
    }

    #[test]
    fn test_map_non_consuming() {
        let h = hlist![9000, "joe", 41f32];
        let mapped = h.as_ref().map(hlist![|&n| n + 1, |&s| s, |&f| f + 1f32]);
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
        let h =
            hlist![
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
        let r: isize = h.as_ref().foldl(|acc, &next| acc + next, 0isize);
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
