//! Module that holds laws for Semigroup implementations
//!
//! # Examples
//!
//! ```
//! # extern crate quickcheck;
//! # extern crate frunk_laws;
//! # extern crate frunk;
//! # use quickcheck::quickcheck;
//! # use frunk::semigroup::*;
//! # fn main() {
//! use frunk_laws::semigroup_laws::*;
//! quickcheck(associativity as fn(Vec<i8>, Vec<i8>, Vec<i8>) -> bool)
//! # }
//! ```

use frunk::semigroup::*;

/// Function for checking adherence to the associativity law
///
///  (x <> y) <> z = x <> (y <> z)
///
/// # Examples
///
/// ```
/// # extern crate quickcheck;
/// # extern crate frunk_laws;
/// # extern crate frunk;
/// # use quickcheck::quickcheck;
/// # use frunk::semigroup::*;
/// # fn main() {
/// use frunk_laws::semigroup_laws::*;
/// quickcheck(associativity as fn(Vec<i8>, Vec<i8>, Vec<i8>) -> bool)
/// # }
/// ```
pub fn associativity<A, Out, Rhs>(a: A, b: A, c: A) -> bool
where
    A: Semigroup<Out, Rhs> + Clone,
    Rhs: From<A> + From<Out>,
    Out: From<A> + Semigroup<Out, Rhs> + Eq,
{
    a.clone().combine(Rhs::from(b.clone())).combine(
        Rhs::from(c.clone()),
    ) == a.combine(Rhs::from(b.combine(Rhs::from(c))))
}


#[cfg(test)]
mod tests {
    use super::*;
    use wrapper::*;
    use quickcheck::quickcheck;
    use std::collections::{HashSet, HashMap};

    #[test]
    fn string_prop() {
        quickcheck(
            associativity::<String, String, String> as fn(String, String, String) -> bool,
        )
    }

    #[test]
    fn option_prop() {
        quickcheck(
            associativity::<Option<String>, Option<String>, Option<String>> as
                fn(Option<String>, Option<String>, Option<String>) -> bool,
        )
    }

    #[test]
    fn vec_prop() {
        quickcheck(associativity as fn(Vec<i8>, Vec<i8>, Vec<i8>) -> bool)
    }

    #[test]
    fn hashset_prop() {
        quickcheck(
            associativity as fn(HashSet<i8>, HashSet<i8>, HashSet<i8>) -> bool,
        )
    }

    #[test]
    fn hashmap_prop() {
        quickcheck(
            associativity as
                fn(HashMap<i8, String>,
                   HashMap<i8, String>,
                   HashMap<i8, String>)
                   -> bool,
        )
    }

    #[test]
    fn max_prop() {
        quickcheck(
            associativity as fn(Wrapper<Max<i8>>, Wrapper<Max<i8>>, Wrapper<Max<i8>>) -> bool,
        )
    }

    #[test]
    fn min_prop() {
        quickcheck(
            associativity as fn(Wrapper<Min<i8>>, Wrapper<Min<i8>>, Wrapper<Min<i8>>) -> bool,
        )
    }

    #[test]
    fn any_prop() {
        quickcheck(
            associativity as fn(Wrapper<Any<bool>>, Wrapper<Any<bool>>, Wrapper<Any<bool>>) -> bool,
        )
    }

    #[test]
    fn all_prop() {
        quickcheck(
            associativity as fn(Wrapper<All<bool>>, Wrapper<All<bool>>, Wrapper<All<bool>>) -> bool,
        )
    }



}
