//! Module for holding the Semigroup typeclass definition and typeclass instances
//!
//! You can, for example, combine tuples.
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate frunk;
//! # #[macro_use] extern crate frunk_core;
//! # use frunk_core::hlist::*; fn main() {
//! use frunk_core::hlist::*;
//! use frunk::semi::*;
//! let t1 = (1, 2.5f32, String::from("hi"), Some(3));
//! let t2 = (1, 2.5f32, String::from(" world"), None);
//!
//! let expected = (2, 5.0f32, String::from("hi world"), Some(3));
//!
//! assert_eq!(t1.combine(t2), expected);
//!
//! // ultimately, the Tuple-based Semigroup implementations are only available for a maximum of
//! // 26 elements. If you need more, use HList, which is has no such limit.
//!
//! let h1 = hlist![1, 3.3, 53i64];
//! let h2 = hlist![2, 1.2, 1i64];
//! let h3 = hlist![3, 4.5, 54];
//! assert_eq!(h1.combine(h2), h3)
//! # }
//! ```

use frunk_core::hlist::*;
use std::ops::{Deref, BitAnd, BitOr};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::cell::*;
use std::hash::Hash;
use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;

/// Wrapper type for types that are ordered and can have a Max combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub struct Max<T: Ord>(pub T);

/// Wrapper type for types that are ordered and can have a Min combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub struct Min<T: Ord>(pub T);

/// Wrapper type for types that can have a Product combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub struct Product<T>(pub T);

/// Wrapper type for boolean that acts as a bitwise && combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub struct All<T>(pub T);

/// Wrapper type for boolean that acts as a bitwise || combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub struct Any<T>(pub T);

/// Output is a parameter because we want to allow Semi to not
/// necessarily return Self (e.g. in the case of Self being a pointer)
///
/// RHS is also a parameter in case we need to chain more combinations together..
///
/// Also, using a type parameter instead of an associated type because
/// there was a weird diverging trait search going with an associated type.
///
/// This means that yes, we need to enforce things with Laws.
pub trait Semi<Output = Self, RHS = Self> {
    fn combine(self, other: RHS) -> Output;
}


/// Allow the combination of any two HLists having the same structure
/// if all of the sub-element types are also Semiups
impl<H, HO, T, TO> Semi<HCons<HO, TO>> for HCons<H, T>
where
    H: Semi<HO>,
    T: HList + Semi<TO>,
{
    fn combine(self, other: Self) -> HCons<HO, TO> {
        let tail_comb = self.tail.combine(other.tail);
        let h_comb = self.head.combine(other.head);
        HCons {
            head: h_comb,
            tail: tail_comb,
        }
    }
}

/// Since () + () = (), the same is true for HNil
impl Semi for HNil {
    fn combine(self, _: Self) -> Self {
        self
    }
}

/// Allow the combination of any two HLists having the same structure
/// if all of the sub-element types are also Semiups
impl<'a, H, HO, T, TO> Semi<HCons<HO, TO>> for &'a HCons<H, T>
where
    &'a H: Semi<HO>,
    &'a T: HList + Semi<TO>,
{
    fn combine(self, other: Self) -> HCons<HO, TO> {
        let tail_comb = self.tail.combine(&other.tail);
        let h_comb = self.head.combine(&other.head);
        HCons {
            head: h_comb,
            tail: tail_comb,
        }
    }
}

/// Since () + () = (), the same is true for HNil
impl<'a> Semi<HNil> for &'a HNil {
    fn combine(self, _: Self) -> HNil {
        HNil
    }
}

impl<T, TO> Semi<Option<TO>> for Option<T>
where
    T: Semi<TO>,
    TO: From<T>,
{
    fn combine(self, other: Self) -> Option<TO> {
        if let Some(s) = self {
            if let Some(o) = other {
                Some(s.combine(o))
            } else {
                Some(TO::from(s))
            }
        } else {
            other.map(TO::from)
        }
    }
}

impl<'a, T, TO> Semi<Option<TO>> for &'a Option<T>
where
    &'a T: Semi<TO>,
    T: Clone,
    TO: From<T>,
{
    fn combine(self, other: Self) -> Option<TO> {
        if let &Some(ref s) = self {
            if let &Some(ref o) = other {
                Some(s.combine(o))
            } else {
                (*self).clone().map(TO::from)
            }
        } else {
            (*other).clone().map(TO::from)
        }
    }
}


