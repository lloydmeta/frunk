//! Module for holding Validated logic
//!
//! `Validated` is a way of running a bunch of operations that can go wrong (for example,
//! functions returning `Result<T, E>`) and, in the case of one or more things going wrong,
//! having all the errors returned to you all at once. In the case that everything went well, you get
//! an `HList` of all your results.
//!
//! # Examples
//!
//! ```
//! # fn main() {
//! use frunk::Validated;
//! use frunk::prelude::*;
//! use frunk_core::{HList, hlist_pat};
//!
//! #[derive(PartialEq, Eq, Debug)]
//! struct Person {
//!     age: i32,
//!     name: String,
//! }
//!
//! fn get_name() -> Result<String, String> {
//!     Ok("James".to_owned())
//! }
//!
//! fn get_age() -> Result<i32, String> {
//!     Ok(32)
//! }
//!
//! let v: Validated<HList!(String, i32), String> = get_name().into_validated() + get_age();
//! let person = v.into_result()
//!                .map(|hlist_pat!(name, age)| {
//!                     Person {
//!                         name,
//!                         age,
//!                     }
//!                 });
//!
//!  assert_eq!(person.unwrap(),
//!             Person {
//!                 name: "James".to_owned(),
//!                 age: 32,
//!             });
//! # }
//! ```

use super::hlist::*;

use std::ops::Add;

/// A Validated is either an Ok holding an HList or an Err, holding a vector
/// of collected errors.
#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub enum Validated<T, E>
where
    T: HList,
{
    Ok(T),
    Err(Vec<E>),
}

impl<T, E> Validated<T, E>
where
    T: HList,
{
    /// Returns true if this validation is Ok, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use frunk::Validated;
    /// use frunk::prelude::*;
    ///
    /// let r1: Result<String, String> = Ok(String::from("hello"));
    /// let v = r1.into_validated();
    /// assert!(v.is_ok());
    /// ```
    pub fn is_ok(&self) -> bool {
        matches!(*self, Validated::Ok(_))
    }

    /// Returns true if this validation is Err, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use frunk::prelude::*;
    ///
    /// let r1: Result<String, i32> = Err(32);
    /// let v = r1.into_validated();
    /// assert!(v.is_err());
    /// ```
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Turns this Validated into a Result.
    ///
    /// If this Validated is Ok, it will become a Ok, holding an HList of all the accumulated
    /// results. Otherwise, it will become a Err with a list of all accumulated errors.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() {
    /// use frunk_core::hlist_pat;
    /// use frunk::Validated;
    /// use frunk::prelude::*;
    ///
    /// #[derive(PartialEq, Eq, Debug)]
    /// struct Person {
    ///     age: i32,
    ///     name: String,
    /// }
    ///
    /// fn get_name() -> Result<String, String> {
    ///     Ok("James".to_owned())
    /// }
    ///
    /// fn get_age() -> Result<i32, String> {
    ///     Ok(32)
    /// }
    ///
    /// let v = get_name().into_validated() + get_age();
    /// let person = v.into_result()
    ///                .map(|hlist_pat!(name, age)| {
    ///                     Person {
    ///                         name,
    ///                         age,
    ///                     }
    ///                 });
    ///
    ///  assert_eq!(person.unwrap(),
    ///             Person {
    ///                 name: "James".to_owned(),
    ///                 age: 32,
    ///             });
    /// # }
    pub fn into_result(self) -> Result<T, Vec<E>> {
        match self {
            Validated::Ok(h) => Ok(h),
            Validated::Err(errors) => Err(errors),
        }
    }
}

/// Trait for "lifting" a given type into a Validated
pub trait IntoValidated<T, E> {
    /// Consumes the current Result into a Validated so that we can begin chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use frunk::prelude::*; // IntoValidated is in the prelude
    ///
    /// let r1: Result<String, i32> = Err(32);
    /// let v = r1.into_validated();
    /// assert!(v.is_err());
    /// ```
    fn into_validated(self) -> Validated<HCons<T, HNil>, E>;
}

impl<T, E> IntoValidated<T, E> for Result<T, E> {
    fn into_validated(self) -> Validated<HCons<T, HNil>, E> {
        match self {
            Err(e) => Validated::Err(vec![e]),
            Ok(v) => Validated::Ok(HCons {
                head: v,
                tail: HNil,
            }),
        }
    }
}

/// Implements Add for the current Validated with a Result, returning a new Validated.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use frunk::Validated;
/// use frunk::prelude::*;
/// use frunk_core::hlist;
///
/// let r1: Result<String, String> = Ok(String::from("hello"));
/// let r2: Result<i32, String> = Ok(1);
/// let v = r1.into_validated() + r2;
/// assert_eq!(v, Validated::Ok(hlist!(String::from("hello"), 1)))
/// # }
/// ```
///
impl<T, E, T2> Add<Result<T2, E>> for Validated<T, E>
where
    T: HList + Add<HCons<T2, HNil>>,
    <T as Add<HCons<T2, HNil>>>::Output: HList,
{
    type Output = Validated<<T as Add<HCons<T2, HNil>>>::Output, E>;

    fn add(self, other: Result<T2, E>) -> Self::Output {
        let other_as_validated = other.into_validated();
        self + other_as_validated
    }
}

