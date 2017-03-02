pub trait LabelledGeneric<Repr> {
    /// Go from something to Repr
    fn into(self) -> Repr;

    /// Go from Repr to something
    fn from(r: Repr) -> Self;
}

/// Given a generic Representation of an A, returns A
pub fn from_labelled_generic<A, Repr>(gen: Repr) -> A
    where A: LabelledGeneric<Repr>
{
    <A as LabelledGeneric<Repr>>::from(gen)
}

/// Given an A, returns its generic Representation
pub fn into_labelled_generic<A, Repr>(a: A) -> Repr
    where A: LabelledGeneric<Repr>
{
    <A as LabelledGeneric<Repr>>::into(a)
}

/// Converts one type into another assuming they have the same generic Representation
pub fn convert_from<A, B, Repr>(a: A) -> B
    where A: LabelledGeneric<Repr>,
          B: LabelledGeneric<Repr>
{
    let repr = <A as LabelledGeneric<Repr>>::into(a);
    <B as LabelledGeneric<Repr>>::from(repr)
}
