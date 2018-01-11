//! Module that holds laws for Monoid implementations
//!
//! Note that you should use the semigroup_laws module to get the associative
//! law test.
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
//! use frunk_laws::monoid_laws::*;
//! quickcheck(left_identity::<String, String, String> as fn(String) -> bool);
//! quickcheck(right_identity::<String, String, String> as fn(String) -> bool);
//! # }
//! ```
use frunk::monoid::*;

/// Left identity law
///
///   mempty <> x = x
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
/// use frunk_laws::monoid_laws::*;
/// quickcheck(left_identity::<String, String, String> as fn(String) -> bool);
/// # }
/// ```
pub fn left_identity<A, Out, RHS>(a: A) -> bool
where
    A: Monoid<Out, RHS> + Clone,
    Out: Monoid<Out, RHS> + From<A> + Eq,
    RHS: From<A>,
{
    <A as Monoid<Out, RHS>>::empty().combine(RHS::from(a.clone())) == Out::from(a)
}

/// Right identity law
///
///   x <> mempty = x
/// # Examples
///
/// ```
/// # extern crate quickcheck;
/// # extern crate frunk_laws;
/// # extern crate frunk;
/// # use quickcheck::quickcheck;
/// # use frunk::semigroup::*;
/// # fn main() {
/// use frunk_laws::monoid_laws::*;
/// quickcheck(right_identity::<String, String, String> as fn(String) -> bool);
/// # }
/// ```
pub fn right_identity<A, Out, RHS>(a: A) -> bool
where
    A: Monoid<Out, RHS> + Clone,
    Out: Monoid<Out, RHS> + From<A> + Eq,
    RHS: From<Out>,
{
    a.clone().combine(
        RHS::from(<A as Monoid<Out, RHS>>::empty()),
    ) == Out::from(a)
}



#[cfg(test)]
mod tests {
    use super::*;
    use wrapper::*;
    use frunk::semigroup::*;
    use quickcheck::quickcheck;
    use std::collections::{HashSet, HashMap};

    #[test]
    fn string_id_prop() {
        quickcheck(
            left_identity::<String, String, String> as fn(String) -> bool,
        );
        quickcheck(
            right_identity::<String, String, String> as fn(String) -> bool,
        );
    }

    #[test]
    fn option_id_prop() {
        quickcheck(
            left_identity::<Option<String>, Option<String>, Option<String>> as
                fn(Option<String>) -> bool,
        );
        quickcheck(
            right_identity::<Option<String>, Option<String>, Option<String>> as
                fn(Option<String>) -> bool,
        );
    }

    #[test]
    fn vec_id_prop() {
        quickcheck(left_identity as fn(Vec<String>) -> bool);
        quickcheck(right_identity as fn(Vec<String>) -> bool);
    }

    #[test]
    fn hashset_id_prop() {
        quickcheck(left_identity as fn(HashSet<i32>) -> bool);
        quickcheck(right_identity as fn(HashSet<i32>) -> bool);
    }

    #[test]
    fn hashmap_id_prop() {
        quickcheck(left_identity as fn(HashMap<i32, String>) -> bool);
        quickcheck(right_identity as fn(HashMap<i32, String>) -> bool);
    }

    #[test]
    fn any_id_prop() {
        quickcheck(left_identity as fn(Wrapper<Any<i32>>) -> bool);
        quickcheck(right_identity as fn(Wrapper<Any<i32>>) -> bool);
    }

    #[test]
    fn all_id_prop() {
        quickcheck(left_identity as fn(Wrapper<All<i32>>) -> bool);
        quickcheck(right_identity as fn(Wrapper<All<i32>>) -> bool);
    }

    macro_rules! numeric_id_props {
      ($($id: ident; $tr:ty,)*) => {

        $(
            #[test]
            fn $id() {
                quickcheck(left_identity::<$tr, $tr, $tr> as fn($tr) -> bool);
                quickcheck(right_identity::<$tr, $tr, $tr> as fn($tr) -> bool);
            }
        )*
      }
    }

    numeric_id_props! {
        i8_id_prop; i8,
        product_i8_id_prop; Wrapper<Product<i8>>,
        u8_id_prop; u8,
        product_u8_id_prop; Wrapper<Product<u8>>,
        i16_id_prop; i16,
        product_i16_id_prop; Wrapper<Product<i16>>,
        u16_id_prop; u16,
        product_u16_id_prop; Wrapper<Product<u16>>,
        i32_id_prop; i32,
        product_i32_id_prop; Wrapper<Product<i32>>,
        u32_id_prop; u32,
        product_u32_id_prop; Wrapper<Product<u32>>,
        i64_id_prop; i64,
        product_i64_id_prop; Wrapper<Product<i64>>,
        u64_id_prop; u64,
        product_u64_id_prop; Wrapper<Product<u64>>,
        isize_id_prop; isize,
        product_isize_id_prop; Wrapper<Product<isize>>,
        usize_id_prop; usize,
        product_usize_id_prop; Wrapper<Product<usize>>,
    }

}
