//! Holds declarations for some variatiosn of F
//!
//! One of the goals of this is to allow for both
//! by reference and by value mapping based purely on
//! the kind of argument taken by the function argument

use kind::*;

pub trait Functor<A, B>
    where Self: Kind<A>,
          Self: Kind<B>
{
    fn map<F>(self, f: F) -> <Self as Kind<B>>::Repr
        where F: FnOnce(A) -> B;
}


impl <A, B> Functor<A, B> for Option<A> where Option<A>: Kind<B>{

    fn map<F>(self, f: F) -> <Self as Kind<B>>::Repr
        where F: FnOnce(A) -> B {
            match self {
                Some(v) => Some(f(v)),
                None => None
            }
        }
}