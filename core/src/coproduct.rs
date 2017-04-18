use hlist::{Here, There};

#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Coproduct<H, T> {
    Inl(H),
    Inr(T),
}

pub enum CNil {}

pub trait CoproductFold<H, T> {
    fn fold<A, F1, F2>(self, l: F1, r: F2) -> A
        where F1: FnOnce(H) -> A,
              F2: FnOnce(T) -> A;
}

impl<H, T> CoproductFold<H, T> for Coproduct<H, T> {
    fn fold<A, F1, F2>(self, l: F1, r: F2) -> A
        where F1: FnOnce(H) -> A,
              F2: FnOnce(T) -> A
    {
        use self::Coproduct::*;
        match self {
            Inl(v) => l(v),
            Inr(v) => r(v),
        }
    }
}


pub trait IntoCoproduct<InsertType, Index> {
    fn into(to_insert: InsertType) -> Self;
}


impl<I, Tail> IntoCoproduct<I, Here> for Coproduct<I, Tail> {
    fn into(to_insert: I) -> Self {
        Coproduct::Inl(to_insert)
    }
}

impl<Head, I, Tail, TailIndex> IntoCoproduct<I, There<TailIndex>> for Coproduct<Head, Tail>
    where Tail: IntoCoproduct<I, TailIndex>
{
    fn into(to_insert: I) -> Self {
        let tail_inserted = <Tail as IntoCoproduct<I, TailIndex>>::into(to_insert);
        Coproduct::Inr(tail_inserted)
    }
}

pub fn into_coproduct<C, I, Index>(to_into: I) -> C
    where C: IntoCoproduct<I, Index>
{
    <C as IntoCoproduct<I, Index>>::into(to_into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Coproduct::*;

    #[test]
    fn test_into_coproduct() {
        type Thing = Coproduct<i32, Coproduct<bool, CNil>>;
        let co1: Thing = into_coproduct(3);
        let co2: Thing = into_coproduct(false);
    }
}
