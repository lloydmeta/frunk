pub trait Kind<T> {
    type Make;
}

impl<A, B> Kind<B> for Option<A> {
    type Make = Option<B>;
}

impl<A, B, E> Kind<B> for Result<A, E> {
    type Make = Result<B, E>;
}

impl<A, B> Kind<B> for Vec<A> {
    type Make = Vec<B>;
}
