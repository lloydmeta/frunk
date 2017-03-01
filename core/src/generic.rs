/// A trait that converts from a type to a generic representation
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
///     first_name: "Joe",
///     last_name: "Blow",
///     age: 30,
/// };
/// let d_person: DomainPerson = convert_from(a_person); // done
/// ```
pub trait Generic<Repr> {
    /// Go from something to Repr
    fn into(self) -> Repr;

    /// Go from Repr to something
    fn from(r: Repr) -> Self;
}

/// Given a generic Representation of an A, returns A
pub fn from_generic<A, Repr>(gen: Repr) -> A
    where A: Generic<Repr>
{
    <A as Generic<Repr>>::from(gen)
}

/// Given an A, returns its generic Representation
pub fn into_generic<A, Repr>(a: A) -> Repr
    where A: Generic<Repr>
{
    <A as Generic<Repr>>::into(a)
}

/// Converts one type into another assuming they have the same generic Representation
pub fn convert_from<A, B, Repr>(a: A) -> B
    where A: Generic<Repr>,
          B: Generic<Repr>
{
    let repr = <A as Generic<Repr>>::into(a);
    <B as Generic<Repr>>::from(repr)
}
