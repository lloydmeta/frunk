use super::hlist::*;
use std::ops::Add;

#[derive(PartialEq, Eq, Debug)]
pub enum Validation<T, E>
    where T: HList
{
    Valid(T),
    Err(Vec<E>),
}

pub fn combine<T1, T2, E>(r1: Result<T1, E>,
                          r2: Result<T2, E>)
                          -> Validation<HCons<T1, HCons<T2, HNil>>, E> {
    match (r1, r2) {
        (Result::Err(e1), Result::Err(e2)) => Validation::Err(vec![e1, e2]),
        (Result::Err(e1), _) => Validation::Err(vec![e1]),
        (_, Result::Err(e2)) => Validation::Err(vec![e2]),
        (Result::Ok(v1), Result::Ok(v2)) => Validation::Valid(hlist!(v1, v2)),
    }
}

impl<T, E> Validation<T, E>
    where T: HList
{
    pub fn combine<T2>(self,
                       other: Result<T2, E>)
                       -> Validation<<T as Add<HCons<T2, HNil>>>::Output, E>
        where T: Add<HCons<T2, HNil>>,
              <T as Add<HCons<T2, HNil>>>::Output: HList
    {
        match (self, other) {
            (Validation::Err(mut errs), Result::Err(e)) => {
                errs.push(e);
                Validation::Err(errs)
            }
            (Validation::Err(errs), _) => Validation::Err(errs),
            (_, Result::Err(e)) => Validation::Err(vec![e]),
            (Validation::Valid(h1), Result::Ok(v2)) => Validation::Valid(h1 + hlist!(v2)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::hlist::*;
    use super::*;

    #[test]
    fn test_combining_ok_results() {
        let r1: Result<String, String> = Result::Ok(String::from("hello"));
        let r2: Result<i32, String> = Result::Ok(1);
        let v = super::combine(r1, r2);
        assert_eq!(v, Validation::Valid(hlist!(String::from("hello"), 1)))
    }

    #[test]
    fn test_combining_validation_with_results() {
        let r1: Result<String, String> = Result::Ok(String::from("hello"));
        let r2: Result<i32, String> = Result::Ok(1);
        let r3: Result<i32, String> = Result::Ok(3);
        let comb = combine(r1, r2).combine(r3);
        assert_eq!(comb, Validation::Valid(hlist!(String::from("hello"), 1, 3)))
    }
}