/// Return this combined with itself `n` times.
pub fn combine_n<T>(o: T, times: u32) -> T
where
    T: Semi + Clone,
{
    // note: range is non-inclusive in the upper bound
    let mut x = o.clone();
    for _ in 1..times {
        x = x.combine(o.clone());
    }
    x
}

/// Given a sequence of `xs`, combine them and return the total
///
/// If the sequence is empty, returns None. Otherwise, returns Some(total).
///
/// # Examples
///
/// ```
/// # use frunk::semi::*;
/// let v1 = vec![1, 2, 3];
/// assert_eq!(combine_all_option(v1), Some(6));
///
/// let v2: Vec<i16> = Vec::new(); // empty!
/// assert_eq!(combine_all_option(v2), None);
/// ```
pub fn combine_all_option<T>(mut xs: Vec<T>) -> Option<T>
where
    T: Semi,
{
    if xs.len() < 1 {
        None
    } else {
        // Chop off xs
        let tail = xs.split_off(1);
        xs.pop().map(|mut x| {
            for i in tail.into_iter() {
                x = x.combine(i)
            }
            x
        })
    }
}

macro_rules! numeric_semi_imps {
  ($($tr:ty),*) => {
    $(
      impl Semi for $tr {
        fn combine(self, other: Self) -> $tr { self + other }
      }
      impl <'a> Semi<$tr, &'a $tr> for $tr {
        fn combine(self, other: &'a $tr) -> $tr { self + other }
      }
      impl <'a> Semi<$tr> for &'a $tr {
        fn combine(self, other: Self) -> $tr { self + other }
      }
    )*
  }
}

numeric_semi_imps!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);

impl<Str: Borrow<str>> Semi<String, Str> for String {
    fn combine(self, other: Str) -> Self {
        let mut s = self;
        s.push_str(other.borrow());
        s
    }
}

impl<'a, Str: Borrow<str>> Semi<String, Str> for &'a str {
    fn combine(self, other: Str) -> String {
        let s = self.to_string();
        s.combine(other)
    }
}

impl<T, TO> Semi<Box<TO>> for Box<T>
where
    T: Semi<TO>,
{
    fn combine(self, other: Self) -> Box<TO> {
        let s = *self;
        let o = *other;
        Box::new(s.combine(o))
    }
}

impl<'a, T, TO> Semi<Box<TO>> for &'a Box<T>
where
    &'a T: Semi<TO>,
{
    fn combine(self, other: Self) -> Box<TO> {
        let s = self.deref();
        let o = other.deref();
        Box::new(s.combine(o))
    }
}

impl<T> Semi for Vec<T> {
    fn combine(self, other: Self) -> Self {
        let mut v = self;
        let mut o = other;
        v.append(&mut o);
        v
    }
}

impl<'a, T: Clone> Semi<Vec<T>> for &'a Vec<T> {
    fn combine(self, other: Self) -> Vec<T> {
        let mut v = self.clone();
        v.extend_from_slice(other);
        v
    }
}

impl<T> Semi for Max<T>
where
    T: Ord,
{
    fn combine(self, Max(y): Self) -> Self {
        let Max(x) = self;
        match x.cmp(&y) {
            Ordering::Less => Max(y),
            _ => Max(x),
        }
    }
}
impl<T> Semi for Min<T>
where
    T: Ord,
{
    fn combine(self, Min(y): Self) -> Self {
        let Min(x) = self;
        match x.cmp(&y) {
            Ordering::Less => Min(x),
            _ => Min(y),
        }
    }
}

// Deriving for all BitAnds sucks because we are then bound on ::Output, which may not be the same type
macro_rules! simple_all_impls {
    ($($tr:ty)*) => {
        $(
            impl Semi for All<$tr> {
                fn combine(self, other: Self) -> Self {
                    let x = self.0;
                    let y = other.0;
                    All(x.bitand(y))
                }
            }
        )*
    }
}

simple_all_impls! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 }


macro_rules! simple_any_impls {
    ($($tr:ty)*) => {
        $(
            impl Semi for Any<$tr> {
                fn combine(self, other: Self) -> Self {
                    let x = self.0;
                    let y = other.0;
                    Any(x.bitor(y))
                }
            }
        )*
    }
}

simple_any_impls! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 }


macro_rules! numeric_product_semigroup_imps {
  ($($tr:ty),*) => {
    $(
      impl Semi for Product<$tr> {
        fn combine(self, other: Self) -> Self {
            let Product(x) = self;
            let Product(y) = other;
            Product(x * y)
         }
      }
    )*
  }
}

numeric_product_semigroup_imps!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);

