use hlist::{Here, There};

#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Coproduct<H, T> {
    Inl(H),
    Inr(T),
}

/// Phantom type for signature purposes only
pub enum CNil {}

/// Returns a type signature for a Coproduct of the provided types
///
/// This is a type macro (introduced in Rust 1.13) that makes it easier
/// to write nested type signatures.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk_core; use frunk_core::coproduct::*; fn main() {
/// type I32Bool = Coproduct!(i32, bool);
/// let co1: I32Bool = into_coproduct(3);
/// # }
/// ```
#[macro_export]
macro_rules! Coproduct {
    // Nothing
    () => { $crate::coproduct::CNil };

    // Just a single item
    ($single: ty) => {
        $crate::coproduct::Coproduct<$single, CNil>
    };

    ($first: ty, $( $repeated: ty ), +) => {
        $crate::coproduct::Coproduct<$first, Coproduct!($($repeated), *)>
    };

    // <-- Forward trailing comma variants
    ($single: ty,) => {
        Coproduct![$single]
    };

    ($first: ty, $( $repeated: ty, ) +) => {
        Coproduct![$first, $($repeated),*]
    };
    // Forward trailing comma variants -->
}


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


// <-- For turning something into a Coproduct
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
// For turning something into a Coproduct -->

/// Trait for retrieving a coproduct element by type
pub trait CoproductSelector<S, I> {
    fn get(&self) -> Option<&S>;
}

impl<Head, Tail> CoproductSelector<Head, Here> for Coproduct<Head, Tail> {
    fn get(&self) -> Option<&Head> {
        use self::Coproduct::*;
        match *self {
            Inl(ref thing) => Some(thing),
            _ => None, // Impossible
        }
    }
}


impl<Head, FromTail, Tail, TailIndex> CoproductSelector<FromTail, There<TailIndex>>
    for Coproduct<Head, Tail>
    where Tail: CoproductSelector<FromTail, TailIndex>
{
    fn get(&self) -> Option<&FromTail> {
        use self::Coproduct::*;
        match *self {
            Inr(ref rest) => rest.get(),
            _ => None, // Impossible
        }
    }
}

pub fn get_from<C, T, Indices>(coprod: &C) -> Option<&T>
    where C: CoproductSelector<T, Indices>
{
    <C as CoproductSelector<T, Indices>>::get(coprod)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_coproduct() {
        type I32Bool = Coproduct!(i32, bool);

        let co1: I32Bool = into_coproduct(3);
        let get_from_1a: Option<&i32> = co1.get();
        let get_from_1b: Option<&bool> = co1.get();
        assert_eq!(get_from_1a, Some(&3));
        assert_eq!(get_from_1b, None);


        let co2: I32Bool = into_coproduct(false);
        let get_from_2a: Option<&i32> = co2.get();
        let get_from_2b: Option<&bool> = co2.get();
        assert_eq!(get_from_2a, None);
        assert_eq!(get_from_2b, Some(&false));
    }
}
