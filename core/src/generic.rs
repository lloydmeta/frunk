pub trait Generic<Repr> {
    fn into(self) -> Repr;

    fn from(r: Repr) -> Self;
}

pub fn from_generic<A, Gen>(gen: Gen) -> A where A: Generic<Gen> {
    <A as Generic<Gen>>::from(gen)
}

pub fn into_generic<A, Gen>(a: A) -> Gen where A: Generic<Gen> {
    <A as Generic<Gen>>::into(a)
}