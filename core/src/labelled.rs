//! This module holds the machinery behind LabelledGeneric.
//!
//! A `LabelledGeneric` instance is pretty much exactly the same as a `Generic`
//! instance, except that the generic representation should contain information
//! about field names.
//!
//! Having a separate trait for `LabelledGeneric`s gives us the freedom to
//! derive both labelled and non-labelled generic trait instances for our types.
//!
//! Asides from the main `LabelledGeneric` trait, this module holds helper
//! methods that allow users to use `LabelledGeneric` without using universal
//! function call syntax.
//!
//! In addition, this module holds macro-generated enums that map to letters
//! in field names (identifiers).
//!
//! # Examples
//!
//! ```
//! #[macro_use]
//! extern crate frunk;
//!
//! # fn main() {
//! use frunk::labelled::chars::*;
//!
//! // Optionally alias our tuple that represents our type-level string
//! type name = (n, a, m, e);
//! let labelled = field![name, "Lloyd"];
//! assert_eq!(labelled.name, "name");
//! assert_eq!(labelled.value, "Lloyd")
//! # }
//! ```
//!
//! A more common usage is to use `LabelledGeneric` to transform structs that
//! have mismatched fields!
//!
//! ```
//! #[macro_use] extern crate frunk;
//! #[macro_use] extern crate frunk_core; // required when using custom derives
//!
//! # fn main() {
//! #[derive(LabelledGeneric)]
//! struct NewUser<'a> {
//!     first_name: &'a str,
//!     last_name: &'a str,
//!     age: usize,
//! }
//!
//! // Notice that the fields are mismatched in terms of ordering
//! // *and* also in terms of the number of fields.
//! #[derive(LabelledGeneric)]
//! struct ShortUser<'a> {
//!     last_name: &'a str,
//!     first_name: &'a str,
//! }
//!
//! let n_user = NewUser {
//!     first_name: "Joe",
//!     last_name: "Blow",
//!     age: 30,
//! };
//!
//! // transform_from automagically sculpts the labelled generic
//! // representation of the source object to that of the target type
//! let s_user: ShortUser = frunk::transform_from(n_user); // done
//! # }
//! ```

use std::marker::PhantomData;
use hlist::*;
use std::fmt;

/// A trait that converts from a type to a labelled generic representation.
///
/// `LabelledGeneric`s allow us to have completely type-safe,
/// boilerplate free conversions between different structs.
///
/// For the most part, you should be using the derivation that is available
/// through `frunk_derive` to generate instances of this trait for your types.
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate frunk;
/// #[macro_use] extern crate frunk_core;
///
/// # fn main() {
/// #[derive(LabelledGeneric)]
/// struct NewUser<'a> {
///     first_name: &'a str,
///     last_name: &'a str,
///     age: usize,
/// }
///
/// // Notice that the fields are mismatched in terms of ordering
/// #[derive(LabelledGeneric)]
/// struct SavedUser<'a> {
///     last_name: &'a str,
///     age: usize,
///     first_name: &'a str,
/// }
///
/// let n_user = NewUser {
///     first_name: "Joe",
///     last_name: "Blow",
///     age: 30,
/// };
///
/// // transform_from automagically sculpts the labelled generic
/// // representation of the source object to that of the target type
/// let s_user: SavedUser = frunk::transform_from(n_user); // done
/// # }
pub trait LabelledGeneric {
    /// The labelled generic representation type.
    type Repr;

    /// Convert a value to its representation type `Repr`.
    fn into(self) -> Self::Repr;

    /// Convert a value's labelled representation type `Repr`
    /// to the values's type.
    fn from(repr: Self::Repr) -> Self;

    /// Convert from one type to another using a type with the same
    /// labelled generic representation
    fn convert_from<Src>(src: Src) -> Self
    where
        Src: LabelledGeneric<Repr = Self::Repr>,
        Self: Sized,
    {
        let repr = <Src as LabelledGeneric>::into(src);
        <Self as LabelledGeneric>::from(repr)
    }

