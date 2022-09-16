//! Module for holding the Semigroup typeclass definition and typeclass instances
//!
//! You can, for example, combine tuples.
#![cfg_attr(
    feature = "std",
    doc = r#"
# Examples

```
# fn main() {
use frunk::Semigroup;
use frunk_core::hlist;

let t1 = (1, 2.5f32, String::from("hi"), Some(3));
let t2 = (1, 2.5f32, String::from(" world"), None);

let expected = (2, 5.0f32, String::from("hi world"), Some(3));

assert_eq!(t1.combine(&t2), expected);

// ultimately, the Tuple-based Semigroup implementations are only available for a maximum of
// 26 elements. If you need more, use HList, which is has no such limit.

let h1 = hlist![1, 3.3, 53i64];
let h2 = hlist![2, 1.2, 1i64];
let h3 = hlist![3, 4.5, 54];
assert_eq!(h1.combine(&h2), h3)
# }
```"#
)]

use frunk_core::hlist::*;
use std::cell::*;
use std::cmp::Ordering;
#[cfg(feature = "std")]
use std::collections::hash_map::Entry;
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "std")]
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, Deref};

/// Wrapper type for types that are ordered and can have a Max combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Max<T: Ord>(pub T);

/// Wrapper type for types that are ordered and can have a Min combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Min<T: Ord>(pub T);

/// Wrapper type for types that can have a Product combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Product<T>(pub T);

/// Wrapper type for boolean that acts as a bitwise && combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct All<T>(pub T);

/// Wrapper type for boolean that acts as a bitwise || combination
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Any<T>(pub T);

/// A Semigroup is a class of thing that has a definable combine operation
pub trait Semigroup {
    /// Associative operation taking which combines two values.
    ///
    /// # Examples
    ///
    /// ```
    /// use frunk::Semigroup;
    ///
    /// assert_eq!(Some(1).combine(&Some(2)), Some(3))
    /// ```
    fn combine(&self, other: &Self) -> Self;
}

/// Allow the combination of any two HLists having the same structure
/// if all of the sub-element types are also Semiups
impl<H: Semigroup, T: HList + Semigroup> Semigroup for HCons<H, T> {
    fn combine(&self, other: &Self) -> Self {
        self.tail
            .combine(&other.tail)
            .prepend(self.head.combine(&other.head))
    }
}

/// Since () + () = (), the same is true for HNil
impl Semigroup for HNil {
    fn combine(&self, _: &Self) -> Self {
        *self
    }
}

/// Return this combined with itself `n` times.
pub fn combine_n<T>(o: &T, times: u32) -> T
where
    T: Semigroup + Clone,
{
    let mut x = o.clone();
    // note: range is non-inclusive in the upper bound
    for _ in 1..times {
        x = o.combine(&x);
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
/// use frunk::semigroup::combine_all_option;
///
/// let v1 = &vec![1, 2, 3];
/// assert_eq!(combine_all_option(v1), Some(6));
///
/// let v2: Vec<i16> = Vec::new(); // empty!
/// assert_eq!(combine_all_option(&v2), None);
/// ```
pub fn combine_all_option<T>(xs: &[T]) -> Option<T>
where
    T: Semigroup + Clone,
{
    xs.first()
        .map(|head| xs[1..].iter().fold(head.clone(), |a, b| a.combine(b)))
}

macro_rules! numeric_semigroup_imps {
  ($($tr:ty),*) => {
    $(
      impl Semigroup for $tr {
        fn combine(&self, other: &Self) -> Self { self + other }
      }
    )*
  }
}

numeric_semigroup_imps!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);

macro_rules! numeric_product_semigroup_imps {
  ($($tr:ty),*) => {
    $(
      impl Semigroup for Product<$tr> {
        fn combine(&self, other: &Self) -> Self {
            let Product(x) = *self;
            let Product(y) = *other;
            Product(x * y)
         }
      }
    )*
  }
}

numeric_product_semigroup_imps!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);

impl<T> Semigroup for Option<T>
where
    T: Semigroup + Clone,
{
    fn combine(&self, other: &Self) -> Self {
        match (self, other) {
            (Some(ref v), Some(ref v_other)) => Some(v.combine(v_other)),
            (Some(_), _) => self.clone(),
            _ => other.clone(),
        }
    }
}

#[cfg(feature = "std")]
impl<T: Semigroup> Semigroup for Box<T> {
    fn combine(&self, other: &Self) -> Self {
        Box::new(self.deref().combine(other.deref()))
    }
}

#[cfg(feature = "std")]
impl Semigroup for String {
    fn combine(&self, other: &Self) -> Self {
        let mut cloned = self.clone();
        cloned.push_str(other);
        cloned
    }
}

#[cfg(feature = "std")]
impl<T: Clone> Semigroup for Vec<T> {
    fn combine(&self, other: &Self) -> Self {
        let mut v = self.clone();
        v.extend_from_slice(other);
        v
    }
}

impl<T> Semigroup for Cell<T>
where
    T: Semigroup + Copy,
{
    fn combine(&self, other: &Self) -> Self {
        Cell::new(self.get().combine(&(other.get())))
    }
}

impl<T: Semigroup> Semigroup for RefCell<T> {
    fn combine(&self, other: &Self) -> Self {
        let self_b = self.borrow();
        let other_b = other.borrow();
        RefCell::new(self_b.deref().combine(other_b.deref()))
    }
}

#[cfg(feature = "std")]
impl<T> Semigroup for HashSet<T>
where
    T: Eq + Hash + Clone,
{
    fn combine(&self, other: &Self) -> Self {
        self.union(other).cloned().collect()
    }
}

