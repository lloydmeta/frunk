use super::hlist::*;
use std::ops::Add;

#[derive(PartialEq, Eq, Debug)]
pub enum Validated<T, E>
    where T: HList
{
    Ok(T),
    Err(Vec<E>),
}

impl<T, E> Validated<T, E>
    where T: HList
{
    /// Returns true if this validation is Ok, false otherwise
    ///
    /// ```
    /// # use frust::validated::*;
    ///
    /// let r1: Result<String, String> = Result::Ok(String::from("hello"));
    /// let v = r1.into_validated();
    /// assert!(v.is_ok());
    /// ```
    pub fn is_ok(&self) -> bool {
        match *self {
            Validated::Ok(_) => true,
            _ => false,
        }
    }

    /// Returns true if this validation is Err, false otherwise
    ///
    /// ```
    /// # use frust::validated::*;
    ///
    /// let r1: Result<String, i32> = Result::Err(32);
    /// let v = r1.into_validated();
    /// assert!(v.is_err());
    /// ```
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

pub trait ToValidated<T, E> {
    fn into_validated(self) -> Validated<HCons<T, HNil>, E>;
}

impl<T, E> ToValidated<T, E> for Result<T, E> {

    /// Consumes the current Result into a Validated so that we can begin chaining
    ///
    /// ```
    /// # use frust::validated::*;
    ///
    /// let r1: Result<String, i32> = Result::Err(32);
    /// let v = r1.into_validated();
    /// assert!(v.is_err());
    /// ```
    fn into_validated(self) -> Validated<HCons<T, HNil>, E> {
        match self {
            Result::Err(e) => Validated::Err(vec![e]),
            Result::Ok(v) => Validated::Ok(hlist![v]),
        }
    }
}

impl<T, E> Validated<T, E>
    where T: HList
{
    /// Combines the current Validated with a Result, returning a new Validated
    ///
    /// ```
    /// # #[macro_use] extern crate frust; use frust::hlist::*; use frust::validated::*; fn main() {
    ///
    /// let r1: Result<String, String> = Result::Ok(String::from("hello"));
    /// let r2: Result<i32, String> = Result::Ok(1);
    /// let v = r1.into_validated()
    ///            .combine(r2);
    /// assert_eq!(v, Validated::Ok(hlist!(String::from("hello"), 1)))
    /// # }
    /// ```
    ///
    ///
    pub fn combine<T2>(self,
                       other: Result<T2, E>)
                       -> Validated<<T as Add<HCons<T2, HNil>>>::Output, E>
        where T: Add<HCons<T2, HNil>>,
              <T as Add<HCons<T2, HNil>>>::Output: HList
    {
        match (self, other) {
            (Validated::Err(mut errs), Result::Err(e)) => {
                errs.push(e);
                Validated::Err(errs)
            }
            (Validated::Err(errs), _) => Validated::Err(errs),
            (_, Result::Err(e)) => Validated::Err(vec![e]),
            (Validated::Ok(h1), Result::Ok(v2)) => Validated::Ok(h1 + hlist!(v2)),
        }
    }

    /// Turns this Validated into a Result.
    ///
    /// If this Validated is Ok, it will become a Result::Ok, holding an HList of all the accumulated
    /// results. Otherwise, it will become a Result::Err with a list of all accumulated errors.
    ///
    /// ```
    /// # #[macro_use] extern crate frust; use frust::hlist::*; use frust::validated::*; fn main() {
    ///
    /// #[derive(PartialEq, Eq, Debug)]
    /// struct Person {
    ///     age: i32,
    ///     name: String,
    /// }
    ///
    /// fn get_name() -> Result<String, String> {
    ///     Result::Ok("James".to_owned())
    /// }
    ///
    /// fn get_age() -> Result<i32, String> {
    ///     Result::Ok(32)
    /// }
    ///
    /// let v = get_name()
    ///         .into_validated()
    ///         .combine(get_age());
    /// let person = v.into_result()
    ///                .map(|HCons { head: name, tail: HCons { head: age, .. }, .. }| {
    ///                     Person {
    ///                         name: name,
    ///                         age: age,
    ///                     }
    ///                 });
    ///
    ///  assert_eq!(person,
    ///             Result::Ok(Person {
    ///                         name: "James".to_owned(),
    ///                        age: 32,
    ///             }));
    /// # }
    pub fn into_result(self) -> Result<T, Vec<E>> {
        match self {
            Validated::Ok(h) => Result::Ok(h),
            Validated::Err(errors) => Result::Err(errors),
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
        let v = r1.into_validated()
                  .combine(r2);
        assert_eq!(v, Validated::Ok(hlist!(String::from("hello"), 1)))
    }

    #[test]
    fn test_combining_validated_oks() {
        let r1: Result<String, String> = Result::Ok(String::from("hello"));
        let r2: Result<i32, String> = Result::Ok(1);
        let r3: Result<i32, String> = Result::Ok(3);
        let comb = r1.into_validated()
                     .combine(r2)
                     .combine(r3);
        assert_eq!(comb, Validated::Ok(hlist!(String::from("hello"), 1, 3)))
    }

    #[test]
    fn test_chaining_invalid() {
        let r1: Result<i16, String> = Result::Ok(1);
        let r2: Result<i16, String> = Result::Err(String::from("NO!"));
        let v1 = r1.into_validated().combine(r2);
        assert!(v1.is_err())
    }

    #[derive(PartialEq, Eq, Debug)]
    struct Person {
        age: i32,
        name: String,
    }

    fn get_name() -> Result<String, String> {
        Result::Ok("James".to_owned())
    }

    fn get_age() -> Result<i32, String> {
        Result::Ok(32)
    }

    fn get_name_faulty() -> Result<String, String> {
        Result::Err("crap name".to_owned())
    }

    fn get_age_faulty() -> Result<i32, String> {
        Result::Err("crap age".to_owned())
    }

    #[test]
    fn test_to_result_ok() {

        let v = get_name()
                    .into_validated()
                    .combine(get_age());
        let person = v.into_result()
                      .map(|HCons { head: name, tail: HCons { head: age, .. }, .. }| {
                          Person {
                              name: name,
                              age: age,
                          }
                      });

        assert_eq!(person,
                   Result::Ok(Person {
                       name: "James".to_owned(),
                       age: 32,
                   }));
    }

    #[test]
    fn test_to_result_faulty() {

        let v = get_name_faulty()
                    .into_validated()
                    .combine(get_age_faulty());
        let person = v.into_result()
                      .map(|HCons { head: name, tail: HCons { head: age, .. }, .. }| {
                          Person {
                              name: name,
                              age: age,
                          }
                      });

        assert_eq!(person,
                   Result::Err(vec!["crap name".to_owned(), "crap age".to_owned()]));
    }

}
