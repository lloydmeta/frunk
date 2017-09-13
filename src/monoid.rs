//! Module for holding Monoid typeclass definitions and default implementations
//!
//! A `Monoid` is a Semigroup that has a defined empty/zero value. This allows us to
//! define a `combine_all` method to work on a list of said things:
//!
//! Have you ever wanted to combine 2 Hashmaps such that for a given key, if it exists in both maps,
//! their values are summed in the new map?
//!
//! # Examples
//!
//! ```
//! # use frunk::monoid::*;
//! # use std::collections::*;
//! let vec_of_no_hashmaps: Vec<HashMap<i32, String>> = Vec::new();
//! assert_eq!(combine_all(vec_of_no_hashmaps),
//!                    <HashMap<i32, String> as Monoid>::empty());
//!
//! let mut h1  = HashMap::new();
//! h1.insert(1, "Hello");  // h1 is HashMap( 1 -> "Hello")
//! let mut h2  = HashMap::new();
//! h2.insert(1, " World");
//! h2.insert(2, "Goodbye");  // h2 is HashMap( 1 -> " World", 2 -> "Goodbye")
//! let mut h3 = HashMap::new();
//! h3.insert(3, "Cruel World");
//! let vec_of_hashes = vec![h1, h2, h3];
//!
//! let mut h_expected = HashMap::new();
//! h_expected.insert(1, "Hello World".to_string());
//! h_expected.insert(2, "Goodbye".to_string());
//! h_expected.insert(3, "Cruel World".to_string()); // h_expected is HashMap ( 1 -> "Hello World", 2 -> "Goodbye", 3 -> "Cruel World")
//! assert_eq!(combine_all(vec_of_hashes), h_expected);
//! ```
use super::semigroup::{Semigroup, Product, All, Any};
use std::collections::*;
use std::hash::Hash;
use std::borrow::Borrow;

/// A Monoid is a Semigroup that has an empty/ zero value
pub trait Monoid<Out = Self, RHS = Self>: Semigroup<Out, RHS> + Sized {
    /// For a given Monoid, returns its empty/zero value
    ///
    /// # Examples
    ///
    /// ```
    /// # use frunk::monoid::*;
    ///
    /// assert_eq!(<i16 as Monoid>::empty(), 0);
    /// ```
    fn empty() -> Out;
}

/// Return this combined with itself `n` times.
///
/// # Examples
///
/// ```
/// # use frunk::monoid::*;
///
/// assert_eq!(combine_n(Some(2), 4), Some(8));
/// ```
pub fn combine_n<Out, T>(o: T, times: usize) -> Out
where
    T: Monoid<Out, T> + Semigroup<Out, T> + Clone,
    Out: Semigroup<Out, T> + From<T>,
{
    if times == 0 {
        <T as Monoid<Out, T>>::empty()
    } else {
        super::semigroup::combine_n(o, times)
    }
}

/// Given a sequence of `xs`, combine them and return the total
///
/// # Examples
///
/// ```
/// # use frunk::monoid::*;
///
/// assert_eq!(combine_all(vec![Some(1), Some(3)]), Some(4));
///
/// let empty_vec_opt_int:  Vec<Option<i32>> = Vec::new();
/// assert_eq!(combine_all(empty_vec_opt_int), None);
///
/// let vec_of_some_strings = vec![Some("Hello"), Some(" World")];
/// assert_eq!(combine_all(vec_of_some_strings), Some("Hello World".to_string()));
/// ```
pub fn combine_all<T, Out>(xs: Vec<T>) -> Out
where
    T: Monoid<Out>,
    Out: Monoid<Out, T>,
{
    xs.into_iter().fold(
        <T as Monoid<Out>>::empty(),
        |acc, next| acc.combine(next),
    )
}

impl<T, Out, RHS> Monoid<Option<Out>, RHS> for Option<T>
where
    Option<T>: Semigroup<Option<Out>, RHS>,
{
    fn empty() -> Option<Out> {
        None
    }
}

impl<'a, Str: Borrow<str>> Monoid<String, Str> for String {
    fn empty() -> Self {
        String::new()
    }
}

