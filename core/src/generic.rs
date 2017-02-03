pub trait Generic<Repr> {
    fn into(self) -> Repr;

    fn from(r: Repr) -> Self;
}

/// Given a generic Representation of an A, returns A
pub fn from_generic<A, Repr>(gen: Repr) -> A where A: Generic<Repr> {
    <A as Generic<Repr>>::from(gen)
}

/// Given an A, returns its generic Representation
pub fn into_generic<A, Repr>(a: A) -> Repr where A: Generic<Repr> {
    <A as Generic<Repr>>::into(a)
}

/// Converts one type into another assuming they have the same generic Representation
pub fn convert_from<A, B, Repr>(a: A) -> B where A: Generic<Repr>, B: Generic<Repr> {
    let repr = <A as Generic<Repr>>::into(a);
    <B as Generic<Repr>>::from(repr)
}