    /// Converts from another type A into Self assuming that A and Self have
    /// labelled generic representations that can be sculpted into each other.
    ///
    /// Note that this method tosses away the "remainder" of the sculpted representation. In other
    /// words, anything that is not needed from A gets tossed out.
    #[deprecated = "obsolete, transform_from instead"]
    fn sculpted_convert_from<A, Indices>(a: A) -> Self
    where
        A: LabelledGeneric,
        Self: Sized,
        // The labelled representation of A must be sculpt-able into the labelled representation of Self
        <A as LabelledGeneric>::Repr: Sculptor<<Self as LabelledGeneric>::Repr, Indices>,
    {
        <Self as LabelledGeneric>::transform_from(a)
    }

    /// Converts from another type `Src` into `Self` assuming that `Src` and
    /// `Self` have labelled generic representations that can be sculpted into
    /// each other.
    ///
    /// Note that this method tosses away the "remainder" of the sculpted
    /// representation. In other words, anything that is not needed from `Src`
    /// gets tossed out.
    fn transform_from<Src, Indices>(src: Src) -> Self
    where
        Src: LabelledGeneric,
        Self: Sized,
        // The labelled representation of `Src` must be sculpt-able into the labelled representation of `Self`
        <Src as LabelledGeneric>::Repr: Sculptor<<Self as LabelledGeneric>::Repr, Indices>,
    {
        let src_gen = <Src as LabelledGeneric>::into(src);
        // We toss away the remainder.
        let (self_gen, _): (<Self as LabelledGeneric>::Repr, _) = src_gen.sculpt();
        <Self as LabelledGeneric>::from(self_gen)
    }
}

/// Given a labelled generic representation of an `Dst`, returns `Dst`
pub fn from_labelled_generic<Dst, Repr>(repr: Repr) -> Dst
where
    Dst: LabelledGeneric<Repr = Repr>,
{
    <Dst as LabelledGeneric>::from(repr)
}

/// Given a `Src`, returns its labelled generic representation.
pub fn into_labelled_generic<Src, Repr>(src: Src) -> Repr
where
    Src: LabelledGeneric<Repr = Repr>,
{
    <Src as LabelledGeneric>::into(src)
}

/// Converts one type into another assuming they have the same labelled generic
/// representation.
pub fn labelled_convert_from<Src, Dst, Repr>(src: Src) -> Dst
where
    Src: LabelledGeneric<Repr = Repr>,
    Dst: LabelledGeneric<Repr = Repr>,
{
    <Dst as LabelledGeneric>::convert_from(src)
}

/// Converts from one type into another assuming that their labelled generic representations
/// can be sculpted into each other.
///
/// The "Indices" type parameter allows the compiler to figure out that the two representations
/// can indeed be morphed into each other.
#[deprecated = "obsolete, transform_from instead"]
pub fn sculpted_convert_from<A, B, Indices>(a: A) -> B
where
    A: LabelledGeneric,
    B: LabelledGeneric,
    // The labelled representation of A must be sculpt-able into the labelled representation of B
    <A as LabelledGeneric>::Repr: Sculptor<<B as LabelledGeneric>::Repr, Indices>,
{
    <B as LabelledGeneric>::transform_from(a)
}
/// Converts from one type into another assuming that their labelled generic representations
/// can be sculpted into each other.
///
/// The "Indices" type parameter allows the compiler to figure out that the two representations
/// can indeed be morphed into each other.
pub fn transform_from<Src, Dst, Indices>(src: Src) -> Dst
where
    Src: LabelledGeneric,
    Dst: LabelledGeneric,
    // The labelled representation of Src must be sculpt-able into the labelled representation of Dst
    <Src as LabelledGeneric>::Repr: Sculptor<<Dst as LabelledGeneric>::Repr, Indices>,
{
    <Dst as LabelledGeneric>::transform_from(src)
}