impl<'a, Str: Borrow<str>> Monoid<String, Str> for &'a str {
    fn empty() -> String {
        String::new()
    }
}

impl<T> Monoid for Vec<T>
where
    Vec<T>: Semigroup,
{
    fn empty() -> Vec<T> {
        Vec::new()
    }
}

impl<T> Monoid for HashSet<T>
where
    T: Hash + Eq,
{
    fn empty() -> Self {
        HashSet::new()
    }
}

impl<K, V, Out, RHS> Monoid<HashMap<K, Out>, HashMap<K, RHS>> for HashMap<K, V>
where
    HashMap<K, V>: Semigroup<
        HashMap<K, Out>,
        HashMap<K, RHS>,
    >,
    K: Eq + Hash,
{
    fn empty() -> HashMap<K, Out> {
        HashMap::new()
    }
}

impl Monoid for All<bool> {
    fn empty() -> Self {
        All(true)
    }
}


impl Monoid for Any<bool> {
    fn empty() -> Self {
        Any(false)
    }
}

macro_rules! numeric_all_impls {
    ($($tr:ty)*) => {
      $(
        impl Monoid for All<$tr> {
            fn empty() -> Self { All(!0) }
        }
      )*
    }
}

numeric_all_impls! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

macro_rules! numeric_any_impls {
    ($($tr:ty)*) => {
      $(
        impl Monoid for Any<$tr> {
            fn empty() -> Self { Any(0) }
        }
      )*
    }
}

numeric_any_impls! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

macro_rules! numeric_monoid_imps {
  ($($zero: expr; $tr:ty),*) => {
    $(
      impl Monoid for $tr {
        fn empty() -> Self { $zero }
      }
    )*
  }
}

numeric_monoid_imps! {
    0; i8,
    0; i16,
    0; i32,
    0; i64,
    0; u8,
    0; u16,
    0; u32,
    0; u64,
    0; isize,
    0; usize,
    0f32; f32,
    0f64; f64
}

macro_rules! numeric_product_monoid_imps {
  ($($one: expr; $tr:ty),*) => {
    $(
      impl Monoid for Product<$tr> {
        fn empty() -> Self { Product($one) }
      }
    )*
  }
}

numeric_product_monoid_imps! {
    1; i8,
    1; i16,
    1; i32,
    1; i64,
    1; u8,
    1; u16,
    1; u32,
    1; u64,
    1; isize,
    1; usize,
    1f32; f32,
    1f64; f64
}

macro_rules! monoid_tuple_impls {
    ([($idx:tt, $typ:ident, $typOut:ident, $typRHS:ident); $( ($nidx:tt, $ntyp:ident, $ntypOut:ident, $ntypRHS: ident); )*]) => {
        impl<$typ, $typOut, $typRHS, $( $ntyp, $ntypOut, $ntypRHS),*> Monoid<($typOut, $( $ntypOut), *), ($typRHS, $( $ntypRHS), *)> for ($typ, $( $ntyp ),*)
        where
            $typ: Monoid<$typOut, $typRHS>,
            $( $ntyp: Monoid<$ntypOut, $ntypRHS>),* {
            fn empty() -> ($typOut, $( $ntypOut), *) {
              (<$typ as Monoid<$typOut, $typRHS>>::empty(), $(<$ntyp as Monoid<$ntypOut, $ntypRHS>>::empty(), )*)
            }
        }
    }
}

