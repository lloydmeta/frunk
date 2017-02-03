pub trait Generic<Repr> {
    fn into(self) -> Repr;

    fn from(r: Repr) -> Self;
}

/// Given a generic representation of an A, returns A
pub fn from_generic<A, Repr>(gen: Repr) -> A where A: Generic<Repr> {
    <A as Generic<Repr>>::from(gen)
}

/// Given an A, returns its generic representation
pub fn into_generic<A, Repr>(a: A) -> Repr where A: Generic<Repr> {
    <A as Generic<Repr>>::into(a)
}