//! Traits that provide generic functionality for multiple types in frunk

/// An alternative to AsRef that does not force the reference type to be a pointer itself.
///
/// This lets us create implementations for our recursive traits that take the resulting
/// Output reference type, without having to deal with strange, spurious overflows
/// that sometimes occur when trying to implement a trait for &'a T (see this comment:
/// <https://github.com/lloydmeta/frunk/pull/106#issuecomment-377927198>)
///
/// This functionality is also provided as an inherent method [on HLists] and [on Coproducts].
/// However, you may find this trait useful in generic contexts.
///
/// [on HLists]: ../hlist/struct.HCons.html#method.to_ref
/// [on Coproducts]: ../coproduct/enum.Coproduct.html#method.to_ref
pub trait ToRef<'a> {
    type Output;

    fn to_ref(&'a self) -> Self::Output;
}

/// An alternative to `AsMut` that does not force the reference type to be a pointer itself.
///
/// This parallels [`ToRef`]; see it for more information.
///
/// [`ToRef`]: trait.ToRef.html
pub trait ToMut<'a> {
    type Output;

    fn to_mut(&'a mut self) -> Self::Output;
}

/// Trait that allows for reversing a given data structure.
///
/// Implemented for HLists.
///
/// This functionality is also provided as an [inherent method].
/// However, you may find this trait useful in generic contexts.
///
/// [inherent method]: ../hlist/struct.HCons.html#method.into_reverse
pub trait IntoReverse {
    type Output;

    /// Reverses a given data structure.
    fn into_reverse(self) -> Self::Output;
}

/// Wrapper type around a function for polymorphic maps and folds.
///
/// This is a thin generic wrapper type that is used to differentiate
/// between single-typed generic closures `F` that implements, say, `Fn(i8) -> bool`,
/// and a Poly-typed `F` that implements multiple Function types
/// via the [`Func`] trait. (say, `Func<i8, Output=bool>` and `Func<bool, Output=f32>`)
///
/// This is needed because there are completely generic impls for many of the
/// HList traits that take a simple unwrapped closure.
///
/// [`Func`]: trait.Func.html
#[derive(Debug, Copy, Clone, Default)]
pub struct Poly<T>(pub T);

/// This is a simple, user-implementable alternative to `Fn`.
///
/// Might not be necessary if/when Fn(Once, Mut) traits are implementable
/// in stable Rust
pub trait Func<Input> {
    type Output;

    /// Call the `Func`.
    ///
    /// Notice that this does not take a self argument, which in turn means `Func`
    /// cannot effectively close over a context. This decision trades power for convenience;
    /// a three-trait `Fn` heirarchy like that in std provides a great deal of power in a
    /// small fraction of use-cases, but it also comes at great expanse to the other 95% of
    /// use cases.
    fn call(i: Input) -> Self::Output;
}
