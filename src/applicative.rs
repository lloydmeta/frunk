use kind::Kind;
use functor::Functor;

pub trait Applicative<B, F>: Functor<B> + Kind<F> + Sized
    where F: Fn(Self::Item) -> B {
    fn pure_(Self::Item) -> Self;
    fn apply(f: <Self as Kind<F>>::Make, Self) -> <Self as Kind<B>>::Make;
}

impl<A, B, F> Applicative<B, F> for Option<A>
    where F: Fn(A) -> B {
    fn pure_(a: A) -> Option<A> {
        Some(a)
    }

    fn apply(f: Option<F>, to: Option<A>) -> Option<B> {
        match f {
            Some(f) => to.map(f),
            None => None,
        }
    }
}

impl<A: Clone, B, F> Applicative<B, F> for Vec<A>
    where F: Fn(A) -> B {
    fn pure_(a: A) -> Vec<A> {
        vec![a]
    }

    fn apply(f: Vec<F>, to: Vec<A>) -> Vec<B> {
        f.into_iter().flat_map(|f| {
            to.clone().into_iter().map(|to| {
                f(to)
            }).collect::<Vec<B>>()
        }).collect()
    }
}
