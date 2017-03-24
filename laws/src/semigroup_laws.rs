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
    use std::collections::{HashSet, HashMap};

    #[test]
    fn string_prop() {
        quickcheck(associativity as fn(String, String, String) -> bool)
    }

    #[test]
    fn vec_prop() {
        quickcheck(associativity as fn(Vec<i8>, Vec<i8>, Vec<i8>) -> bool)
    }

    #[test]
    fn hashset_prop() {
        quickcheck(associativity as fn(HashSet<i8>, HashSet<i8>, HashSet<i8>) -> bool)
    }

    #[test]
    fn hashmap_prop() {
        quickcheck(associativity as
                   fn(HashMap<i8, String>,
                      HashMap<i8, String>,
                      HashMap<i8, String>)
                      -> bool)
    }



}
