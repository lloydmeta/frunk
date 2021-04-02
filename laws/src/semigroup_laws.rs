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
pub fn associativity<A: Semigroup + Eq>(a: A, b: A, c: A) -> bool {
    a.combine(&b).combine(&c) == a.combine(&b.combine(&c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;
    #[cfg(feature = "std")]
    use std::collections::{HashMap, HashSet};
    use wrapper::*;

    #[test]
    #[cfg(feature = "std")]
    fn string_prop() {
        quickcheck(associativity as fn(String, String, String) -> bool)
    }

    #[test]
    #[cfg(feature = "std")]
    fn option_prop() {
        quickcheck(associativity as fn(Option<String>, Option<String>, Option<String>) -> bool)
    }

    #[test]
    #[cfg(feature = "std")]
    fn vec_prop() {
        quickcheck(associativity as fn(Vec<i8>, Vec<i8>, Vec<i8>) -> bool)
    }

    #[test]
    #[cfg(feature = "std")]
    fn hashset_prop() {
        quickcheck(associativity as fn(HashSet<i8>, HashSet<i8>, HashSet<i8>) -> bool)
    }

    #[test]
    #[cfg(feature = "std")]
    fn hashmap_prop() {
        quickcheck(
            associativity
                as fn(HashMap<i8, String>, HashMap<i8, String>, HashMap<i8, String>) -> bool,
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
