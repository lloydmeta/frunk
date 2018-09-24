//! This module holds the machinery behind LabelledGeneric.
//!
//! A `LabelledGeneric` instance is pretty much exactly the same as a `Generic`
//! instance, except that the generic representation should contain information
//! about field names.
//!
//! Having a separate trait for `LabelledGeneric`s gives us the freedom to
//! derive both labelled and non-labelled generic trait instances for our types.
//!
//! Aside from the main `LabelledGeneric` trait, this module holds helper
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

use hlist::*;
use indices::*;
use std::fmt;
use std::marker::PhantomData;

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

/// Given a labelled generic representation of a `Dst`, returns `Dst`
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

/// Trait for getting the key types of a given implementation (at the type-level)
pub trait Keys {
    type Out;
}

/// HNil implementation; just returns HNil
impl Keys for HNil {
    type Out = HNil;
}

/// Labelled HList implementation
impl<K, V, Tail: Keys> Keys for HCons<Field<K, V>, Tail> {
    type Out = HCons<K, <Tail as Keys>::Out>;
}

/// Trait for plucking out a value from a Record by type-level Key
pub trait ByKeyPlucker<TargetKey, Index> {
    type TargetValue;
    type Remainder;

    /// Returns a pair consisting of the  value pointed to by the target key and the remainder
    fn pluck_by_key(self) -> (Self::TargetValue, Self::Remainder);
}

/// Implementation when the pluck target key is in head
impl<K, V, Tail> ByKeyPlucker<K, Here> for HCons<Field<K, V>, Tail> {
    type TargetValue = V;
    type Remainder = Tail;

    #[inline(always)]
    fn pluck_by_key(self) -> (Self::TargetValue, Self::Remainder) {
        (self.head.value, self.tail)
    }
}

/// Implementation when the pluck target key is in the tail
impl<Head, Tail, K, TailIndex> ByKeyPlucker<K, There<TailIndex>> for HCons<Head, Tail>
where
    Tail: ByKeyPlucker<K, TailIndex>,
{
    type TargetValue = <Tail as ByKeyPlucker<K, TailIndex>>::TargetValue;
    type Remainder = HCons<Head, <Tail as ByKeyPlucker<K, TailIndex>>::Remainder>;

    #[inline(always)]
    fn pluck_by_key(self) -> (Self::TargetValue, Self::Remainder) {
        let (target, tail_remainder): (
            Self::TargetValue,
            <Tail as ByKeyPlucker<K, TailIndex>>::Remainder,
        ) = <Tail as ByKeyPlucker<K, TailIndex>>::pluck_by_key(self.tail);
        (
            target,
            HCons {
                head: self.head,
                tail: tail_remainder,
            },
        )
    }
}

/// Trait for plucking out a Field from a Record by type-level Key
pub trait ByNameFieldPlucker<TargetKey, Index> {
    type TargetValue;
    type Remainder;

    /// Returns a pair consisting of the  value pointed to by the target key and the remainder
    fn pluck_by_name(self) -> (Field<TargetKey, Self::TargetValue>, Self::Remainder);
}

/// Implementation when the pluck target key is in head
impl<K, V, Tail> ByNameFieldPlucker<K, Here> for HCons<Field<K, V>, Tail> {
    type TargetValue = V;
    type Remainder = Tail;

    #[inline(always)]
    fn pluck_by_name(self) -> (Field<K, Self::TargetValue>, Self::Remainder) {
        (self.head, self.tail)
    }
}

/// Implementation when the pluck target key is in the tail
impl<Head, Tail, K, TailIndex> ByNameFieldPlucker<K, There<TailIndex>> for HCons<Head, Tail>
where
    Tail: ByNameFieldPlucker<K, TailIndex>,
{
    type TargetValue = <Tail as ByNameFieldPlucker<K, TailIndex>>::TargetValue;
    type Remainder = HCons<Head, <Tail as ByNameFieldPlucker<K, TailIndex>>::Remainder>;

    #[inline(always)]
    fn pluck_by_name(self) -> (Field<K, Self::TargetValue>, Self::Remainder) {
        let (target, tail_remainder) =
            <Tail as ByNameFieldPlucker<K, TailIndex>>::pluck_by_name(self.tail);
        (
            target,
            HCons {
                head: self.head,
                tail: tail_remainder,
            },
        )
    }
}

/// Sculpts a given Labelled Record by type-level Keys, returning the remainder
pub trait ByKeySculptor<TargetKeys, Indices> {
    type TargetValues;
    type Remainder;

    /// Returns the values pointed to by the provided keys and the remainder
    fn sculpt_by_keys(self) -> (Self::TargetValues, Self::Remainder);
}

/// Implementation for when the target keys is an empty HList (HNil)
///
/// Index type is HNil because we don't need an index for finding HNil
impl<Source> ByKeySculptor<HNil, HNil> for Source {
    type TargetValues = HNil;
    type Remainder = Source;

