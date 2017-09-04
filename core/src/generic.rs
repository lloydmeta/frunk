//! This module holds the machinery behind Generic.
//!
//! It contains the Generic typeclass and some helper methods for using the Generic type class
//! without having to use universal function call syntax.
//!
//! # Examples
//!
//! ```rust
//! # #[allow(unused_imports)]
//! # #[macro_use] extern crate frunk_derives;
//! # #[macro_use] extern crate frunk_core;
//! # use frunk_core::hlist::*; fn main() {
//! # use frunk_core::hlist::*;
//! # use frunk_core::generic::*;
//! #[derive(Generic)]
//! struct ApiPerson<'a> {
//!     FirstName: &'a str,
//!     LastName: &'a str,
//!     Age: usize,
//! }
//!
//! #[derive(Generic)]
//! struct DomainPerson<'a> {
//!     first_name: &'a str,
//!     last_name: &'a str,
//!     age: usize,
//! }
//!
//! let a_person = ApiPerson {
//!     FirstName: "Joe",
//!     LastName: "Blow",
//!     Age: 30,
//! };
//! let d_person: DomainPerson = convert_from(a_person); // done
//! # }

/// A trait that converts from a type to a generic representation
///
/// For the most part, you should be using the derivation that is available through
/// frunk_derive to generate instances of this typeclass for your types.
///
/// # Examples
///
/// ```rust
/// # #[allow(unused_imports)]
/// # #[macro_use] extern crate frunk_derives;
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::hlist::*; fn main() {
/// use frunk_core::hlist::*;
/// use frunk_core::generic::*;
/// #[derive(Generic)]
/// struct ApiPerson<'a> {
///     FirstName: &'a str,
///     LastName: &'a str,
///     Age: usize,
/// }
///
/// #[derive(Generic)]
/// struct DomainPerson<'a> {
///     first_name: &'a str,
///     last_name: &'a str,
///     age: usize,
/// }
///
/// let a_person = ApiPerson {
///     FirstName: "Joe",
///     LastName: "Blow",
///     Age: 30,
/// };
/// let d_person: DomainPerson = convert_from(a_person); // done
/// # }
/// ```
pub trait Generic {
    /// The generic representation type
    type Repr;

    /// Go from something to Repr
    fn into(self) -> Self::Repr;

    /// Go from Repr to something
    fn from(r: Self::Repr) -> Self;

    /// From one type to another using a type with a compatible generic representation
    fn convert_from<A>(a: A) -> Self
    where
        A: Generic<Repr = Self::Repr>,
        Self: Sized,
    {
        let repr = <A as Generic>::into(a);
        <Self as Generic>::from(repr)
    }
}

/// Given a generic Representation of an A, returns A
pub fn from_generic<A, Repr>(gen: Repr) -> A
where
    A: Generic<Repr = Repr>,
{
    <A as Generic>::from(gen)
}

/// Given an A, returns its generic Representation
pub fn into_generic<A, Repr>(a: A) -> Repr
where
    A: Generic<Repr = Repr>,
{
    <A as Generic>::into(a)
}

/// Converts one type into another assuming they have the same generic Representation
pub fn convert_from<A, B, Repr>(a: A) -> B
where
    A: Generic<Repr = Repr>,
    B: Generic<Repr = Repr>,
{
    <B as Generic>::convert_from(a)
}
