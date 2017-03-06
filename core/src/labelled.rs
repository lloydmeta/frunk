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
//!
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
/// # fn main() {
/// let labelled = label![(n,a,m,e), "Lloyd"];
/// assert_eq!(labelled.name, "name")
/// # }
/// ```

use std::marker::PhantomData;
use hlist::*;
use std::fmt;

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
/// let s_user = <SavedUser as LabelledGeneric>::sculpted_convert_from(n_user); // done
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
create_enums_for! { a b c d e f g h i j k l m n o p q r s t u v w x y z A B C D E F G H I J K L M N O P Q R S T U V W X Y Z __ _1 _2 _3 _4 _5 _6 _7 _8 _9 _0 }

/// A Label contains a type-level Name, a runtime value, and
/// a reference to a `&'static str` name.
///
/// To construct one, use the `label!` macro.
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
/// # fn main() {
/// let labelled = label![(n,a,m,e), "joe"];
/// assert_eq!(labelled.name, "name")
/// # }
/// ```
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Labelled<Name, Type> {
    name_type_holder: PhantomData<Name>,
    pub name: &'static str,
    pub value: Type,
}

impl <Name, Type> fmt::Debug for Labelled<Name, Type>
    where Type: fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v_debug = format!("{:?}", self.value);
        write!(f, "Labelled{{ name: {}, value: {} }}", self.name, v_debug)
    }
}

/// Returns a new label for a given value and custom name.
///
/// If you don't want to provide a custom name and want to rely on the type you provide
/// to build a name, then please use the label! macro.
///
/// ```
/// # use frunk_core::labelled::*;
/// let l = label_with_name::<(n,a,m,e),_>("name", "joe");
/// assert_eq!(l.value, "joe");
/// assert_eq!(l.name, "name");
/// ```
pub fn label_with_name<Label, Value>(name: &'static str, value: Value) -> Labelled<Label, Value> {
    Labelled {
        name_type_holder: PhantomData,
        name: name,
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
    ///     label!((n, a, m, e), "joe"),
    ///     label!((a, g, e), 3)
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

/// Used for creating a Labelled value
///
/// There are two forms of this macro:
///
/// * Create an instance of the `Labelled` struct with the name
///   set to the the concatenation of the types passed in the
///   tuple used as the first argument.
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
/// # fn main() {
/// let labelled = label![(n,a,m,e), "joe"];
/// assert_eq!(labelled.name, "name")
/// # }
/// ```
///
/// * Create an instance of the `Labelled` struct with a custom, name,
///   passed as the last argument in the macro
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
/// # fn main() {
/// let labelled = label![(a,g,e), 30, "Age"];
/// assert_eq!(labelled.name, "Age");
/// # }
/// ```
#[macro_export]
macro_rules! label {
    // No name provided and type is a tuple
    (($($repeated: ty),*), $value: expr) => {
        label!( ($($repeated),*), $value, concat!( $(stringify!($repeated)),* ) )
    };
    // No name provided and type is a tuple, but with trailing commas
    (($($repeated: ty,)*), $value: expr) => {
        label!( ($($repeated),*), $value )
    };
    // We are provided any type, with a stable name
    ($name_type: ty, $value: expr, $name: expr) => {
        $crate::labelled::label_with_name::<$name_type,_>($name, $value)
    }
}

#[test]
fn test_label_new_building() {
    let l1 = label!((a, b, c), 3);
    assert_eq!(l1.value, 3);
    assert_eq!(l1.name, "abc");
    let l2 = label!((a, b, c,), 3);
    assert_eq!(l2.value, 3);
    assert_eq!(l2.name, "abc");

    // test named
    let l3 = label!((a,b,c), 3, "nope");
    assert_eq!(l3.value, 3);
    assert_eq!(l3.name, "nope");
    let l4 = label!((a,b,c,), 3, "nope");
    assert_eq!(l4.value, 3);
    assert_eq!(l4.name, "nope");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_construction() {
        let f1 = label!((a, g, e), 3);
        let f2 = label!((a, g, e), 3);
        assert_eq!(f1, f2)
    }

    #[test]
    fn test_anonymous_record_useage() {
        let record = hlist![
            label!((n, a, m, e), "Joe"),
            label!((a, g, e), 30)
        ];
        let (name, _): (Labelled<(n, a, m, e), _>, _) = record.pluck();
        assert_eq!(name.value, "Joe")
    }

    #[test]
    fn test_unlabelling() {
        let labelled_hlist = hlist![
            label!((n, a, m, e), "joe"),
            label!((a, g, e), 3)];
        let unlabelled = labelled_hlist.into_unlabelled();
        assert_eq!(unlabelled, hlist!["joe", 3])
    }

    #[test]
    fn test_name() {
        let labelled = label!((n, a, m, e), "joe");
        assert_eq!(labelled.name, "name")
    }
}
