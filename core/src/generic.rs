pub trait Generic {
    type Repr;

    fn into_generic(self) -> Self::Repr;

    fn from_generic(r: Self::Repr) -> Self;
}