/// Implements Add for the current Validated with another Validated, returning a new Validated.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use frunk::Validated;
/// use frunk::prelude::*;
/// use frunk_core::hlist;
///
/// let r1: Result<String, String> = Ok(String::from("hello"));
/// let r2: Result<i32, String> = Ok(1);
/// let v1 = r1.into_validated();
/// let v2 = r2.into_validated();
/// let v3 = v1 + v2;
/// assert_eq!(v3, Validated::Ok(hlist!(String::from("hello"), 1)))
/// # }
/// ```
impl<T, E, T2> Add<Validated<T2, E>> for Validated<T, E>
where
    T: HList + Add<T2>,
    T2: HList,
    <T as Add<T2>>::Output: HList,
{
    type Output = Validated<<T as Add<T2>>::Output, E>;

    fn add(self, other: Validated<T2, E>) -> Self::Output {
        match (self, other) {
            (Validated::Err(mut errs), Validated::Err(errs2)) => {
                errs.extend(errs2);
                Validated::Err(errs)
            }
            (Validated::Err(errs), _) => Validated::Err(errs),
            (_, Validated::Err(errs)) => Validated::Err(errs),
            (Validated::Ok(h1), Validated::Ok(h2)) => Validated::Ok(h1 + h2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frunk_core::{hlist, hlist_pat};

    #[test]
    fn test_adding_ok_results() {
        let r1: Result<String, String> = Ok(String::from("hello"));
        let r2: Result<i32, String> = Ok(1);
        let v = r1.into_validated() + r2;
        assert_eq!(v, Validated::Ok(hlist!(String::from("hello"), 1)))
    }

    #[test]
    fn test_adding_validated_oks() {
        let r1: Result<String, String> = Ok(String::from("hello"));
        let r2: Result<i32, String> = Ok(1);
        let r3: Result<i32, String> = Ok(3);
        let v1 = r1.into_validated();
        let v2 = r2.into_validated();
        let v3 = r3.into_validated();
        let comb = v1 + v2 + v3;
        assert_eq!(comb, Validated::Ok(hlist!(String::from("hello"), 1, 3)))
    }

    #[test]
    fn test_adding_err_results() {
        let r1: Result<i16, String> = Ok(1);
        let r2: Result<i16, String> = Err(String::from("NO!"));
        let v1 = r1.into_validated() + r2;
        assert!(v1.is_err());
        assert_eq!(v1, Validated::Err(vec!["NO!".to_owned()]))
    }

    #[derive(PartialEq, Eq, Debug)]
    struct Person {
        age: i32,
        name: String,
        email: String,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum YahNah {
        Yah,
        Nah,
    }

    /// Our Errors
    #[derive(PartialEq, Eq, Debug)]
    pub enum Nope {
        Name,
        Age,
        Email,
    }

    fn get_name(yah_nah: YahNah) -> Result<String, Nope> {
        match yah_nah {
            YahNah::Yah => Ok("James".to_owned()),
            _ => Err(Nope::Name),
        }
    }

    fn get_age(yah_nah: YahNah) -> Result<i32, Nope> {
        match yah_nah {
            YahNah::Yah => Ok(32),
            _ => Err(Nope::Age),
        }
    }

    fn get_email(yah_nah: YahNah) -> Result<String, Nope> {
        match yah_nah {
            YahNah::Yah => Ok("hello@world.com".to_owned()),
            _ => Err(Nope::Email),
        }
    }

    #[test]
    fn test_to_result_ok() {
        let v =
            get_name(YahNah::Yah).into_validated() + get_age(YahNah::Yah) + get_email(YahNah::Yah);
        let person =
            v.into_result()
                .map(|hlist_pat!(name, age, email)| Person { name, age, email });

        assert_eq!(
            person.unwrap(),
            Person {
                name: "James".to_owned(),
                age: 32,
                email: "hello@world.com".to_owned(),
            }
        );
    }

    #[test]
    fn test_to_result_all_faulty() {
        let v =
            get_name(YahNah::Nah).into_validated() + get_age(YahNah::Nah) + get_email(YahNah::Nah);
        let person = v.into_result().map(|_| unimplemented!());

        assert_eq!(
            person.unwrap_err(),
            vec![Nope::Name, Nope::Age, Nope::Email]
        );
    }

    #[test]
    fn test_to_result_one_faulty() {
        let v =
            get_name(YahNah::Nah).into_validated() + get_age(YahNah::Yah) + get_email(YahNah::Nah);
        let person = v.into_result().map(|_| unimplemented!());

        assert_eq!(person.unwrap_err(), vec![Nope::Name, Nope::Email]);
    }
}
