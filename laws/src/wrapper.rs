//! This module holds the Wrapper newtype; used to write
//! instances of typeclasses that we don't define for types we don't
//! own

use frunk::monoid::*;
use frunk::semigroup::*;
use quickcheck::*;

/// The Wrapper NewType. Used for writing implementations of traits
/// that we don't own for type we don't own.
///
/// Avoids the orphan typeclass instances problem in Haskell.
#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Hash)]
pub struct Wrapper<A>(A);

impl<A: Arbitrary + Ord + Clone> Arbitrary for Wrapper<Max<A>> {
    fn arbitrary(g: &mut Gen) -> Self {
        Wrapper(Max(Arbitrary::arbitrary(g)))
    }
}

impl<A: Arbitrary + Ord + Clone> Arbitrary for Wrapper<Min<A>> {
    fn arbitrary(g: &mut Gen) -> Self {
        Wrapper(Min(Arbitrary::arbitrary(g)))
    }
}

impl<A: Arbitrary> Arbitrary for Wrapper<All<A>> {
    fn arbitrary(g: &mut Gen) -> Self {
        Wrapper(All(Arbitrary::arbitrary(g)))
    }
}

impl<A: Arbitrary> Arbitrary for Wrapper<Any<A>> {
    fn arbitrary(g: &mut Gen) -> Self {
        Wrapper(Any(Arbitrary::arbitrary(g)))
    }
}

impl<A: Arbitrary> Arbitrary for Wrapper<Product<A>> {
    fn arbitrary(g: &mut Gen) -> Self {
        Wrapper(Product(Arbitrary::arbitrary(g)))
    }
}

impl<A: Semigroup> Semigroup for Wrapper<A> {
    fn combine(&self, other: &Self) -> Self {
        Wrapper(self.0.combine(&other.0))
    }
}

impl<A: Monoid> Monoid for Wrapper<A> {
    fn empty() -> Self {
        Wrapper(<A as Monoid>::empty())
    }
}