internal_tuple_impl_builder![build monoid_tuple_impls];

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::semigroup::{Product, All, Any};

    #[test]
    fn test_combine_n() {
        assert_eq!(combine_n(1, 0), 0);
        assert_eq!(combine_n(2, 1), 2);
        assert_eq!(combine_n(Some(2), 0), None);
        assert_eq!(combine_n(Some(2), 4), Some(8));
    }

    #[test]
    fn test_combine_all_basic() {
        assert_eq!(combine_all(vec![1, 2, 3]), 6);

        let empty_vec_int: Vec<i32> = Vec::new();
        assert_eq!(combine_all(empty_vec_int), 0);

        let empty_vec_opt_int: Vec<Option<i32>> = Vec::new();
        assert_eq!(combine_all(empty_vec_opt_int), None);

        let vec_of_some_strings = vec![Some("Hello".to_owned()), Some(" World".to_owned())];
        assert_eq!(
            combine_all(vec_of_some_strings),
            Some("Hello World".to_owned())
        );
    }

    #[test]
    fn test_combine_all_hashset() {
        let vec_of_no_hashes: Vec<HashSet<i32>> = Vec::new();
        assert_eq!(
            combine_all(vec_of_no_hashes),
            <HashSet<i32> as Monoid>::empty()
        );

        let mut h1 = HashSet::new();
        h1.insert(1);
        let mut h2 = HashSet::new();
        h2.insert(2);
        let mut h3 = HashSet::new();
        h3.insert(3);
        let vec_of_hashes = vec![h1, h2, h3];
        let mut h_expected = HashSet::new();
        h_expected.insert(1);
        h_expected.insert(2);
        h_expected.insert(3);
        assert_eq!(combine_all(vec_of_hashes), h_expected);
    }

    #[test]
    fn test_combine_all_hashmap() {
        let vec_of_no_hashmaps: Vec<HashMap<i32, &str>> = Vec::new();
        assert_eq!(
            combine_all(vec_of_no_hashmaps),
            <HashMap<i32, String> as Monoid>::empty()
        );

        let mut h1 = HashMap::new();
        h1.insert(1, String::from("Hello")); // h1 is HashMap( 1 -> "Hello")
        let mut h2 = HashMap::new();
        h2.insert(1, String::from(" World"));
        h2.insert(2, String::from("Goodbye")); // h2 is HashMap( 1 -> " World", 2 -> "Goodbye")
        let mut h3 = HashMap::new();
        h3.insert(3, String::from("Cruel World")); // h3 is HashMap( 3 -> "Cruel World")
        let vec_of_hashes = vec![h1, h2, h3];

        let mut h_expected = HashMap::new();
        h_expected.insert(1, String::from("Hello World"));
        h_expected.insert(2, String::from("Goodbye"));
        h_expected.insert(3, String::from("Cruel World")); // h_expected is HashMap ( 1 -> "Hello World", 2 -> "Goodbye", 3 -> "Cruel World")
        assert_eq!(combine_all(vec_of_hashes), h_expected);
    }

    #[test]
    fn test_combine_all_all() {
        let v1: Vec<All<i32>> = Vec::new();
        assert_eq!(combine_all(v1), All(!0));
        assert_eq!(combine_all(vec![All(3), All(7)]), All(3));

        let v2: Vec<All<bool>> = Vec::new();
        assert_eq!(combine_all(v2), All(true));
        assert_eq!(combine_all(vec![All(false), All(false)]), All(false));
        assert_eq!(combine_all(vec![All(true), All(true)]), All(true));
    }

    #[test]
    fn test_combine_all_any() {
        let v1: Vec<Any<i32>> = Vec::new();
        assert_eq!(combine_all(v1), Any(0));
        assert_eq!(combine_all(vec![Any(3), Any(8)]), Any(11));

        let v2: Vec<Any<bool>> = Vec::new();
        assert_eq!(combine_all(v2), Any(false));
        assert_eq!(combine_all(vec![Any(false), Any(false)]), Any(false));
        assert_eq!(combine_all(vec![Any(true), Any(false)]), Any(true));
    }

    #[test]
    fn test_combine_all_tuple() {
        let t1 = (1, 2.5f32, String::from("hi"), Some(3));
        let t2 = (1, 2.5f32, String::from(" world"), None);
        let t3 = (1, 2.5f32, String::from(", goodbye"), Some(10));
        let tuples = vec![t1, t2, t3];

        let expected = (3, 7.5f32, String::from("hi world, goodbye"), Some(13));
        assert_eq!(combine_all(tuples), expected)
    }

    #[test]
    fn test_combine_all_product() {
        let v = vec![Product(2), Product(3), Product(4)];
        assert_eq!(combine_all(v), Product(24))
    }

}