    #[inline(always)]
    fn sculpt_by_keys(self) -> (Self::TargetValues, Self::Remainder) {
        (HNil, self)
    }
}

/// Implementation for when we have a non-empty HCons target
impl<TKeyHead, TKeyTail, SHead, STail, IndexHead, IndexTail>
    ByKeySculptor<HCons<TKeyHead, TKeyTail>, HCons<IndexHead, IndexTail>> for HCons<SHead, STail>
where
    HCons<SHead, STail>: ByKeyPlucker<TKeyHead, IndexHead>,
    <HCons<SHead, STail> as ByKeyPlucker<TKeyHead, IndexHead>>::Remainder:
        ByKeySculptor<TKeyTail, IndexTail>,
{
    type TargetValues = HCons<
        <HCons<SHead, STail> as ByKeyPlucker<TKeyHead, IndexHead>>::TargetValue,
        <<HCons<SHead, STail> as ByKeyPlucker<TKeyHead, IndexHead>>::Remainder as ByKeySculptor<
            TKeyTail,
            IndexTail,
        >>::TargetValues,
    >;
    type Remainder =
        <<HCons<SHead, STail> as ByKeyPlucker<TKeyHead, IndexHead>>::Remainder as ByKeySculptor<
            TKeyTail,
            IndexTail,
        >>::Remainder;

    #[inline(always)]
    fn sculpt_by_keys(self) -> (Self::TargetValues, Self::Remainder) {
        let (p, r) = self.pluck_by_key();
        let (tail, tail_remainder) = r.sculpt_by_keys();
        (
            HCons {
                head: p,
                tail: tail,
            },
            tail_remainder,
        )
    }
}

/// Trait for transmogrifying a Source type into a Target type
///
pub trait Transmogrifier<Target, TransmogrifyIndexIndices> {
    fn transmogrify(self) -> Target;
}

pub enum IdentityTransMog {}

pub struct DoTransmog<PluckByKeyIndex, TransMogIndex> {
    _marker1: PhantomData<PluckByKeyIndex>,
    _marker2: PhantomData<TransMogIndex>,
}

/// Implementation of Transmogrifier for identity transforms; when
/// Source is also Target
impl<Source> Transmogrifier<Source, HNil> for Source {
    fn transmogrify(self) -> Source {
        self
    }
}

/// Implementation of Transmogrifier for when the target is empty
impl<Source> Transmogrifier<HNil, IdentityTransMog> for Source {
    fn transmogrify(self) -> HNil {
        HNil
    }
}

impl<
        SourceHeadName,
        SourceHeadValue,
        SourceTail,
        TargetHeadName,
        TargetHeadValue,
        TargetTail,
        PluckSourceHeadNameIndex,
        TransMogSourceHeadValueIndices,
        TransMogTailIndices,
    >
    Transmogrifier<
        HCons<Field<TargetHeadName, TargetHeadValue>, TargetTail>,
        HCons<
            DoTransmog<PluckSourceHeadNameIndex, TransMogSourceHeadValueIndices>,
            TransMogTailIndices,
        >,
    > for HCons<Field<SourceHeadName, SourceHeadValue>, SourceTail>
