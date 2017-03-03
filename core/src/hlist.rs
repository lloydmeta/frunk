//! Module that holds HList data structures and implementations
//!
//! Typically, you would want to use the `hlist!` macro to make it easier
//! for you to use HList.
//!
//! ```
//! # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
//! let h = hlist![1, "hi"];
//! assert_eq!(h.length(), 2);
//! let (a, b) = h.into_tuple2();
//! assert_eq!(a, 1);
//! assert_eq!(b, "hi");
//! # }
//! ```

use std::ops::Add;
use std::marker::PhantomData;

/// Typeclass for HList-y behaviour
///
/// An HList is a heterogeneous list, one that is statically typed at compile time. In simple terms,
/// it is just an arbitrarily-nested Tuple2.
pub trait HList: Sized {
    /// Returns the length of a given HList
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    /// let h = hlist![1, "hi"];
    /// assert_eq!(h.length(), 2);
    /// # }
    /// ```
    fn length(&self) -> u32;

    /// Prepends an item to the current HList
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
/// Used to begin one:
///
/// ```
/// # use frunk_core::hlist::*;
///
/// let h = h_cons(1, HNil);
/// let h = h.head;
/// assert_eq!(h, 1);
/// ```
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct HNil;

impl HList for HNil {
    fn length(&self) -> u32 {
        0
    }
}

/// Represents the most basic non-empty HList. Its value is held in `head`
/// while its tail is another HList.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}

impl<H, T: HList> HList for HCons<H, T> {
    fn length(&self) -> u32 {
        1 + self.tail.length()
    }
}

impl<H, T> HCons<H, T> {
    /// Returns the head of the list and the tail of the list as a tuple2.
    /// The original list is consumed
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    ///
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
/// ```
/// # use frunk_core::hlist::*;
///
/// let h_list = h_cons("what", h_cons(1.23f32, HNil));
/// let (h1, h2) = h_list.into_tuple2();
/// assert_eq!(h1, "what");
/// assert_eq!(h2, 1.23f32);
/// ```
pub fn h_cons<H, T: HList>(h: H, tail: T) -> HCons<H, T> {
    tail.prepend(h)
}

/// Returns an `HList` based on the values passed in.
///
/// Helps to avoid having to write nested `HCons`.
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
///
/// let h = hlist![13.5f32, "hello", Some(41)];
/// let (h1, (h2, h3)) = h.into_tuple2();
/// assert_eq!(h1, 13.5f32);
/// assert_eq!(h2, "hello");
/// assert_eq!(h3, Some(41))
/// # }
/// ```
#[macro_export]
macro_rules! hlist {

    // Nothing
    () => { $crate::hlist::HNil };

    // Just a single item
    ($single: expr) => {
        $crate::hlist::HCons { head: $single, tail: HNil }
    };

    ($first: expr, $( $repeated: expr ), +) => {
        $crate::hlist::HCons { head: $first, tail: hlist!($($repeated), *)}
    };

}

/// Macro for pattern-matching on HLists.
///
/// Taken from https://github.com/tbu-/rust-rfcs/blob/master/text/0873-type-macros.md
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
///
/// let h = hlist![13.5f32, "hello", Some(41)];
/// let hlist_pat![h1, h2, h3] = h;
/// assert_eq!(h1, 13.5f32);
/// assert_eq!(h2, "hello");
/// assert_eq!(h3, Some(41))
/// # }
/// ```
#[macro_export]
macro_rules! hlist_pat {
    {} => { $crate::hlist::HNil };
    { $head:pat, $($tail:tt), +} => { $crate::hlist::HCons{ head: $head, tail: hlist_pat!($($tail),*) } };
    { $head:pat } => { $crate::hlist::HCons { head: $head, tail: $crate::hlist::HNil } };
}

/// Returns a type signature for an HList of the provided types
///
/// This is a type macro (introduced in Rust 1.13) that makes it easier
/// to write nested type signatures.
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
///
/// let h: Hlist!(f32, &str, Option<i32>) = hlist![13.5f32, "hello", Some(41)];
/// # }
/// ```
#[macro_export]
macro_rules! Hlist {
    // Nothing
    () => { $crate::hlist::HNil };

    // Just a single item
    ($single: ty) => {
        $crate::hlist::HCons<$single, HNil>
    };

    ($first: ty, $( $repeated: ty ), +) => {
        $crate::hlist::HCons<$first, Hlist!($($repeated), *)>
    };
}

impl<RHS> Add<RHS> for HNil
    where RHS: HList
{
    type Output = RHS;

    fn add(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, T, RHS> Add<RHS> for HCons<H, T>
    where T: Add<RHS>,
          RHS: HList
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
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    ///
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
pub trait Extractor<Target, Index> {
    type Remainder;

    /// Returns the target with the remainder of the list in a pair
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    ///
    ///
    /// let h = hlist![1, "hello", true, 42f32];
    /// let (t, r): (i32, _) = h.extract_from();
    /// assert_eq!(t, 1);
    /// assert_eq!(r, hlist!["hello", true, 42f32])
    /// # }
    /// ```
    fn extract_from(self) -> (Target, Self::Remainder);
}

impl<T, Tail> Extractor<T, Here> for HCons<T, Tail> {
    type Remainder = Tail;

    fn extract_from(self) -> (T, Self::Remainder) {
        (self.head, self.tail)
    }
}

impl<Head, Tail, FromTail, TailIndex> Extractor<FromTail, There<TailIndex>> for HCons<Head, Tail>
    where Tail: Extractor<FromTail, TailIndex>
{
    type Remainder = HCons<Head, <Tail as Extractor<FromTail, TailIndex>>::Remainder>;

    fn extract_from(self) -> (FromTail, Self::Remainder) {
        let (target, tail_remainder): (FromTail, <Tail as Extractor<FromTail, TailIndex>>::Remainder) =
            <Tail as Extractor<FromTail, TailIndex>>::extract_from(self.tail);
        (target,
         HCons {
             head: self.head,
             tail: tail_remainder,
         })
    }
}

/// Trait that allows for reversing a given data structure.
///
/// Implemented for HCons and HNil.
pub trait IntoReverse {
    type Output;

    /// Reverses a given data structure.
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    ///
    /// let nil = HNil;
    ///
    /// assert_eq!(nil.into_reverse(), nil);
    ///
    /// let h = hlist![1, "hello", true, 42f32];
    /// assert_eq!(h.into_reverse(), hlist![42f32, true, "hello", 1])
    ///
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
    where Tail: IntoReverse,
          <Tail as IntoReverse>::Output: Add<HCons<H, HNil>>
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
pub trait HMappable<Mapper> {
    type Output;


    /// Maps over the current data structure using functions stored in another
    /// data structure.
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    ///
    /// let nil = HNil;
    ///
    /// assert_eq!(nil.map(HNil), HNil);
    ///
    /// let h = hlist![1, false, 42f32];
    ///
    /// // Sadly we need to help the compiler understand the bool type in our mapper
    ///
    /// let mapped = h.map(hlist![
    ///     |n| n + 1,
    ///     |b: bool| !b,
    ///     |f| f + 1f32]);
    /// assert_eq!(mapped, hlist![2, true, 43f32]);
    /// # }
    /// ```
    fn map(self, folder: Mapper) -> Self::Output;
}

impl<F> HMappable<F> for HNil {
    type Output = HNil;

    fn map(self, _: F) -> Self::Output {
        self
    }
}

impl<F, MapperHeadR, MapperTail, H, Tail> HMappable<HCons<F, MapperTail>> for HCons<H, Tail>
    where F: FnOnce(H) -> MapperHeadR,
          Tail: HMappable<MapperTail>
{
    type Output = HCons<MapperHeadR, <Tail as HMappable<MapperTail>>::Output>;
    fn map(self, mapper: HCons<F, MapperTail>) -> Self::Output {
        let f = mapper.head;
        HCons {
            head: f(self.head),
            tail: self.tail.map(mapper.tail),
        }
    }
}

/// Foldr for HLists
pub trait HFoldRightable<Folder, Init> {
    type Output;

    /// foldr over a data structure
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    ///
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

impl<F, Init> HFoldRightable<F, Init> for HNil {
    type Output = Init;

    fn foldr(self, _: F, i: Init) -> Self::Output {
        i
    }
}

impl<F, FolderHeadR, FolderTail, H, Tail, Init> HFoldRightable<HCons<F, FolderTail>, Init> for HCons<H, Tail>
    where
        Tail: HFoldRightable<FolderTail, Init>,
        F: FnOnce(H, <Tail as HFoldRightable<FolderTail, Init>>::Output) -> FolderHeadR {
    type Output = FolderHeadR;

    fn foldr(self, folder: HCons<F, FolderTail>, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder.tail, init);
        (folder.head)(self.head, folded_tail)
    }
}

/// Left fold for a given data structure
pub trait HFoldLeftable<Folder, Init> {
    type Output;

    /// foldl over a data structure
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core; use frunk_core::hlist::*; fn main() {
    ///
    /// let nil = HNil;
    ///
    /// assert_eq!(nil.foldl(HNil, 0), 0);
    ///
    /// let h = hlist![1, false, 42f32];
    ///
    /// let folded = h.foldl(
    ///     hlist![
    ///         |acc, i| i + acc,
    ///         |acc, b: bool| if !b && acc > 42 { 9000f32 } else { 0f32 },
    ///         |acc, f| f + acc
    ///     ],
    ///     1
    /// );
    ///
    /// assert_eq!(42f32, folded)
    ///
    /// # }
    /// ```
    fn foldl(self, folder: Folder, i: Init) -> Self::Output;
}

impl<F, Acc> HFoldLeftable<F, Acc> for HNil {
    type Output = Acc;

    fn foldl(self, _: F, acc: Acc) -> Self::Output {
        acc
    }
}

impl<F, FolderHeadR, FolderTail, H, Tail, Acc> HFoldLeftable<HCons<F, FolderTail>, Acc>
    for HCons<H, Tail>
    where Tail: HFoldLeftable<FolderTail, FolderHeadR>,
          F: FnOnce(Acc, H) -> FolderHeadR
{
    type Output = <Tail as HFoldLeftable<FolderTail, FolderHeadR>>::Output;

    fn foldl(self, folder: HCons<F, FolderTail>, acc: Acc) -> Self::Output {
        self.tail.foldl(folder.tail, (folder.head)(acc, self.head))
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
    where Tail: IntoTuple2
{
    type HeadType = T;
    type TailOutput = (<Tail as IntoTuple2>::HeadType, <Tail as IntoTuple2>::TailOutput);

    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.into_tuple2())
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
        assert_eq!(retrieved.length(), 1);
        let new_list = h_cons(2, retrieved);
        assert_eq!(new_list.length(), 2);
    }

    #[test]
    fn test_macro() {
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
    fn test_pattern_matching() {
        let h = hlist![5, 3.2f32, true, "blue".to_owned()];
        let hlist_pat!(five, float, right, s) = h;
        assert_eq!(five, 5);
        assert_eq!(float, 3.2f32);
        assert_eq!(right, true);
        assert_eq!(s, "blue".to_owned());
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
    fn test_foldr() {
        let h = hlist![1, false, 42f32];
        let folded = h.foldr(hlist![|i, acc| i + acc,
                                    |_, acc| if acc > 42f32 { 9000 } else { 0 },
                                    |f, acc| f + acc],
                             1f32);
        assert_eq!(folded, 9001)
    }

    #[test]
    fn test_foldl() {
        let h = hlist![1, false, 42f32];
        let folded = h.foldl(hlist![|acc, i| i + acc,
                                    |acc, b: bool| if !b && acc > 42 { 9000f32 } else { 0f32 },
                                    |acc, f| f + acc],
                             1);
        assert_eq!(42f32, folded)
    }

    #[test]
    fn test_map() {
        let h = hlist![9000, "joe", 41f32];
        let mapped = h.map(hlist![|n| n + 1, |s| s, |f| f + 1f32]);
        assert_eq!(mapped, hlist![9001, "joe", 42f32]);
    }
}
