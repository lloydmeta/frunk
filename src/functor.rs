pub trait Functor<'a, A, B, F>
    where A: 'a,
          F: Fn(&'a A) -> B
{
    type Output;
    fn map(&'a self, f: F) -> Self::Output;
}

/// Function to map a function over a given functor
pub fn map<'a, A, B, X, F>(x: &'a X, f: F) -> X::Output
    where F: Fn(&'a A) -> B,
          X: Functor<'a, A, B, F>
{
    x.map(f)
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Option<A>
    where A: 'a,
          F: Fn(&'a A) -> B
{
    type Output = Option<B>;
    fn map(&'a self, f: F) -> Self::Output {
        self.as_ref().map(|x| f(&x))
    }
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Vec<A>
    where A: 'a,
          F: Fn(&'a A) -> B
{
    type Output = Vec<B>;
    fn map(&'a self, f: F) -> Self::Output {
        self.iter().map(|x| f(&x)).collect()
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_option() {
        let maybe_i = Some(1);
        assert_eq!(map(&maybe_i, |x| x + 1), Some(2));
    }

    #[test]
    fn test_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(map(&v, |x| x + 1), vec![2, 3, 4]);
    }

}
