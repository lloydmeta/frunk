//! This module holds the machinery behind LabelledGeneric.
//!
//! A LabelledGeneric instance is pretty much exactly the same as a Generic instance, except
//! that the generic representation should contain information about field names.
//!
//! Having a separate trait for LabelledGenerics gives us the freedom to derive both
//! lablled and non-labelled generic type class instances for our types.
//!
//! Asides from the main LabelledGeneric trait, this module holds helper methods that allow
//! users to use LabelledGeneric without using universal function call syntax.
//!
//! In addition, this module holds macro-generated enums that map to letters in field names (identifiers).

use std::marker::PhantomData;
use hlist::*;

/// A trait that converts from a type to a labelled generic representation
///
/// For the most part, you should be using the derivation that is available through
/// frunk_derive to generate instances of this typeclass for your types.
///
/// I would highly recommend you check out `derivation_tests.rs` to see how to actually use
/// this trait in real life. Since frunk_derive depends on this trait, I can't actually
/// pull it in as a dependency here (otherwise the dependency would be circular) and show
/// how to use it in a proper doc test.
///
/// ```rust,ignore
/// #[derive(LabelledGeneric)]
/// struct NewUser<'a> {
///     first_name: &'a str,
///     last_name: &'a str,
///     age: usize,
/// }
///
/// #[derive(LabelledGeneric)]
/// struct SavedUser<'a> {
///     first_name: &'a str,
///     last_name: &'a str,
///     age: usize,
/// }
///
/// let n_user = NewUser {
///     first_name: "Joe",
///     last_name: "Blow",
///     age: 30,
/// };
///
/// let s_user = <SavedUser as LabelledGeneric>::convert_from(n_user); // done
/// ```
pub trait LabelledGeneric {
    /// The labelled generic representation type
    type Repr;

    /// Go from something to Repr
    fn into(self) -> Self::Repr;

    /// Go from labelled Repr to something
    fn from(r: Self::Repr) -> Self;

    /// From one type to another using a type with a compatible labelled generic representation
    fn convert_from<A>(a: A) -> Self
        where A: LabelledGeneric<Repr = Self::Repr>,
              Self: Sized
    {
        let repr = <A as LabelledGeneric>::into(a);
        <Self as LabelledGeneric>::from(repr)
    }

    /// Converts from another type A into Self assuming that A and Self have labelled generic representations
    /// that can be sculpted into each other.
    ///
    /// Note that this method tosses away the "remainder" of the sculpted representation. In other
    /// words, anything that is not needed from A gets tossed out.
    fn sculpted_convert_from<A, Indices>(a: A) -> Self
        where A: LabelledGeneric,
              Self: Sized,
    // The labelled representation of A must be sculpt-able into the labelled representation of Self
              <A as LabelledGeneric>::Repr: Sculptor<<Self as LabelledGeneric>::Repr, Indices> {
        let a_gen = <A as LabelledGeneric>::into(a);
        // We toss away the remainder.
        let (self_gen, _): (<Self as LabelledGeneric>::Repr, _) = a_gen.sculpt();
        <Self as LabelledGeneric>::from(self_gen)
    }
}

/// Given a labelled generic Representation of an A, returns A
pub fn from_labelled_generic<A, Repr>(gen: Repr) -> A
    where A: LabelledGeneric<Repr = Repr>
{
    <A as LabelledGeneric>::from(gen)
}

/// Given an A, returns its labelled generic Representation
pub fn into_labelled_generic<A, Repr>(a: A) -> Repr
    where A: LabelledGeneric<Repr = Repr>
{
    <A as LabelledGeneric>::into(a)
}

/// Converts one type into another assuming they have the same labelled generic Representation
pub fn labelled_convert_from<A, B, Repr>(a: A) -> B
    where A: LabelledGeneric<Repr = Repr>,
          B: LabelledGeneric<Repr = Repr>
{
    <B as LabelledGeneric>::convert_from(a)
}

