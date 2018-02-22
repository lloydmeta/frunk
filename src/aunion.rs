//! An anonymous and extensible union.

use std::marker::PhantomData;

/// A trait for anonymous unions.
pub trait AUnion<T, U, Idx: Nat> {
    /// Places a value into the union.
    fn inject(t: T) -> Self;

    /// Attempts to get a value from the union.
    fn select(self) -> Result<T, U>;
}

/// The nil case of an anonymous union. Equivalent to the void/false/never/`!`
/// type; that is, an enum with no branches (and thus no values).
///
/// Note that this does not impl `AUnion` at all -- you can't `inject` to or
/// `select` from an empty union!
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AUnionNil {}

impl AUnionNil {
    /// Consumes this value; since there are no values of this type, it's
    /// impossible to execute this function (without `unsafe`).
    pub fn unreachable(self) -> ! {
        match self {}
    }
}

/// The cons case of an anonymous union.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AUnionCons<Hd, Tl> {
    /// The head value is present.
    Hd(Hd),

    /// The head value is not present, so a value must be present in the tail.
    Tl(Tl),
}

impl<Hd, Tl> AUnion<Hd, Tl, NZero> for AUnionCons<Hd, Tl> {
    fn inject(h: Hd) -> AUnionCons<Hd, Tl> {
        AUnionCons::Hd(h)
    }

    fn select(self) -> Result<Hd, Tl> {
        match self {
            AUnionCons::Hd(h) => Ok(h),
            AUnionCons::Tl(t) => Err(t),
        }
    }
}

impl<Hd, Tl, T, U, N: Nat> AUnion<T, AUnionCons<Hd, U>, NSucc<N>> for AUnionCons<Hd, Tl>
where
    Tl: AUnion<T, U, N>,
{
    fn inject(t: T) -> AUnionCons<Hd, Tl> {
        AUnionCons::Tl(AUnion::inject(t))
    }

    fn select(self) -> Result<T, AUnionCons<Hd, U>> {
        match self {
            AUnionCons::Hd(h) => Err(AUnionCons::Hd(h)),
            AUnionCons::Tl(t) => t.select().map_err(AUnionCons::Tl),
        }
    }
}

/// A type-level natural number.
pub trait Nat {
    /// The value of the natural number at the value level.
    const VALUE: usize;
}

/// The type-level representation of zero.
pub struct NZero;

impl Nat for NZero {
    const VALUE: usize = 0;
}

/// The type-level representation of `n+1` for some `n`.
pub struct NSucc<T: Nat>(PhantomData<T>);

impl<T: Nat> Nat for NSucc<T> {
    const VALUE: usize = 1 + T::VALUE;
}

/// A trait for the length of a structure.
pub trait Length<N: Nat> {}
