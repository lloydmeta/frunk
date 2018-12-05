//! Holds models, traits, and logic for generic traversal of models
//!
//! ```
//! #[macro_use] extern crate frunk;
//! #[macro_use] extern crate frunk_core; // required when using custom derives
//! # fn main() {
//! # use frunk_core::hlist::*;
//! # use frunk_core::labelled::chars::*;
//! # use frunk_core::path::*;
//! # use frunk::prelude::*;
//! #[allow(non_camel_case_types)]
//! type name = (n, a, m, e);
//! type address = (a, d, d, r, e, s, s);
//!
//! #[derive(LabelledGeneric)]
//! struct Address<'a> {
//!     name: &'a str,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct User<'a> {
//!     name: &'a str,
//!     address: Address<'a>,
//! }
//!
//! let u = User {
//!   name: "Joe",
//!   address: Address { name: "blue pond" },
//! };
//!
//! {
//! let name_path = Path::<Hlist![name]>::new();
//! let traversed_name = name_path.get(&u);
//! assert_eq!(*traversed_name, "Joe");
//! }
//!
//! let address_name_path = Path::<HCons<address, Path<HCons<name, HNil>>>>::new();
//! let traversed_address_name = address_name_path.get(u);
//! assert_eq!(traversed_address_name, "blue pond");
//! # }
//! ```

use super::hlist::*;
use super::labelled::*;

use std::marker::PhantomData;

pub struct Path<T>(PhantomData<T>);

impl<T> Path<T> {
    /// Creates a new Path
    pub fn new() -> Path<T> {
        Path(PhantomData)
    }

    /// Gets something using the current path
    pub fn get<V, I, O>(&self, o: O) -> V
    where
        O: PathTraverser<Self, I, TargetValue = V>,
    {
        o.get()
    }
}

/// Trait for traversing based on Path
pub trait PathTraverser<Path, Indices> {
    type TargetValue;

    /// Returns a pair consisting of the value pointed to by the target key and the remainder.
    #[inline(always)]
    fn get(self) -> Self::TargetValue;
}

// For the case where we have no more field names to traverse
impl<Name, PluckIndex, Traversable> PathTraverser<Path<HCons<Name, HNil>>, PluckIndex>
    for Traversable
where
    Traversable: IntoLabelledGeneric,
    <Traversable as IntoLabelledGeneric>::Repr: ByNameFieldPlucker<Name, PluckIndex>,
{
    type TargetValue = <<Traversable as IntoLabelledGeneric>::Repr as ByNameFieldPlucker<
        Name,
        PluckIndex,
    >>::TargetValue;

    fn get(self) -> Self::TargetValue {
        self.into().pluck_by_name().0.value
    }
}

// For the case where a path nests another path (e.g. nested traverse)
impl<HeadName, TailNames, HeadPluckIndex, TailPluckIndices, Traversable>
    PathTraverser<Path<HCons<HeadName, Path<TailNames>>>, HCons<HeadPluckIndex, TailPluckIndices>>
    for Traversable
where
    Traversable: IntoLabelledGeneric,
    <Traversable as IntoLabelledGeneric>::Repr: ByNameFieldPlucker<HeadName, HeadPluckIndex>,
    <<Traversable as IntoLabelledGeneric>::Repr as ByNameFieldPlucker<HeadName, HeadPluckIndex>>::TargetValue:
        PathTraverser<Path<TailNames>, TailPluckIndices>,
{
    type TargetValue = <<<Traversable as IntoLabelledGeneric>::Repr as ByNameFieldPlucker<HeadName, HeadPluckIndex>>::TargetValue as
    PathTraverser<Path<TailNames>, TailPluckIndices>>::TargetValue ;

    fn get(self) -> Self::TargetValue {
        self.into().pluck_by_name().0.value.get()
    }
}