#[cfg(feature = "std")]
impl<K, V> Semigroup for HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Semigroup + Clone,
{
    fn combine(&self, other: &Self) -> Self {
        let mut h: HashMap<K, V> = self.clone();
        for (k, v) in other {
            let k_clone = k.clone();
            match h.entry(k_clone) {
                Entry::Occupied(o) => {
                    let existing = o.into_mut();
                    let comb = existing.combine(v);
                    *existing = comb;
                }
                Entry::Vacant(o) => {
                    o.insert(v.clone());
                }
            }
        }
        h
    }
}

impl<T> Semigroup for Max<T>
where
    T: Ord + Clone,
{
    fn combine(&self, other: &Self) -> Self {
        let x = self.0.clone();
        let y = other.0.clone();
        match x.cmp(&y) {
            Ordering::Less => Max(y),
            _ => Max(x),
        }
    }
}

impl<T> Semigroup for Min<T>
where
    T: Ord + Clone,
{
    fn combine(&self, other: &Self) -> Self {
        let x = self.0.clone();
        let y = other.0.clone();
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
            impl Semigroup for All<$tr> {
                fn combine(&self, other: &Self) -> Self {
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
            impl Semigroup for Any<$tr> {
                fn combine(&self, other: &Self) -> Self {
                    let x = self.0;
                    let y = other.0;
                    Any(x.bitor(y))
                }
            }
        )*
    }
}

simple_any_impls! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

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
        impl<$typ: Semigroup, $( $ntyp: Semigroup),*> Semigroup for ($typ, $( $ntyp ),*) {
            fn combine(&self, other: &Self) -> Self {
                (self.$idx.combine(&other.$idx), $(self.$nidx.combine(&other.$nidx), )*)
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
    use frunk_core::hlist;

    macro_rules! semigroup_tests {
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

    semigroup_tests! {
        test_i8, 1.combine(&2) => 3, i8
        test_product_i8, Product(1).combine(&Product(2)) => Product(2), Product<i8>
        test_i16, 1.combine(&2) => 3, i16
        test_i32, 1.combine(&2) => 3, i32
        test_u8, 1.combine(&2) => 3, u8
        test_u16, 1.combine(&2) => 3, u16
        test_u32, 1.combine(&2) => 3, u32
        test_usize, 1.combine(&2) => 3, usize
        test_isize, 1.combine(&2) => 3, isize
        test_f32, 1f32.combine(&2f32) => 3f32, f32
        test_f64, 1f64.combine(&2f64) => 3f64, f64
        test_option_i16, Some(1).combine(&Some(2)) => Some(3), Option<i16>
        test_option_i16_none1, None.combine(&Some(2)) => Some(2), Option<i16>
        test_option_i16_none2, Some(2).combine(&None) => Some(2), Option<i16>
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_string() {
        let v1 = String::from("Hello");
        let v2 = String::from(" world");
        assert_eq!(v1.combine(&v2), "Hello world")
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_vec_i32() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];
        assert_eq!(v1.combine(&v2), vec![1, 2, 3, 4, 5, 6])
    }

    #[test]
    fn test_refcell() {
        let v1 = RefCell::new(1);
        let v2 = RefCell::new(2);
        assert_eq!(v1.combine(&v2), RefCell::new(3))
    }

    #[test]
    #[cfg(feature = "std")]
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
        assert_eq!(v1.combine(&v2), expected)
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_tuple() {
        let t1 = (1, 2.5f32, String::from("hi"), Some(3));
        let t2 = (1, 2.5f32, String::from(" world"), None);

        let expected = (2, 5.0f32, String::from("hi world"), Some(3));

        assert_eq!(t1.combine(&t2), expected)
    }

    #[test]
    fn test_max() {
        assert_eq!(Max(1).combine(&Max(2)), Max(2));

        let v = [Max(1), Max(2), Max(3)];
        assert_eq!(combine_all_option(&v), Some(Max(3)));
    }

    #[test]
    fn test_min() {
        assert_eq!(Min(1).combine(&Min(2)), Min(1));

        let v = [Min(1), Min(2), Min(3)];
        assert_eq!(combine_all_option(&v), Some(Min(1)));
    }

    #[test]
    fn test_all() {
        assert_eq!(All(3).combine(&All(5)), All(1));
        assert_eq!(All(true).combine(&All(false)), All(false));
    }

    #[test]
    fn test_any() {
        assert_eq!(Any(3).combine(&Any(5)), Any(7));
        assert_eq!(Any(true).combine(&Any(false)), Any(true));
    }

    #[test]
    #[cfg(feature = "std")]
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
        assert_eq!(v1.combine(&v2), expected)
    }

    #[test]
    fn test_combine_all_option() {
        let v1 = [1, 2, 3];
        assert_eq!(combine_all_option(&v1), Some(6));
        let v2 = [Some(1), Some(2), Some(3)];
        assert_eq!(combine_all_option(&v2), Some(Some(6)));
    }

    #[test]
    fn test_combine_n() {
        assert_eq!(combine_n(&1, 3), 3);
        assert_eq!(combine_n(&2, 1), 2);
        assert_eq!(combine_n(&Some(2), 4), Some(8));
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_combine_hlist() {
        let h1 = hlist![Some(1), 3.3, 53i64, "hello".to_owned()];
        let h2 = hlist![Some(2), 1.2, 1i64, " world".to_owned()];
        let h3 = hlist![Some(3), 4.5, 54, "hello world".to_owned()];
        assert_eq!(h1.combine(&h2), h3)
    }
}