impl<T> Semi for Cell<T>
where
    T: Semi + Copy,
{
    fn combine(self, other: Self) -> Self {
        Cell::new(self.get().combine((other.get())))
    }
}

impl<T, Out> Semi<RefCell<Out>> for RefCell<T>
where
    T: ToOwned,
    <T as ToOwned>::Owned: Semi<Out>,
{
    fn combine(self, other: Self) -> RefCell<Out> {
        let self_b = self.borrow().deref().to_owned();
        let other_b = other.borrow().to_owned();
        RefCell::new(self_b.combine(other_b))
    }
}

impl<T> Semi for HashSet<T>
where
    T: Eq + Hash,
{
    fn combine(self, other: Self) -> Self {
        let mut h = HashSet::new();
        for i in self {
            h.insert(i);
        }
        for i in other {
            h.insert(i);
        }
        h
    }
}

impl<K, V> Semi for HashMap<K, V>
where
    K: Eq + Hash,
    V: Semi,
{
    fn combine(self, other: Self) -> Self {
        let mut h: HashMap<K, V> = HashMap::new();
        for (k, v) in self {
            h.insert(k, v);
        }
        let mut combined = vec![];
        for (k, v) in other {
            match h.entry(k) {
                Entry::Occupied(o) => {
                    // Store and insert later
                    let (k, existing) = o.remove_entry();
                    let comb = existing.combine(v);
                    combined.push((k, comb));
                }
                Entry::Vacant(o) => {
                    o.insert(v);
                }
            }
        }
        for (k, v) in combined {
            h.insert(k, v);
        }
        h
    }
}


macro_rules! tuple_impls {
    () => {}; // no more

    (($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident), )*) => {
// Invoke recursive reversal of list that ends in the macro expansion implementation
// of the reversed list
//
        tuple_impls!([($idx, $typ);] $( ($nidx => $ntyp), )*);
        tuple_impls!($( ($nidx => $ntyp), )*); // invoke macro on tail
    };

// ([accumulatedList], listToReverse); recursively calls tuple_impls until the list to reverse
// + is empty (see next pattern)
//
    ([$(($accIdx: tt, $accTyp: ident);)+]  ($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident), )*) => {
      tuple_impls!([($idx, $typ); $(($accIdx, $accTyp); )*] $( ($nidx => $ntyp), ) *);
    };

// Finally expand into our implementation
    ([($idx:tt, $typ:ident); $( ($nidx:tt, $ntyp:ident); )*]) => {
        impl<$typ: Semi, $( $ntyp: Semi),*> Semi for ($typ, $( $ntyp ),*) {
            fn combine(self, other: Self) -> Self {
                (self.$idx.combine(other.$idx), $(self.$nidx.combine(other.$nidx), )*)
            }
        }
    }
}

