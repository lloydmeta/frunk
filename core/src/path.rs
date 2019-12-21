//! Holds models, traits, and logic for generic traversal of models
//!
//! ```
//! #[macro_use] extern crate frunk;
//! #[macro_use] extern crate frunk_core; // required when using custom derives
//! # extern crate frunk_proc_macros;
//! # use frunk_proc_macros::path;
//! # fn main() {//!
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
//! let name_path = path!(name);
//!
//! {
//! let traversed_name = name_path.get(&u);
//! assert_eq!(*traversed_name, "Joe");
//! }
//!
//! // You can also **add** paths together
//! let address_path = path!(address);
//! let address_name_path = address_path + name_path;
//!
//! let traversed_address_name = address_name_path.get(u);
//! assert_eq!(traversed_address_name, "blue pond");
//! # }
//! ```

use super::hlist::*;
use super::labelled::*;

use std::marker::PhantomData;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn get(self) -> Self::TargetValue {
        self.into().pluck_by_name().0.value.get()
    }
}

// For the simple case of adding to a single path
impl<Name, RHSParam> Add<Path<RHSParam>> for Path<HCons<Name, HNil>> {
    type Output = Path<HCons<Name, Path<RHSParam>>>;

    #[inline(always)]
    fn add(self, _: Path<RHSParam>) -> Self::Output {
        Path::new()
    }
}

impl<Name, Tail, RHSParam> Add<Path<RHSParam>> for Path<HCons<Name, Path<Tail>>>
where
    Path<Tail>: Add<Path<RHSParam>>,
{
    type Output = Path<HCons<Name, <Path<Tail> as Add<Path<RHSParam>>>::Output>>;

    #[inline(always)]
    fn add(self, _: Path<RHSParam>) -> <Self as Add<Path<RHSParam>>>::Output {
        Path::new()
    }
}