/// Converts from one type into another assuming that their labelled generic representations
/// can be sculpted into each other.
///
/// The "Indices" type parameter allows the compiler to figure out that the two representations
/// can indeed be morphed into each other.
pub fn sculpted_convert_from<A, B, Indices>(a: A) -> B
    where A: LabelledGeneric,
          B: LabelledGeneric,
// The labelled representation of A must be sculpt-able into the labelled representation of B
          <A as LabelledGeneric>::Repr: Sculptor<<B as LabelledGeneric>::Repr, Indices> {
    <B as LabelledGeneric>::sculpted_convert_from(a)
}

// Create a bunch of enums that can be used to represent characters on the type level
macro_rules! create_enums_for {
    ($($i: ident)*) => {
        $(
            #[allow(non_snake_case, non_camel_case_types)]
            #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
            pub enum $i {}
        )*
    }
}

// Add more as needed.
create_enums_for! { a b c d e f g h i j k l m n o p q r s t u v w x y z A B C D E F G H I J K L M N O P Q R S T U V W X Y Z __ _1 _2 _3 _4 _5 _6 _7 _8 _9 _0 _uc uc_ }

#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Labelled<Name, Type> {
    name: PhantomData<Name>,
    pub value: Type,
}

/// Helper function for building a new Labelled value.
///
/// Useful so that users don't need to deal with PhantomData directly.
///
/// ```
/// # use frunk_core::labelled::*;
/// let f1 = label::<(a, g, e), i32>(3);
/// let f2 = label::<(a, g, e), i32>(3);
/// assert_eq!(f1, f2)
///
/// ```
pub fn label<Label, Value>(value: Value) -> Labelled<Label, Value> {
    Labelled {
        name: PhantomData,
        value: value,
    }
}

/// Trait for turning a Labelled HList into an un-labelled HList
pub trait IntoUnlabelled {
    type Output;

    /// Turns the current HList into an unlabelled on.
    ///
    /// Effectively extracts the values held inside the individual Labelled
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # use frunk_core::labelled::*;
    /// # use frunk_core::hlist::*;
    /// # fn main() {
    ///
    /// let labelled_hlist = hlist![
    ///     label::<(n, a, m, e), _>("joe"),
    ///     label::<(a, g, e), _>(3)
    /// ];
    ///
    /// let unlabelled = labelled_hlist.into_unlabelled();
    ///
    /// assert_eq!(unlabelled, hlist!["joe", 3])
    /// # }
    /// ```
    fn into_unlabelled(self) -> Self::Output;
}

/// Implementation for HNil
impl IntoUnlabelled for HNil {
    type Output = HNil;
    fn into_unlabelled(self) -> Self::Output {
        self
    }
}

/// Implementation when we have a non-empty HCons holding a label in its head
impl<Label, Value, Tail> IntoUnlabelled for HCons<Labelled<Label, Value>, Tail>
    where Tail: IntoUnlabelled
{
    type Output = HCons<Value, <Tail as IntoUnlabelled>::Output>;

    fn into_unlabelled(self) -> Self::Output {
        HCons {
            head: self.head.value,
            tail: self.tail.into_unlabelled(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_construction() {
        let f1 = label::<(a, g, e), i32>(3);
        let f2 = label::<(a, g, e), i32>(3);
        assert_eq!(f1, f2)
    }

    #[test]
    fn test_anonymous_record_useage() {
        let record = hlist![
            label::<(n, a, m, e), _>("Joe"),
            label::<(a, g, e), _>(30)
        ];
        let (name, _): (Labelled<(n, a, m, e), _>, _) = record.pluck();
        assert_eq!(name.value, "Joe")
    }

    #[test]
    fn test_unlabelling() {
        let labelled_hlist = hlist![
            label::<(n, a, m, e), &str>("joe"),
            label::<(a, g, e), i32>(3)];
        let unlabelled = labelled_hlist.into_unlabelled();
        assert_eq!(unlabelled, hlist!["joe", 3])
    }
}
