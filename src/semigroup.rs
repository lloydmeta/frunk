use std::cell::*;
use std::hash::Hash;
use std::ops::Deref;
use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;

pub trait Semigroup {
    /// Associative operation taking which combines two values.
    fn combine(&self, other: &Self) -> Self;

}

/// Return this combined with itself `n` times.
pub fn combine_n<T>(o: &T, times: u32) -> T where T: Semigroup + Clone {
    let mut x = o.clone();
    // note: range is non-inclusive in the upper bound
    for _ in 1 .. times {
        x = o.combine(&x);
    }
    x
}

/// Given a sequence of `xs`, combine them and return the total
///
/// If the sequence is empty, returns None. Otherwise, returns Some(total).
///
/// ```
/// # use frust::semigroup::*;
///
/// let v1 = &vec![1, 2, 3];
/// assert_eq!(combine_all_option(v1), Some(6));
///
/// let v2: Vec<i16> = Vec::new(); // empty!
/// assert_eq!(combine_all_option(&v2), None);
/// ```
pub fn combine_all_option<T>(xs: &Vec<T>) -> Option<T> where T: Semigroup + Clone {
    match xs.first() {
        Some(ref head) => {
            let tail = xs[1 .. ].to_vec();
            // TODO figure out how to write this as a fold
            let mut x = (*head).clone();
            for i in tail {
                x = x.combine(&i)
            }
            Some(x)
        },
        _ => None
    }
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

impl<T> Semigroup for Option<T>
    where T: Semigroup + Clone
{
    fn combine(&self, other: &Self) -> Self {
        match *self {
            Some(ref v) => {
                match *other {
                    Some(ref v_other) => Some(v.combine(v_other)),
                    _ => self.clone(),
                }
            }
            _ => other.clone(),
        }
    }
}

impl<T: Semigroup> Semigroup for Box<T> {
    fn combine(&self, other: &Self) -> Self {
        Box::new(self.deref().combine(other.deref()))
    }
}

impl Semigroup for String {
    fn combine(&self, other: &Self) -> Self {
        let mut cloned = self.clone();
        cloned.push_str(other);
        cloned
    }
}

impl<T: Clone> Semigroup for Vec<T> {
    fn combine(&self, other: &Self) -> Self {
        let mut v = self.clone();
        v.extend_from_slice(other);
        v
    }
}

impl<T> Semigroup for Cell<T>
    where T: Semigroup + Copy
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

impl<T> Semigroup for HashSet<T>
    where T: Eq + Hash + Clone
{
    fn combine(&self, other: &Self) -> Self {
        let mut h = HashSet::new();
        for i in self {
            h.insert(i.clone());
        }
        for i in other {
            h.insert(i.clone());
        }
        h
    }
}

impl<K, V> Semigroup for HashMap<K, V>
    where K: Eq + Hash + Clone,
          V: Semigroup + Clone
{
    fn combine(&self, other: &Self) -> Self {
        let mut h: HashMap<K, V> = HashMap::new();
        for (k, v) in self {
            h.insert(k.clone(), v.clone());
        }
        for (k, v) in other {
            let k_clone = k.clone();
            match h.entry(k_clone) {
                Entry::Occupied(o) => {
                    let existing = o.into_mut();
                    let comb = existing.combine(&v);
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


#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::*;
    use std::cell::*;

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
    fn test_string() {
        let v1 = String::from("Hello");
        let v2 = String::from(" world");
        assert_eq!(v1.combine(&v2), "Hello world")
    }

    #[test]
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
        let v1 = &vec![1, 2, 3];
        assert_eq!(combine_all_option(v1), Some(6));
        let v2 = &vec![Some(1), Some(2), Some(3)];
        assert_eq!(combine_all_option(v2), Some(Some(6)));
    }

    #[test]
    fn test_combine_n() {
        assert_eq!(combine_n(&1, 3), 3);
        assert_eq!(combine_n(&2, 1), 2);
        assert_eq!(combine_n(&Some(2), 4), Some(8));
    }

}
