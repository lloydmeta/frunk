//! Types used for indexing into HLists and coproducts.
//!
//! frunk frequently uses phantom index types as a technique to avoid
//! overlapping impls for some traits.
//!
//! Currently, `Index` type parameters in traits are not ever really intended
//! to be selected by the user, and are instead simply solved for by type
//! inference wherever the compiler can see that there is a unique solution.
//! Therefore, you don't really have much of a reason to use the things in this
//! module.
//!
//! **...yet.** `;)`

use std::marker::PhantomData;

// Largely lifted from https://github.com/Sgeo/hlist/blob/master/src/lib.rs#L30

/// Used as an index into an `HList`.
///
/// `Here` is 0, pointing to the head of the HList.
///
/// Users should normally allow type inference to create this type.
pub struct Here {
    _priv: (),
}

/// Used as an index into an `HList`.
///
/// `There<T>` is 1 + `T`.
///
/// Users should normally allow type inference to create this type.
pub struct There<T> {
    _marker: PhantomData<T>,
}

/// An index denoting that `Suffix` is just that.
pub struct Suffixed<Suffix> {
    _marker: PhantomData<Suffix>,
}