tuple_impls! {
    (20 => U),
    (19 => T),
    (18 => S),
    (17 => R),
    (16 => Q),
    (15 => P),
    (14 => O),
    (13 => N),
    (12 => M),
    (11 => L),
    (10 => K),
    (9 => J),
    (8 => I),
    (7 => H),
    (6 => G),
    (5 => F),
    (4 => E),
    (3 => D),
    (2 => C),
    (1 => B),
    (0 => A),
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! semi_tests {
      ($($name:ident, $comb: expr => $expected: expr, $tr:ty)+) => {
        $(
          #[test]
          fn $name() {
            let r: $tr = $comb;
            assert_eq!(r, $expected)
          }
        )*
      }
    }

    semi_tests! {
        test_i8, 1.combine(2) => 3, i8
        test_i8_2, (&1).combine(&2) => 3, i8
        test_i16, 1.combine(2) => 3, i16
        test_i32, 1.combine(2) => 3, i32
        test_u8, 1.combine(2) => 3, u8
        test_u16, 1.combine(2) => 3, u16
        test_u32, 1.combine(2) => 3, u32
        test_usize, 1.combine(2) => 3, usize
        test_isize, 1.combine(2) => 3, isize
        test_f32, 1f32.combine(2f32) => 3f32, f32
        test_f64, 1f64.combine(2f64) => 3f64, f64
        test_option_i16, Some(1).combine(Some(2)) => Some(3), Option<i16>
        test_option_i16_none1, None.combine(Some(2)) => Some(2), Option<i16>
        test_option_i16_none2, Some(2).combine(None) => Some(2), Option<i16>
        test_option_i16_ref, (&Some(1)).combine(&Some(2)) => Some(3), Option<i16>
        test_option_i16_none1_ref, (&None).combine(&Some(2)) => Some(2), Option<i16>
        test_option_i16_none2_ref, (&Some(2)).combine(&None) => Some(2), Option<i16>
    }

    #[test]
    fn test_combine_hlist() {
        let h1 = hlist![Some(1), 3.3, 53i64, "hello".to_owned()];
        let h2 = hlist![Some(2), 1.2, 1i64, " world".to_owned()];
        let h3 = hlist![Some(3), 4.5, 54, "hello world".to_owned()];
        assert_eq!(h1.combine(h2), h3)
    }

    #[test]
    fn test_combine_hlist_2() {
        let h1 = hlist![Some(1), 3.3, 53i64, "hello"];
        let h2 = hlist![Some(2), 1.2, 1i64, " world"];
        let h3 = hlist![Some(3), 4.5, 54, "hello world".to_owned()]; // sadly types don't line up otherwise
        assert_eq!(h1.combine(h2), h3)
    }

    #[test]
    fn test_combine_str() {
        assert_eq!("hello".combine(" world"), "hello world")
    }

    #[test]
    fn test_combine_string() {
        assert_eq!(
            "hello".to_string().combine(" world".to_string()),
            "hello world"
        )
    }

    #[test]
    fn test_combine_str_chained() {
        assert_eq!("hello".combine(" world").combine(" yay"), "hello world yay")
    }

    #[test]
    fn test_combine_i32_chained() {
        let i1: i32 = 50;
        let i2: i32 = 51;
        let i3: i32 = 52;
        let c: i32 = (&i1).combine(&i2).combine(&i3);
        assert_eq!(c, 153)
    }

    #[test]
    fn test_combine_option_str() {
        assert_eq!(
            Some("hello").combine(Some(" world")),
            Some("hello world".to_string())
        )
    }


    #[test]
    fn test_max() {
        assert_eq!(Max(1).combine(Max(2)), Max(2));

        let v = vec![Max(1), Max(2), Max(3)];
        assert_eq!(combine_all_option(v), Some(Max(3)));
    }

    #[test]
    fn test_min() {
        assert_eq!(Min(1).combine(Min(2)), Min(1));

        let v = vec![Min(1), Min(2), Min(3)];
        assert_eq!(combine_all_option(v), Some(Min(1)));
    }

    #[test]
    fn test_all() {
        assert_eq!(All(3).combine(All(5)), All(1));
        assert_eq!(All(true).combine(All(false)), All(false));
    }

    #[test]
    fn test_any() {
        assert_eq!(Any(3).combine(Any(5)), Any(7));
        assert_eq!(Any(true).combine(Any(false)), Any(true));
    }

    #[test]
    fn test_combine_all_option() {
        let v1 = vec![1, 2, 3];
        assert_eq!(combine_all_option(v1), Some(6));
        let v2 = vec![Some(1), Some(2), Some(3)];
        assert_eq!(combine_all_option(v2), Some(Some(6)));
    }

    #[test]
    fn test_combine_n() {
        assert_eq!(combine_n(1, 3), 3);
        assert_eq!(combine_n(2, 1), 2);
        assert_eq!(combine_n(Some(2), 4), Some(8));
    }

    #[test]
    fn test_hashset() {
        let mut v1 = HashSet::new();
        v1.insert(1);
        v1.insert(2);
        assert!(!v1.contains(&3));
        let mut v2 = HashSet::new();
        v2.insert(3);
        v2.insert(4);
        assert!(!v2.contains(&1));
        let mut expected = HashSet::new();
        expected.insert(1);
        expected.insert(2);
        expected.insert(3);
        expected.insert(4);
        assert_eq!(v1.combine(v2), expected)
    }


    #[test]
    fn test_hashmap() {
        let mut v1: HashMap<i32, Option<String>> = HashMap::new();
        v1.insert(1, Some("Hello".to_owned()));
        v1.insert(2, Some("Goodbye".to_owned()));
        v1.insert(4, None);
        let mut v2: HashMap<i32, Option<String>> = HashMap::new();
        v2.insert(1, Some(" World".to_owned()));
        v2.insert(4, Some("Nope".to_owned()));
        let mut expected = HashMap::new();
        expected.insert(1, Some("Hello World".to_owned()));
        expected.insert(2, Some("Goodbye".to_owned()));
        expected.insert(4, Some("Nope".to_owned()));
        assert_eq!(v1.combine(v2), expected)
    }

    #[test]
    fn test_refcell() {
        let v1 = RefCell::new(1);
        let v2 = RefCell::new(2);
        assert_eq!(v1.combine(v2), RefCell::new(3))
    }

}
