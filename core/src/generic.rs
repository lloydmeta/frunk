//! This module holds the machinery behind `Generic`.
//!
//! It contains the `Generic` trait and some helper methods for using the
//! `Generic` trait without having to use universal function call syntax.
//!
//! # Examples
//!
//! ```rust
//! use frunk::Generic;
//!
//! # fn main() {
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
//! let d_person: DomainPerson = frunk::convert_from(a_person); // done
//! # }

/// A trait that converts from a type to a generic representation.
///
/// For the most part, you should be using the derivation that is available
/// through `frunk_derive` to generate instances of this trait for your types.
///
/// # Laws
///
/// Any implementation of `Generic` must satisfy the following two laws:
///
/// 1. `forall x : Self. x == Generic::from(Generic::into(x))`
/// 2. `forall y : Repr. y == Generic::into(Generic::from(y))`
///
/// That is, `from` and `into` should make up an isomorphism between
/// `Self` and the representation type `Repr`.
///
/// # Examples
///
/// ```rust
/// use frunk::Generic;
///
/// # fn main() {
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
/// let d_person: DomainPerson = frunk::convert_from(a_person); // done
/// # }
/// ```
pub trait Generic {
    /// The generic representation type.
    type Repr;

    /// Convert a value to its representation type `Repr`.
    fn into(self) -> Self::Repr;

    /// Convert a value's representation type `Repr` to the value's type.
    fn from(repr: Self::Repr) -> Self;

    /// Convert a value to another type provided that they have
    /// the same representation type.
    fn convert_from<Src>(src: Src) -> Self
    where
        Self: Sized,
        Src: Generic<Repr = Self::Repr>,
    {
        let repr = <Src as Generic>::into(src);
        <Self as Generic>::from(repr)
    }

    /// Maps the given value of type `Self` by first transforming it to
    /// the representation type `Repr`, then applying a `mapper` function
    /// on `Repr` and finally transforming it back to a value of type `Self`.
    fn map_repr<Mapper>(self, mapper: Mapper) -> Self
    where
        Self: Sized,
        Mapper: FnOnce(Self::Repr) -> Self::Repr,
    {
        Self::from(mapper(self.into()))
    }

    /// Maps the given value of type `Self` by first transforming it
    /// a type `Inter` that has the same representation type as `Self`,
    /// then applying a `mapper` function on `Inter` and finally transforming
    /// it back to a value of type `Self`.
    fn map_inter<Inter, Mapper>(self, mapper: Mapper) -> Self
    where
        Self: Sized,
        Inter: Generic<Repr = Self::Repr>,
        Mapper: FnOnce(Inter) -> Inter,
    {
        Self::convert_from(mapper(Inter::convert_from(self)))
    }
}

/// Given a generic representation `Repr` of a `Dst`, returns `Dst`.
pub fn from_generic<Dst, Repr>(repr: Repr) -> Dst
where
    Dst: Generic<Repr = Repr>,
{
    <Dst as Generic>::from(repr)
}

/// Given a value of type `Src`, returns its generic representation `Repr`.
pub fn into_generic<Src, Repr>(src: Src) -> Repr
where
    Src: Generic<Repr = Repr>,
{
    <Src as Generic>::into(src)
}

/// Converts one type `Src` into another type `Dst` assuming they have the same
/// representation type `Repr`.
pub fn convert_from<Src, Dst, Repr>(src: Src) -> Dst
where
    Src: Generic<Repr = Repr>,
    Dst: Generic<Repr = Repr>,
{
    <Dst as Generic>::convert_from(src)
}

/// Maps a value of a given type `Origin` using a function on
/// the representation type `Repr` of `Origin`.
pub fn map_repr<Origin, Mapper>(val: Origin, mapper: Mapper) -> Origin
where
    Origin: Generic,
    Mapper: FnOnce(Origin::Repr) -> Origin::Repr,
{
    <Origin as Generic>::map_repr(val, mapper)
}

/// Maps a value of a given type `Origin` using a function on
/// a type `Inter` which has the same representation type of `Origin`.
///
/// Note that the compiler will have a hard time inferring the type variable
/// `Inter`. Thus, using `map_inter` is mostly effective if the type is
/// constrained by the input function or by the body of a lambda.
pub fn map_inter<Inter, Origin, Mapper>(val: Origin, mapper: Mapper) -> Origin
where
    Origin: Generic,
    Inter: Generic<Repr = Origin::Repr>,
    Mapper: FnOnce(Inter) -> Inter,
{
    <Origin as Generic>::map_inter(val, mapper)
}