pub mod chars {
    //! Types for building type-level labels from character sequences.
    //!
    //! This is designed to be glob-imported:
    //!
    //! ```rust
    //! # extern crate frunk;
    //! # fn main() {
    //! # #[allow(unused)]
    //! use frunk::labelled::chars::*;
    //! # }
    //! ```

    macro_rules! create_enums_for {
        ($($i: ident)*) => {
            $(
                #[allow(non_snake_case, non_camel_case_types)]
                #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
                pub enum $i {}
            )*
        }
    }

    // Add more as needed.
    create_enums_for! {
        // all valid identifier characters
        a b c d e f g h i j k l m n o p q r s t u v w x y z
        A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
        _1 _2 _3 _4 _5 _6 _7 _8 _9 _0 __
    }

    #[test]
    fn simple_var_names_are_allowed() {
        // Rust forbids variable bindings that shadow unit structs,
        // so unit struct characters would cause a lot of trouble.
        //
        // Good thing I don't plan on adding reified labels. - Exp
        let a = 3;
        match a {
            a => assert_eq!(a, 3),
        }
    }
}

/// A Label contains a type-level Name, a runtime value, and
/// a reference to a `&'static str` name.
///
/// To construct one, use the `field!` macro.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk;
/// use frunk::labelled::chars::*;
///
/// # fn main() {
/// let labelled = field![(n,a,m,e), "joe"];
/// assert_eq!(labelled.name, "name");
/// assert_eq!(labelled.value, "joe")
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Field<Name, Type> {
    name_type_holder: PhantomData<Name>,
    pub name: &'static str,
    pub value: Type,
}

/// A version of Field that doesn't have a type-level label, just a
/// value-level one
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct ValueField<Type> {
    pub name: &'static str,
    pub value: Type,
}

impl<Name, Type> fmt::Debug for Field<Name, Type>
where
    Type: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v_debug = format!("{:?}", self.value);
        write!(f, "Field{{ name: {}, value: {} }}", self.name, v_debug)
    }
}

impl<Type> fmt::Debug for ValueField<Type>
where
    Type: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v_debug = format!("{:?}", self.value);
        write!(f, "ValueField{{ name: {}, value: {} }}", self.name, v_debug)
    }
}

/// Returns a new Field for a given value and custom name.
///
/// If you don't want to provide a custom name and want to rely on the type you provide
/// to build a name, then please use the field! macro.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate frunk; fn main() {
/// use frunk::labelled::chars::*;
/// use frunk::labelled::field_with_name;
///
/// let l = field_with_name::<(n,a,m,e),_>("name", "joe");
/// assert_eq!(l.value, "joe");
/// assert_eq!(l.name, "name");
/// # }
/// ```
pub fn field_with_name<Label, Value>(name: &'static str, value: Value) -> Field<Label, Value> {
    Field {
        name_type_holder: PhantomData,
        name: name,
        value: value,
    }
}

/// Trait for turning a Field HList into an un-labelled HList
pub trait IntoUnlabelled {
    type Output;

    /// Turns the current HList into an unlabelled one.
    ///
    /// Effectively extracts the values held inside the individual Field
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// use frunk::labelled::chars::*;
    /// use frunk::labelled::IntoUnlabelled;
    ///
    /// let labelled_hlist = hlist![
    ///     field!((n, a, m, e), "joe"),
    ///     field!((a, g, e), 3)
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
impl<Label, Value, Tail> IntoUnlabelled for HCons<Field<Label, Value>, Tail>
where
    Tail: IntoUnlabelled,
{
    type Output = HCons<Value, <Tail as IntoUnlabelled>::Output>;

    fn into_unlabelled(self) -> Self::Output {
        HCons {
            head: self.head.value,
            tail: self.tail.into_unlabelled(),
        }
    }
}

/// A trait that strips type-level strings from the labels
pub trait IntoValueLabelled {
    type Output;

