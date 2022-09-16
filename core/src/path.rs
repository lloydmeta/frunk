//! Holds models, traits, and logic for generic traversal of models
//!
//! ```
//! # use frunk_proc_macros::path;
//! # use frunk_derives::LabelledGeneric;
//! # fn main() {
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
//!
//! There is also a Path! type macro that allows you to declare type constraints for
//! shape-dependent functions on LabelledGeneric types.
//!
//! ```
//! # use frunk_proc_macros::{path, Path};
//! # use frunk_core::path::PathTraverser;
//! # use frunk_derives::LabelledGeneric;
//! # fn main() {
//! #[derive(LabelledGeneric)]
//! struct Dog<'a> {
//!     name: &'a str,
//!     dimensions: Dimensions,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct Cat<'a> {
//!     name: &'a str,
//!     dimensions: Dimensions,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct Dimensions {
//!     height: usize,
//!     width: usize,
//!     unit: SizeUnit,
//! }
//!
//! #[derive(Debug)]
//! enum SizeUnit {
//!     Cm,
//!     Inch,
//! }
//!
//! let dog = Dog {
//!     name: "Joe",
//!     dimensions: Dimensions {
//!         height: 10,
//!         width: 5,
//!         unit: SizeUnit::Inch,
//!     },
//! };
//!
//! let cat = Cat {
//!     name: "Schmoe",
//!     dimensions: Dimensions {
//!         height: 7,
//!         width: 3,
//!         unit: SizeUnit::Cm,
//!     },
//! };
//!
//! // Prints height as long as `A` has the right "shape" (e.g.
//! // has `dimensions.height: usize` and `dimension.unit: SizeUnit)
//! fn print_height<'a, A, HeightIdx, UnitIdx>(obj: &'a A) -> String
//! where
//!     &'a A: PathTraverser<Path!(dimensions.height), HeightIdx, TargetValue = &'a usize>
//!         + PathTraverser<Path!(dimensions.unit), UnitIdx, TargetValue = &'a SizeUnit>,
//! {
//!     format!(
//!         "Height [{} {:?}]",
//!         path!(dimensions.height).get(obj),
//!         path!(dimensions.unit).get(obj)
//!     )
//! }
//!
//! assert_eq!(print_height(&dog), "Height [10 Inch]".to_string());
//! assert_eq!(print_height(&cat), "Height [7 Cm]".to_string());
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

impl<T> Default for Path<T> {
    fn default() -> Self {
        Self::new()
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
    #[allow(clippy::type_complexity)]
    type Output = Path<HCons<Name, <Path<Tail> as Add<Path<RHSParam>>>::Output>>;

    #[inline(always)]
    fn add(self, _: Path<RHSParam>) -> <Self as Add<Path<RHSParam>>>::Output {
        Path::new()
    }
}