where
    HCons<Field<SourceHeadName, SourceHeadValue>, SourceTail>:
        ByNameFieldPlucker<TargetHeadName, PluckSourceHeadNameIndex>, // pluck a value out of the Source by the Head Target Name
    <HCons<Field<SourceHeadName, SourceHeadValue>, SourceTail> as ByNameFieldPlucker<
        TargetHeadName,
        PluckSourceHeadNameIndex,
    >>::TargetValue: Transmogrifier<TargetHeadValue, TransMogSourceHeadValueIndices>, // the value we pluck out needs to be able to be transmogrified to the Head Target Value type
    <HCons<Field<SourceHeadName, SourceHeadValue>, SourceTail> as ByNameFieldPlucker<
        TargetHeadName,
        PluckSourceHeadNameIndex,
    >>::Remainder: Transmogrifier<TargetTail, TransMogTailIndices>, // the remainder from plucking out the Head Target Name must be able to be transmogrified to the target tail, utilising the other remaining indices
{
    fn transmogrify(self) -> HCons<Field<TargetHeadName, TargetHeadValue>, TargetTail> {
        let (source_field_for_head_target_name, remainder) = self.pluck_by_name();
        let transmogrified_head_field: TargetHeadValue =
            source_field_for_head_target_name.value.transmogrify();
        let as_field: Field<TargetHeadName, TargetHeadValue> = field_with_name(
            source_field_for_head_target_name.name,
            transmogrified_head_field,
        );
        HCons {
            head: as_field,
            tail: remainder.transmogrify(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::chars::*;
    use super::*;

    // Set up some aliases
    #[allow(non_camel_case_types)]
    type abc = (a, b, c);
    #[allow(non_camel_case_types)]
    type name = (n, a, m, e);
    #[allow(non_camel_case_types)]
    type age = (a, g, e);
    #[allow(non_camel_case_types)]
    type is_admin = (i, s, __, a, d, m, i, n);
    #[allow(non_camel_case_types)]
    type inner = (i, n, n, e, r);

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

    #[test]
    fn test_keys() {
        #[derive(PartialEq, Eq, Debug)]
        struct Meh<T>(PhantomData<T>);
        type ExpectedKeys = Hlist![name, age];
        type ObservedKeys = <Hlist![Field<name, &'static str>, Field<age, isize>] as Keys>::Out;
        let expected = Meh::<ExpectedKeys>(PhantomData);
        let observed = Meh::<ObservedKeys>(PhantomData);
        assert_eq!(expected, observed);
    }

    #[test]
    fn test_pluck_by_key() {
        type Record = Hlist![Field<name, &'static str>, Field<age, isize>];
        let record: Record = hlist![field!(name, "joe"), field!((a, g, e), 3)];
        let (value, remainder) = <Record as ByKeyPlucker<name, _>>::pluck_by_key(record);
        assert_eq!(value, "joe");
        assert_eq!(remainder, hlist![field!(age, 3)]);
    }

    #[test]
    fn test_transmogrify_simple_identity() {
        let one: i32 = 1;
        let one_again: i32 = one.transmogrify();
        assert_eq!(one, one_again);
    }
    //
    //    #[test]
    //    fn test_transmogrify_hnil_identity() {
    //        let hnil_again: HNil = HNil.transmogrify();
    //        assert_eq!(HNil, hnil_again);
    //    }

    #[test]
    fn test_transmogrify_hcons_identity() {
        let hcons = hlist!(1, 2f32, true);
        let again: Hlist![isize, f32, bool] = hcons.transmogrify();
        assert_eq!(again, hlist!(1, 2f32, true));
    }

    #[test]
    fn test_transmogrify_hcons_sculpting_super_simple() {
        type Source = Hlist![Field<name, &'static str>, Field<age, i32>, Field<is_admin, bool>];
        type Target = Hlist![Field<age, i32>];
        let hcons: Source = hlist!(field!(name, "joe"), field!(age, 3), field!(is_admin, true));
        let t_hcons: Target = hcons.transmogrify();
        assert_eq!(t_hcons, hlist!(field!(age, 3)));
    }

    #[test]
    fn test_transmogrify_hcons_sculpting_somewhat_simple() {
        type Source = Hlist![Field<name, &'static str>, Field<age, i32>, Field<is_admin, bool>];
        type Target = Hlist![Field<is_admin, bool>, Field<name, &'static str>];
        let hcons: Source = hlist!(field!(name, "joe"), field!(age, 3), field!(is_admin, true));
        let t_hcons: Target = hcons.transmogrify();
        assert_eq!(t_hcons, hlist!(field!(is_admin, true), field!(name, "joe")));
    }

    //    #[test]
    //    fn test_transmogrify_hcons_sculpting_required_simple() {
    //        type Source = Hlist![Field<name, &'static str>, Field<age, i32>, Field<is_admin, bool>];
    //        type Target = Hlist![Field<is_admin, bool>, Field<name, &'static str>, Field<age, i32>];
    //        let hcons: Source = hlist!(field!(name, "joe"), field!(age, 3), field!(is_admin, true));
    ////        let t_hcons: Target = <Source as Transmogrifier<
    ////            Target,
    ////            HCons<
    ////                DoTransmog<There<There<Here>>, HNil>,
    ////                HCons<
    ////                    DoTransmog<Here, HNil>,
    ////                    HCons<DoTransmog<Here, HNil>, HNil>,
    ////                >,
    ////            >,
    ////        >>::transmogrify(hcons);
    //        let t_hcons: Target = hcons.transmogrify();
    //        assert_eq!(
    //            t_hcons,
    //            hlist!(field!(is_admin, true), field!(name, "joe"), field!(age, 3))
    //        );
    //    }

    //    #[test]
    //    fn test_transmogrify_hcons_sculpting_required_simple_recursive() {
    //        let hcons = hlist!(
    //            field!(name, "joe"),
    //            field!(inner, hlist!(field!(age, 3), field!(is_admin, true)))
    //        );
    //        let t_hcons: Hlist![
    //            Field<inner,
    //                Hlist![
    //                    Field<is_admin, bool>,
    //                    Field<age, isize>
    //                ]
    //             >,
    //             Field<name, &str>] = hcons.transmogrify();
    //        assert_eq!(
    //            t_hcons,
    //            hlist!(
    //                field_with_name::<inner, Hlist![Field<is_admin, bool>, Field<age, isize>]>(
    //                    ZIPPED_FIELD,
    //                    hlist!(
    //                        field_with_name::<is_admin, bool>(ZIPPED_FIELD, true),
    //                        field_with_name::<age, isize>(ZIPPED_FIELD, 3)
    //                    ),
    //                ),
    //                field_with_name::<name, &str>(ZIPPED_FIELD, "joe")
    //            )
    //        );
    //    }
}
