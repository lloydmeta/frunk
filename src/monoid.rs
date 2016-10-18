use super::semigroup::Semigroup;
use std::collections::*;
use std::hash::Hash;

pub trait Monoid {
    fn empty() -> Self;
}

/// Return this combined with itself `n` times.
pub fn combine_n<T>(o: &T, times: u32) -> T where T: Monoid + Semigroup + Clone {
    if times == 0 {
        <T as Monoid>::empty()
    } else {
        super::semigroup::combine_n(o, times)
    }
}

/// Given a sequence of `xs`, combine them and return the total
///
/// ```
/// # use frust::monoid::*;
///
/// assert_eq!(combine_all(&vec![Some(1), Some(3)]), Some(4));
///
/// let empty_vec_opt_int:  Vec<Option<i32>> = Vec::new();
/// assert_eq!(combine_all(&empty_vec_opt_int), None);
///
/// let vec_of_some_strings = vec![Some("Hello".to_owned()), Some(" World".to_owned())];
/// assert_eq!(combine_all(&vec_of_some_strings), Some("Hello World".to_owned()));
/// ```
pub fn combine_all<T>(xs: &Vec<T>) -> T where T: Monoid + Semigroup + Clone {
    let mut r = <T as Monoid>::empty();
    for i in xs {
        r = r.combine(i);
    }
    r
}

impl<T> Monoid for Option<T>
    where T: Semigroup + Clone {
    fn empty() -> Self { None }
}

impl Monoid for String {
    fn empty() -> Self { String::new() }
}

impl <T> Monoid for Vec<T> {
    fn empty() -> Self { Vec::new() }
}

impl <T> Monoid for HashSet<T> where T: Hash + Eq {
    fn empty() -> Self { HashSet::new()}
}

impl<K, V> Monoid for HashMap<K, V>
where K: Eq + Hash + Clone,
      V: Semigroup + Clone {
    fn empty() -> Self { HashMap::new()}
}

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


macro_rules! tuple_impls {
    () => {}; // no more

    (($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident), )*) => {
        /*
         * Invoke recursive reversal of list that ends in the macro expansion implementation
         * of the reversed list
        */
        tuple_impls!([($idx, $typ);] $( ($nidx => $ntyp), )*);
        tuple_impls!($( ($nidx => $ntyp), )*); // invoke macro on tail
    };

    /*
     * ([accumulatedList], listToReverse); recursively calls tuple_impls until the list to reverse
     + is empty (see next pattern)
    */
    ([$(($accIdx: tt, $accTyp: ident);)+]  ($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident), )*) => {
      tuple_impls!([($idx, $typ); $(($accIdx, $accTyp); )*] $( ($nidx => $ntyp), ) *);
    };

    // Finally expand into our implementation
    ([($idx:tt, $typ:ident); $( ($nidx:tt, $ntyp:ident); )*]) => {
        impl<$typ: Monoid, $( $ntyp: Monoid),*> Monoid for ($typ, $( $ntyp ),*) {
            fn empty() -> Self {
              (<$typ as Monoid>::empty(), $(<$ntyp as Monoid>::empty(), )*)
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
    use std::collections::*;

    #[test]
    fn test_combine_n() {
        assert_eq!(combine_n(&1, 0), 0);
        assert_eq!(combine_n(&2, 1), 2);
        assert_eq!(combine_n(&Some(2), 0), None);
        assert_eq!(combine_n(&Some(2), 4), Some(8));
    }

    #[test]
    fn test_combine_all() {
        assert_eq!(combine_all(&vec![1,2,3]), 6);

        let empty_vec_int:  Vec<i32> = Vec::new();
        assert_eq!(combine_all(&empty_vec_int), 0);

        let empty_vec_opt_int:  Vec<Option<i32>> = Vec::new();
        assert_eq!(combine_all(&empty_vec_opt_int), None);

        let vec_of_some_strings = vec![Some("Hello".to_owned()), Some(" World".to_owned())];
        assert_eq!(combine_all(&vec_of_some_strings), Some("Hello World".to_owned()));

        let vec_of_no_hashes: Vec<HashSet<i32>> = Vec::new();
        assert_eq!(combine_all(&vec_of_no_hashes), <HashSet<i32> as Monoid>::empty());

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
        assert_eq!(combine_all(&vec_of_hashes), h_expected);
    }

    #[test]
    fn test_tuple() {
        let t1 = (1, 2.5f32, String::from("hi"), Some(3));
        let t2 = (1, 2.5f32, String::from(" world"), None);
        let t3 = (1, 2.5f32, String::from(", goodbye"), Some(10));
        let tuples = vec![t1, t2, t3];

        let expected = (3, 7.5f32, String::from("hi world, goodbye"), Some(13));
        assert_eq!(combine_all(&tuples), expected)
    }

}