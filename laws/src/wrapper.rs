//! This module holds the Wrapper newtype; used to write
//! instances of typeclasses that we don't define for types we don't
//! own

use frunk::semigroup::*;
use quickcheck::*;

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone)]
pub struct Wrapper<A>(A);

impl<A: Arbitrary + Ord + Clone> Arbitrary for Wrapper<Max<A>> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Wrapper(Max(Arbitrary::arbitrary(g)))
    }
}

impl<A: Arbitrary + Ord + Clone> Arbitrary for Wrapper<Min<A>> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Wrapper(Min(Arbitrary::arbitrary(g)))
    }
}

impl<A: Arbitrary> Arbitrary for Wrapper<All<A>> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Wrapper(All(Arbitrary::arbitrary(g)))
    }
}

impl<A: Arbitrary> Arbitrary for Wrapper<Any<A>> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Wrapper(Any(Arbitrary::arbitrary(g)))
    }
}

impl<A: Arbitrary> Arbitrary for Wrapper<Product<A>> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Wrapper(Product(Arbitrary::arbitrary(g)))
    }
}

impl<A: Semigroup> Semigroup for Wrapper<A> {
    fn combine(&self, other: &Self) -> Self {
        Wrapper(self.0.combine(&other.0))
    }
}