    /// Turns the current HList into a value-labelled one.
    ///
    /// Effectively extracts the names and values held inside the individual Fields
    /// and puts them into ValueFields, which do not have type-level names.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// use frunk::labelled::{ValueField, IntoValueLabelled};
    /// use frunk::labelled::chars::*;
    ///
    /// let labelled_hlist = hlist![
    ///     field!((n, a, m, e), "joe"),
    ///     field!((a, g, e), 3)
    /// ];
    /// // Notice the lack of type-level names
    /// let value_labelled: Hlist![ValueField<&str>, ValueField<isize>] = labelled_hlist.into_value_labelled();
    ///
    /// assert_eq!(
    ///   value_labelled,
    ///   hlist![
    ///     ValueField {
    ///       name: "name",
    ///       value: "joe",
    ///     },
    ///     ValueField {
    ///       name: "age",
    ///       value: 3,
    ///     },
    /// ]);
    /// # }
    /// ```
    fn into_value_labelled(self) -> Self::Output;
}

impl IntoValueLabelled for HNil {
    type Output = HNil;
    fn into_value_labelled(self) -> Self::Output {
        self
    }
}

impl<Label, Value, Tail> IntoValueLabelled for HCons<Field<Label, Value>, Tail>
where
    Tail: IntoValueLabelled,
{
    type Output = HCons<ValueField<Value>, <Tail as IntoValueLabelled>::Output>;

    fn into_value_labelled(self) -> Self::Output {
        HCons {
            head: ValueField {
                name: self.head.name,
                value: self.head.value,
            },
            tail: self.tail.into_value_labelled(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::chars::*;

    // Set up some aliases
    #[allow(non_camel_case_types)]
    type abc = (a, b, c);
    #[allow(non_camel_case_types)]
    type name = (n, a, m, e);
    #[allow(non_camel_case_types)]
    type age = (a, g, e);

    #[test]
    fn test_label_new_building() {
        let l1 = field!(abc, 3);
        assert_eq!(l1.value, 3);
        assert_eq!(l1.name, "abc");
        let l2 = field!((a, b, c), 3);
        assert_eq!(l2.value, 3);
        assert_eq!(l2.name, "abc");

        // test named
        let l3 = field!(abc, 3, "nope");
        assert_eq!(l3.value, 3);
        assert_eq!(l3.name, "nope");
        let l4 = field!((a, b, c), 3, "nope");
        assert_eq!(l4.value, 3);
        assert_eq!(l4.name, "nope");
    }

    #[test]
    fn test_field_construction() {
        let f1 = field!(age, 3);
        let f2 = field!((a, g, e), 3);
        assert_eq!(f1, f2)
    }

    #[test]
    fn test_anonymous_record_usage() {
        let record = hlist![field!(name, "Joe"), field!((a, g, e), 30)];
        let (name, _): (Field<name, _>, _) = record.pluck();
        assert_eq!(name.value, "Joe")
    }

    #[test]
    fn test_unlabelling() {
        let labelled_hlist = hlist![field!(name, "joe"), field!((a, g, e), 3)];
        let unlabelled = labelled_hlist.into_unlabelled();
        assert_eq!(unlabelled, hlist!["joe", 3])
    }

    #[test]
    fn test_value_labelling() {
        let labelled_hlist = hlist![field!(name, "joe"), field!((a, g, e), 3)];
        let value_labelled: Hlist![ValueField<&str>, ValueField<isize>] =
            labelled_hlist.into_value_labelled();
        let hlist_pat!(f1, f2) = value_labelled;
        assert_eq!(f1.name, "name");
        assert_eq!(f2.name, "age");
    }

    #[test]
    fn test_name() {
        let labelled = field!(name, "joe");
        assert_eq!(labelled.name, "name")
    }
}
