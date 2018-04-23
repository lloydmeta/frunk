use kind::Kind;

pub trait Functor<B>: Kind<B> {
    type Item;
    fn fmap<F>(self, f: F) -> <Self as Kind<B>>::Make where F: Fn(Self::Item) -> B;
}

impl<A, B> Functor<B> for Option<A> {
    type Item = A;
    fn fmap<F>(self, f: F) -> Option<B> where F: Fn(A) -> B {
        self.map(f)
    }
}

impl<A, B, E> Functor<B> for Result<A, E> {
    type Item = A;
    fn fmap<F>(self, f: F) -> Result<B, E> where F: Fn(A) -> B {
        self.map(f)
    }
}

impl<A, B> Functor<B> for Vec<A> {
    type Item = A;
    fn fmap<F>(self, f: F) -> Vec<B> where F: Fn(A) -> B{
        self.into_iter().map(f).collect()
    }
}
