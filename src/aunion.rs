//! An anonymous and extensible union.

use super::coproduct::{CoprodInjector, Coproduct};
use super::hlist::{Here, There};

/// A trait for anonymous unions.
pub trait AUnion<T, U, Idx>: CoprodInjector<T, Idx> {
    /// Places a value into the union.
    fn inject(t: T) -> Self;

    /// Attempts to get a value from the union.
    fn select(self) -> Result<T, U>;
}

impl<Hd, Tl> AUnion<Hd, Tl, Here> for Coproduct<Hd, Tl> {
    fn inject(h: Hd) -> Coproduct<Hd, Tl> {
        Coproduct::Inl(h)
    }

    fn select(self) -> Result<Hd, Tl> {
        match self {
            Coproduct::Inl(h) => Ok(h),
            Coproduct::Inr(t) => Err(t),
        }
    }
}

impl<Hd, Tl, T, U, N> AUnion<T, Coproduct<Hd, U>, There<N>> for Coproduct<Hd, Tl>
where
    Tl: AUnion<T, U, N>,
{
    fn inject(t: T) -> Coproduct<Hd, Tl> {
        Coproduct::Inr(AUnion::inject(t))
    }

    fn select(self) -> Result<T, Coproduct<Hd, U>> {
        match self {
            Coproduct::Inl(h) => Err(Coproduct::Inl(h)),
            Coproduct::Inr(t) => t.select().map_err(Coproduct::Inr),
        }
    }
}
