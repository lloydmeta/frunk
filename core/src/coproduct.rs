//! Module that holds Coproduct data structures, traits, and implementations
//!
//! Think of "Coproduct" as ad-hoc enums; allowing you to do something like this
//!
//! ```
//! #[macro_use]
//! extern crate frunk;
//!
//! # fn main() {
//! // For simplicity, assign our Coproduct type to a type alias
//! // This is purely optional.
//! type I32Bool = Coprod!(i32, bool);
//! // Inject things into our Coproduct type
//! let co1 = I32Bool::inject(3);
//! let co2 = I32Bool::inject(true);
//!
//! // Getting stuff
//! let get_from_1a: Option<&i32> = co1.get();
//! let get_from_1b: Option<&bool> = co1.get();
//! assert_eq!(get_from_1a, Some(&3));
//! assert_eq!(get_from_1b, None);
//!
//! let get_from_2a: Option<&i32> = co2.get();
//! let get_from_2b: Option<&bool> = co2.get();
//! assert_eq!(get_from_2a, None);
//! assert_eq!(get_from_2b, Some(&true));
//!
//! // *Taking* stuff (by value)
//! let take_from_1a: Option<i32> = co1.take();
//! assert_eq!(take_from_1a, Some(3));
//!
//! // Or with a Result
//! let uninject_from_1a: Result<i32, _> = co1.uninject();
//! let uninject_from_1b: Result<bool, _> = co1.uninject();
//! assert_eq!(uninject_from_1a, Ok(3));
//! assert!(uninject_from_1b.is_err());
//! # }
//! ```
//!
//! Or, if you want to "fold" over all possible values of a coproduct
//!
//! ```
//! # #[macro_use] extern crate frunk;
//! # fn main() {
//! # type I32Bool = Coprod!(i32, bool);
//! # let co1 = I32Bool::inject(3);
//! # let co2 = I32Bool::inject(true);
//! // In the below, we use unreachable!() to make it obvious hat we know what type of
//! // item is inside our coproducts co1 and co2 but in real life, you should be writing
//! // complete functions for all the cases when folding coproducts
//! //
//! // to_ref borrows every item so that we can fold without consuming the coproduct.
//! assert_eq!(
//!     co1.to_ref().fold(hlist![|&i| format!("i32 {}", i),
//!                              |&b| unreachable!() /* we know this won't happen for co1 */ ]),
//!     "i32 3".to_string());
//! assert_eq!(
//!     co2.to_ref().fold(hlist![|&i| unreachable!() /* we know this won't happen for co2 */,
//!                              |&b| String::from(if b { "t" } else { "f" })]),
//!     "t".to_string());
//!
//! // Here, we use the poly_fn! macro to declare a polymorphic function to avoid caring
//! // about the order in which declare handlers for the types in our coproduct
//! let folded = co1.fold(
//!       poly_fn![
//!         |_b: bool| -> String { unreachable!() }, /* we know this won't happen for co1 */
//!         |i:  i32 | -> String { format!("i32 {}", i) },
//!       ]
//!      );
//! assert_eq!(folded, "i32 3".to_string());
//! # }
//! ```

use core::mem::ManuallyDrop;

use crate::hlist::{HCons, HNil};
use crate::indices::{Here, There};
use crate::traits::{Func, Poly, ToMut, ToRef};

/// Coproduct that does not know what variant it currently holds.
/// Note: contents must be manually dropped!

// C representation is required to ensure that the same variant is represented
// in exactly the same way in all coproducts.
// See https://rust-lang.github.io/unsafe-code-guidelines/layout/unions.html
#[repr(C)]
pub union UntaggedCoproduct<H, T> {
    head: ManuallyDrop<H>,
    tail: ManuallyDrop<T>,
}

/// Phantom type for signature purposes only (has no value)
///
/// Used by the macro to terminate the Coproduct type signature
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CNil {}

pub trait IndexedDrop {
    fn idrop(&mut self, i: u32);
}

impl IndexedDrop for CNil {
    fn idrop(&mut self, _: u32) {}
}

impl<H, T: IndexedDrop> IndexedDrop for UntaggedCoproduct<H, T> {
    fn idrop(&mut self, i: u32) {
        if i == 0 {
            unsafe { ManuallyDrop::drop(&mut self.head) }
        } else {
            unsafe { &mut self.tail }.idrop(i - 1)
        }
    }
}

pub trait Injector<InjectType, Index> {
    fn inject(to_insert: InjectType) -> Self;
}

impl<I, Tail: IndexedDrop> Injector<I, Here> for UntaggedCoproduct<I, Tail> {
    fn inject(to_insert: I) -> Self {
        UntaggedCoproduct {
            head: ManuallyDrop::new(to_insert),
        }
    }
}

impl<Head, I, Tail: IndexedDrop, TailIndex> Injector<I, There<TailIndex>>
    for UntaggedCoproduct<Head, Tail>
where
    Tail: Injector<I, TailIndex>,
{
    fn inject(to_insert: I) -> Self {
        let tail_inserted = <Tail as Injector<I, TailIndex>>::inject(to_insert);
        UntaggedCoproduct {
            tail: ManuallyDrop::new(tail_inserted),
        }
    }
}

pub trait Selector<S, I> {
    /// # Safety
    /// Getting a variant that isn't the active variant
    /// will result in undefined behaviour.
    unsafe fn get(&self) -> &S;
}

impl<Head, Tail: IndexedDrop> Selector<Head, Here> for UntaggedCoproduct<Head, Tail> {
    unsafe fn get(&self) -> &Head {
        &self.head
    }
}

impl<Head, FromTail, Tail: IndexedDrop, TailIndex> Selector<FromTail, There<TailIndex>>
    for UntaggedCoproduct<Head, Tail>
where
    Tail: Selector<FromTail, TailIndex>,
{
    unsafe fn get(&self) -> &FromTail {
        self.tail.get()
    }
}

/// Trait for retrieving a coproduct element by type
///
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::take`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. If you have a Coproduct of known type,
/// then `co.take()` should "just work" even without the trait.
///
/// [`Coproduct::take`]: enum.Coproduct.html#method.take
pub trait Taker<S, I> {
    /// Retrieve an element from a coproduct by type, ignoring all others.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.take
    /// # Safety
    /// If the desired type is not the active variant of the coproduct,
    /// this function produces undefined behaviour.
    unsafe fn take(self) -> S;
}

impl<Head, Tail: IndexedDrop> Taker<Head, Here> for UntaggedCoproduct<Head, Tail> {
    unsafe fn take(self) -> Head {
        ManuallyDrop::into_inner(self.head)
    }
}

impl<Head, FromTail, Tail: IndexedDrop, TailIndex> Taker<FromTail, There<TailIndex>>
    for UntaggedCoproduct<Head, Tail>
where
    Tail: Taker<FromTail, TailIndex>,
{
    unsafe fn take(self) -> FromTail {
        ManuallyDrop::into_inner(self.tail).take()
    }
}

/// Enum type representing a Coproduct. Think of this as a Result, but capable
/// of supporting any arbitrary number of types instead of just 2.
///
/// To construct a Coproduct, you would typically declare a type using the `Coprod!` type
/// macro and then use the `inject` method.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate frunk;
/// # fn main() {
/// type I32Bool = Coprod!(i32, bool);
/// let co1 = I32Bool::inject(3);
/// let get_from_1a: Option<&i32> = co1.get();
/// let get_from_1b: Option<&bool> = co1.get();
/// assert_eq!(get_from_1a, Some(&3));
/// assert_eq!(get_from_1b, None);
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coproduct<Untagged: IndexedDrop> {
    tag: u32,
    untagged: Untagged,
}

pub fn absurd(x: Coproduct<CNil>) -> CNil {
    match x.untagged {}
}

/*
impl<T: IndexedDrop> Drop for Coproduct<T> {
    fn drop(&mut self) {
        self.untagged.idrop(self.tag)
    }
}
*/

/// Used by the Coprod! macro to remove the wrapper from a coproduct.
pub trait Untagger {
    type Untagged;
}

impl<T: IndexedDrop> Untagger for Coproduct<T> {
    type Untagged = T;
}

impl<T: IndexedClone + IndexedDrop> Clone for Coproduct<T> {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag,
            untagged: unsafe { self.untagged.iclone(self.tag) },
        }
    }
}

trait IndexedClone {
    /// # Safety
    /// The argument `i` must be the index of the active variant
    /// of the UntaggedCoproduct.
    unsafe fn iclone(&self, i: u32) -> Self;
}

impl<H: Clone, T: IndexedClone> IndexedClone for UntaggedCoproduct<H, T> {
    unsafe fn iclone(&self, i: u32) -> Self {
        if i == 0 {
            UntaggedCoproduct {
                head: self.head.clone(),
            }
        } else {
            UntaggedCoproduct {
                tail: ManuallyDrop::new(self.tail.iclone(i - 1)),
            }
        }
    }
}

impl IndexedClone for CNil {
    unsafe fn iclone(&self, _: u32) -> Self {
        match *self {}
    }
}

impl<H: Copy, T: Copy> Clone for UntaggedCoproduct<H, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<H: Copy, T: Copy> Copy for UntaggedCoproduct<H, T> {}

impl<Untagged: Copy + IndexedClone + IndexedDrop> Copy for Coproduct<Untagged> {}

impl<H: PartialEq, T: Comparer + IndexedDrop> PartialEq for Coproduct<UntaggedCoproduct<H, T>> {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
            && unsafe { UntaggedCoproduct::compare(self.tag, &self.untagged, &other.untagged) }
    }
}

trait Comparer {
    unsafe fn compare(i: u32, a: &Self, b: &Self) -> bool;
}

impl<H: PartialEq, T: Comparer> Comparer for UntaggedCoproduct<H, T> {
    unsafe fn compare(i: u32, a: &Self, b: &Self) -> bool {
        if i == 0 {
            a.head == b.head
        } else {
            T::compare(i - 1, &a.tail, &b.tail)
        }
    }
}

impl Comparer for CNil {
    unsafe fn compare(_: u32, a: &Self, _: &Self) -> bool {
        match *a {}
    }
}

impl<H, T: IndexedDrop> std::fmt::Debug for Coproduct<UntaggedCoproduct<H, T>> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Coproduct")
            .field("tag", &self.tag)
            .field("untagged", &"asd")
            .finish()
    }
}

pub trait Counter {
    fn count() -> u32;
}

impl Counter for Here {
    fn count() -> u32 {
        0
    }
}

impl<N> Counter for There<N>
where
    N: Counter,
{
    fn count() -> u32 {
        N::count() + 1
    }
}

pub trait Drop1<Index> {
    type Remainder: IndexedDrop;
}

impl<H, T: IndexedDrop> Drop1<Here> for UntaggedCoproduct<H, T> {
    type Remainder = T;
}

impl<Hd, Tl: IndexedDrop, N> Drop1<There<N>> for UntaggedCoproduct<Hd, Tl>
where
    Tl: Drop1<N>,
{
    type Remainder = UntaggedCoproduct<Hd, Tl::Remainder>;
}

/// # Safety
/// This function is only safe to call if the variant removed
/// is not active. Otherwise it can produce a boolean with
/// the value 3, for example.
unsafe fn remove<H, T, Index>(
    cp: UntaggedCoproduct<H, T>,
) -> <UntaggedCoproduct<H, T> as Drop1<Index>>::Remainder
where
    UntaggedCoproduct<H, T>: Drop1<Index>,
{
    #[repr(C)]
    union Transmuter<T, Index>
    where
        T: Drop1<Index>,
    {
        before: ManuallyDrop<T>,
        after: ManuallyDrop<T::Remainder>,
    }
    ManuallyDrop::into_inner(
        Transmuter {
            before: ManuallyDrop::new(cp),
        }
        .after,
    )
}

// Inherent methods
impl<Untagged: IndexedDrop> Coproduct<Untagged> {
    /// Instantiate a coproduct from an element.
    ///
    /// This is generally much nicer than nested usage of `Coproduct::{Inl, Inr}`.
    /// The method uses a trick with type inference to automatically build the correct variant
    /// according to the input type.
    ///
    /// In standard usage, the `Index` type parameter can be ignored,
    /// as it will typically be solved for using type inference.
    ///
    /// # Rules
    ///
    /// If the type does not appear in the coproduct, the conversion is forbidden.
    ///
    /// If the type appears multiple times in the coproduct, type inference will fail.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// use frunk::Coproduct;
    ///
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// // Constructing coproducts using inject:
    /// let co1_nice: I32F32 = Coproduct::inject(1i32);
    /// let co2_nice: I32F32 = Coproduct::inject(42f32);
    ///
    /// // Compare this to the "hard way":
    /// let co1_ugly: I32F32 = Coproduct::here(1i32);
    /// let co2_ugly: I32F32 = Coproduct::there(Coproduct::here(42f32));
    ///
    /// assert_eq!(co1_nice, co1_ugly);
    /// assert_eq!(co2_nice, co2_ugly);
    ///
    /// // Feel free to use `inject` on a type alias, or even directly on the
    /// // `Coprod!` macro. (the latter requires wrapping the type in `<>`)
    /// let _ = I32F32::inject(42f32);
    /// let _ = <Coprod!(i32, f32)>::inject(42f32);
    ///
    /// // You can also use a turbofish to specify the type of the input when
    /// // it is ambiguous (e.g. an empty `vec![]`).
    /// // The Index parameter should be left as `_`.
    /// type Vi32Vf32 = Coprod!(Vec<i32>, Vec<f32>);
    /// let _: Vi32Vf32 = Coproduct::inject::<Vec<i32>, _>(vec![]);
    /// # }
    /// ```
    #[inline(always)]
    pub fn inject<T, Index>(to_insert: T) -> Self
    where
        Index: Counter,
        Untagged: Injector<T, Index>,
    {
        Coproduct {
            tag: Index::count(),
            untagged: Untagged::inject(to_insert),
        }
    }

    /// Borrow an element from a coproduct by type.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// // You can let type inference find the desired type:
    /// let co1 = I32F32::inject(42f32);
    /// let co1_as_i32: Option<&i32> = co1.get();
    /// let co1_as_f32: Option<&f32> = co1.get();
    /// assert_eq!(co1_as_i32, None);
    /// assert_eq!(co1_as_f32, Some(&42f32));
    ///
    /// // You can also use turbofish syntax to specify the type.
    /// // The Index parameter should be left as `_`.
    /// let co2 = I32F32::inject(1i32);
    /// assert_eq!(co2.get::<i32, _>(), Some(&1));
    /// assert_eq!(co2.get::<f32, _>(), None);
    /// # }
    /// ```
    #[inline(always)]
    pub fn get<S, Index>(&self) -> Option<&S>
    where
        Index: Counter,
        Untagged: Selector<S, Index>,
    {
        if self.tag == Index::count() {
            Some(unsafe { self.untagged.get() })
        } else {
            None
        }
    }

    /// Retrieve an element from a coproduct by type, ignoring all others.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// // You can let type inference find the desired type:
    /// let co1 = I32F32::inject(42f32);
    /// let co1_as_i32: Option<i32> = co1.take();
    /// let co1_as_f32: Option<f32> = co1.take();
    /// assert_eq!(co1_as_i32, None);
    /// assert_eq!(co1_as_f32, Some(42f32));
    ///
    /// // You can also use turbofish syntax to specify the type.
    /// // The Index parameter should be left as `_`.
    /// let co2 = I32F32::inject(1i32);
    /// assert_eq!(co2.take::<i32, _>(), Some(1));
    /// assert_eq!(co2.take::<f32, _>(), None);
    /// # }
    /// ```
    #[inline(always)]
    pub fn take<T, Index: Counter>(self) -> Option<T>
    where
        Untagged: Taker<T, Index>,
    {
        if self.tag == Index::count() {
            Some(unsafe { self.untagged.take() })
        } else {
            None
        }
    }
}

impl<Head, Tail: IndexedDrop> Coproduct<UntaggedCoproduct<Head, Tail>> {
    pub fn here(x: Head) -> Self {
        Self {
            tag: 0,
            untagged: UntaggedCoproduct {
                head: ManuallyDrop::new(x),
            },
        }
    }
    pub fn there(x: Coproduct<Tail>) -> Self {
        Self {
            tag: x.tag + 1,
            untagged: UntaggedCoproduct {
                tail: ManuallyDrop::new(x.untagged),
            },
        }
    }
    /// Attempt to extract a value from a coproduct (or get the remaining possibilities).
    ///
    /// By chaining calls to this, one can exhaustively match all variants of a coproduct.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// type I32F32 = Coprod!(i32, f32);
    /// type I32 = Coprod!(i32); // remainder after uninjecting f32
    /// type F32 = Coprod!(f32); // remainder after uninjecting i32
    ///
    /// let co1 = I32F32::inject(42f32);
    ///
    /// // You can let type inference find the desired type.
    /// let co1 = I32F32::inject(42f32);
    /// let co1_as_i32: Result<i32, F32> = co1.uninject();
    /// let co1_as_f32: Result<f32, I32> = co1.uninject();
    /// assert_eq!(co1_as_i32, Err(F32::inject(42f32)));
    /// assert_eq!(co1_as_f32, Ok(42f32));
    ///
    /// // It is not necessary to annotate the type of the remainder:
    /// let res: Result<i32, _> = co1.uninject();
    /// assert!(res.is_err());
    ///
    /// // You can also use turbofish syntax to specify the type.
    /// // The Index parameter should be left as `_`.
    /// let co2 = I32F32::inject(1i32);
    /// assert_eq!(co2.uninject::<i32, _>(), Ok(1));
    /// assert_eq!(co2.uninject::<f32, _>(), Err(I32::inject(1)));
    /// # }
    /// ```
    ///
    /// Chaining calls for an exhaustive match:
    ///
    /// ```rust
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// // Be aware that this particular example could be
    /// // written far more succinctly using `fold`.
    /// fn handle_i32_f32(co: I32F32) -> &'static str {
    ///     // Remove i32 from the coproduct
    ///     let co = match co.uninject::<i32, _>() {
    ///         Ok(x) => return "integer!",
    ///         Err(co) => co,
    ///     };
    ///
    ///     // Remove f32 from the coproduct
    ///     let co = match co.uninject::<f32, _>() {
    ///         Ok(x) => return "float!",
    ///         Err(co) => co,
    ///     };
    ///
    ///     // Now co is empty
    ///     match frunk::coproduct::absurd(co) {}
    /// }
    ///
    /// assert_eq!(handle_i32_f32(I32F32::inject(3)), "integer!");
    /// assert_eq!(handle_i32_f32(I32F32::inject(3.0)), "float!");
    /// # }
    #[inline(always)]
    pub fn uninject<T, Index: Counter>(
        self,
    ) -> Result<T, Coproduct<<UntaggedCoproduct<Head, Tail> as Drop1<Index>>::Remainder>>
    where
        UntaggedCoproduct<Head, Tail>: Taker<T, Index> + Drop1<Index>,
    {
        if self.tag == Index::count() {
            Ok(unsafe { self.untagged.take() })
        } else {
            let rem = unsafe { remove(self.untagged) };
            Err(if self.tag < Index::count() {
                Coproduct {
                    tag: self.tag,
                    untagged: rem,
                }
            } else {
                Coproduct {
                    tag: self.tag - 1,
                    untagged: rem,
                }
            })
        }
    }

    fn take_head(self) -> Result<Head, Coproduct<Tail>> {
        if self.tag == 0 {
            Ok(ManuallyDrop::into_inner(unsafe { self.untagged.head }))
        } else {
            Err(Coproduct {
                tag: self.tag - 1,
                untagged: ManuallyDrop::into_inner(unsafe { self.untagged.tail }),
            })
        }
    }
}

impl<Untagged: IndexedDrop> Coproduct<Untagged> {
    /// Extract a subset of the possible types in a coproduct (or get the remaining possibilities)
    ///
    /// This is basically [`uninject`] on steroids.  It lets you remove a number
    /// of types from a coproduct at once, leaving behind the remainder in an `Err`.
    /// For instance, one can extract `Coprod!(C, A)` from `Coprod!(A, B, C, D)`
    /// to produce `Result<Coprod!(C, A), Coprod!(B, D)>`.
    ///
    /// Each type in the extracted subset is required to be part of the input coproduct.
    ///
    /// [`uninject`]: #method.uninject
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// use ::frunk::Coproduct;
    ///
    /// # fn main() {
    /// type I32BoolF32 = Coprod!(i32, bool, f32);
    /// type I32F32 = Coprod!(i32, f32);
    ///
    /// let co1 = I32BoolF32::inject(42_f32);
    /// let co2 = I32BoolF32::inject(true);
    ///
    /// let sub1: Result<Coprod!(i32, f32), _> = co1.subset();
    /// let sub2: Result<Coprod!(i32, f32), _> = co2.subset();
    /// assert!(sub1.is_ok());
    /// assert!(sub2.is_err());
    ///
    /// // Turbofish syntax for specifying the target subset is also supported.
    /// // The Indices parameter should be left to type inference using `_`.
    /// assert!(co1.subset::<Coprod!(i32, f32), _>().is_ok());
    /// assert!(co2.subset::<Coprod!(i32, f32), _>().is_err());
    ///
    /// // Order doesn't matter.
    /// assert!(co1.subset::<Coprod!(f32, i32), _>().is_ok());
    /// # }
    /// ```
    ///
    /// Like `uninject`, `subset` can be used for exhaustive matching,
    /// with the advantage that it can remove more than one type at a time:
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// use frunk::Coproduct;
    ///
    /// # fn main() {
    /// fn handle_stringly_things(co: Coprod!(&'static str, String)) -> String {
    ///     co.fold(hlist![
    ///         |s| format!("&str {}", s),
    ///         |s| format!("String {}", s),
    ///     ])
    /// }
    ///
    /// fn handle_countly_things(co: Coprod!(u32)) -> String {
    ///     co.fold(hlist![
    ///         |n| vec!["."; n as usize].concat(),
    ///     ])
    /// }
    ///
    /// fn handle_all(co: Coprod!(String, u32, &'static str)) -> String {
    ///     // co is currently Coprod!(String, u32, &'static str)
    ///     let co = match co.subset().map(handle_stringly_things) {
    ///         Ok(s) => return s,
    ///         Err(co) => co,
    ///     };
    ///
    ///     // Now co is Coprod!(u32).
    ///     let co = match co.subset().map(handle_countly_things) {
    ///         Ok(s) => return s,
    ///         Err(co) => co,
    ///     };
    ///
    ///     // Now co is empty.
    ///     match frunk::coproduct::absurd(co) {}
    /// }
    ///
    /// assert_eq!(handle_all(Coproduct::inject("hello")), "&str hello");
    /// assert_eq!(handle_all(Coproduct::inject(String::from("World!"))), "String World!");
    /// assert_eq!(handle_all(Coproduct::inject(4)), "....");
    /// # }
    /// ```
    #[inline(always)]
    pub fn subset<Targets, Indices>(
        self,
    ) -> Result<Targets, <Self as CoproductSubsetter<Targets, Indices>>::Remainder>
    where
        Self: CoproductSubsetter<Targets, Indices>,
    {
        CoproductSubsetter::subset(self)
    }

    /// Convert a coproduct into another that can hold its variants.
    ///
    /// This converts a coproduct into another one which is capable of holding each
    /// of its types. The most well-supported use-cases (i.e. those where type inference
    /// is capable of solving for the indices) are:
    ///
    /// * Reordering variants: `Coprod!(C, A, B) -> Coprod!(A, B, C)`
    /// * Embedding into a superset: `Coprod!(B, D) -> Coprod!(A, B, C, D, E)`
    /// * Coalescing duplicate inputs: `Coprod!(B, B, B, B) -> Coprod!(A, B, C)`
    ///
    /// and of course any combination thereof.
    ///
    /// # Rules
    ///
    /// If any type in the input does not appear in the output, the conversion is forbidden.
    ///
    /// If any type in the input appears multiple times in the output, type inference will fail.
    ///
    /// All of these rules fall naturally out of its fairly simple definition,
    /// which is equivalent to:
    ///
    /// ```text
    /// coprod.fold(hlist![
    ///     |x| Coproduct::inject(x),
    ///     |x| Coproduct::inject(x),
    ///             ...
    ///     |x| Coproduct::inject(x),
    /// ])
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// type I32BoolF32 = Coprod!(i32, bool, f32);
    /// type BoolI32 = Coprod!(bool, i32);
    ///
    /// let co = BoolI32::inject(true);
    /// let embedded: I32BoolF32 = co.embed();
    /// assert_eq!(embedded, I32BoolF32::inject(true));
    ///
    /// // Turbofish syntax for specifying the output type is also supported.
    /// // The Indices parameter should be left to type inference using `_`.
    /// let embedded = co.embed::<<I32BoolF32 as frunk::coproduct::Untagger>::Untagged, _>();
    /// assert_eq!(embedded, I32BoolF32::inject(true));
    /// # }
    /// ```
    #[inline(always)]
    pub fn embed<Targets: IndexedDrop, Indices>(self) -> Coproduct<Targets>
    where
        Self: CoproductEmbedder<Targets, Indices>,
    {
        CoproductEmbedder::embed(self)
    }

    /// Borrow each variant of the Coproduct.
    ///
    /// # Example
    ///
    /// Composing with `subset` to match a subset of variants without
    /// consuming the coproduct:
    ///
    /// ```
    /// # #[macro_use] extern crate frunk; fn main() {
    /// use frunk::Coproduct;
    ///
    /// let co: Coprod!(i32, bool, String) = Coproduct::inject(true);
    ///
    /// assert!(co.to_ref().subset::<Coprod!(&bool, &String), _>().is_ok());
    /// # }
    /// ```
    #[inline(always)]
    pub fn to_ref<'a>(&'a self) -> <Self as ToRef<'a>>::Output
    where
        Self: ToRef<'a>,
    {
        ToRef::to_ref(self)
    }

    /// Borrow each variant of the `Coproduct` mutably.
    ///
    /// # Example
    ///
    /// Composing with `subset` to match a subset of variants without
    /// consuming the coproduct:
    ///
    /// ```
    /// # #[macro_use] extern crate frunk; fn main() {
    /// use frunk::Coproduct;
    ///
    /// let mut co: Coprod!(i32, bool, String) = Coproduct::inject(true);
    ///
    /// assert!(co.to_mut().subset::<Coprod!(&mut bool, &mut String), _>().is_ok());
    /// # }
    /// ```
    #[inline(always)]
    pub fn to_mut<'a>(&'a mut self) -> <Self as ToMut<'a>>::Output
    where
        Self: ToMut<'a>,
    {
        ToMut::to_mut(self)
    }

    /// Use functions to transform a Coproduct into a single value.
    ///
    /// A variety of types are supported for the `Folder` argument:
    ///
    /// * An `hlist![]` of closures (one for each type, in order).
    /// * A single closure (for a Coproduct that is homogenous).
    /// * A single [`Poly`].
    ///
    /// [`Poly`]: ../traits/struct.Poly.html
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// type I32F32StrBool = Coprod!(i32, f32, bool);
    ///
    /// let co1 = I32F32StrBool::inject(3);
    /// let co2 = I32F32StrBool::inject(true);
    /// let co3 = I32F32StrBool::inject(42f32);
    ///
    /// let folder = hlist![|&i| format!("int {}", i),
    ///                     |&f| format!("float {}", f),
    ///                     |&b| (if b { "t" } else { "f" }).to_string()];
    ///
    /// assert_eq!(co1.to_ref().fold(folder), "int 3".to_string());
    /// # }
    /// ```
    ///
    /// Using a polymorphic function type has the advantage of not
    /// forcing you to care about the order in which you declare
    /// handlers for the types in your Coproduct.
    ///
    /// ```
    /// # #[macro_use] extern crate frunk;
    /// # fn main() {
    /// use frunk::{Poly, Func};
    ///
    /// type I32F32StrBool = Coprod!(i32, f32, bool);
    ///
    /// impl Func<i32> for P {
    ///     type Output = bool;
    ///     fn call(args: i32) -> Self::Output {
    ///         args > 100
    ///     }
    /// }
    /// impl Func<bool> for P {
    ///     type Output = bool;
    ///     fn call(args: bool) -> Self::Output {
    ///         args
    ///     }
    /// }
    /// impl Func<f32> for P {
    ///     type Output = bool;
    ///     fn call(args: f32) -> Self::Output {
    ///         args > 9000f32
    ///     }
    /// }
    /// struct P;
    ///
    /// let co1 = I32F32StrBool::inject(3);
    /// let folded = co1.fold(Poly(P));
    /// # }
    /// ```
    #[inline(always)]
    pub fn fold<Output, Folder>(self, folder: Folder) -> Output
    where
        Self: CoproductFoldable<Folder, Output>,
    {
        CoproductFoldable::fold(self, folder)
    }
}

/// Trait for folding a coproduct into a single value.
///
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::fold`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts or Folders of unknown type. If the type of everything is known,
/// then `co.fold(folder)` should "just work" even without the trait.
///
/// [`Coproduct::fold`]: enum.Coproduct.html#method.fold
pub trait CoproductFoldable<Folder, Output> {
    /// Use functions to fold a coproduct into a single value.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.fold
    fn fold(self, f: Folder) -> Output;
}

impl<P, R, CH, CTail: IndexedDrop> CoproductFoldable<Poly<P>, R>
    for Coproduct<UntaggedCoproduct<CH, CTail>>
where
    P: Func<CH, Output = R>,
    Coproduct<CTail>: CoproductFoldable<Poly<P>, R>,
{
    fn fold(self, f: Poly<P>) -> R {
        match self.take_head() {
            Ok(x) => P::call(x),
            Err(tail) => tail.fold(f),
        }
    }
}

impl<F, R, FTail, CH, CTail: IndexedDrop> CoproductFoldable<HCons<F, FTail>, R>
    for Coproduct<UntaggedCoproduct<CH, CTail>>
where
    F: FnOnce(CH) -> R,
    Coproduct<CTail>: CoproductFoldable<FTail, R>,
{
    fn fold(self, f: HCons<F, FTail>) -> R {
        let f_head = f.head;
        let f_tail = f.tail;
        match self.take_head() {
            Ok(r) => (f_head)(r),
            Err(rest) => rest.fold(f_tail),
        }
    }
}

/// This is literally impossible; CNil is not instantiable
impl<F, R> CoproductFoldable<F, R> for Coproduct<CNil> {
    fn fold(self, _: F) -> R {
        unreachable!()
    }
}

impl<'a, H: 'a, T: IndexedDrop, Out> ToRef<'a> for Coproduct<UntaggedCoproduct<H, T>>
where
    T: ToRef<'a>,
    UntaggedCoproduct<H, T>: ToRef<'a, Output = Out>,
    Out: IndexedDrop,
{
    type Output = Coproduct<Out>;
    fn to_ref(&'a self) -> Self::Output {
        Coproduct {
            tag: self.tag,
            untagged: self.untagged.to_ref(),
        }
    }
}

impl<'a, H: 'a, T> ToRef<'a> for UntaggedCoproduct<H, T>
where
    T: ToRef<'a>,
{
    type Output = UntaggedCoproduct<&'a H, <T as ToRef<'a>>::Output>;

    #[inline(always)]
    fn to_ref(&'a self) -> Self::Output {
        union Transmuter<'a, H, T>
        where
            T: ToRef<'a>,
        {
            before: &'a UntaggedCoproduct<H, T>,
            after: ManuallyDrop<UntaggedCoproduct<&'a H, T::Output>>,
        }
        ManuallyDrop::into_inner(unsafe { Transmuter { before: self }.after })
    }
}

impl<'a> ToRef<'a> for CNil {
    type Output = CNil;

    fn to_ref(&'a self) -> Self::Output {
        match *self {}
    }
}

impl<'a, H: 'a, T: IndexedDrop, Out> ToMut<'a> for Coproduct<UntaggedCoproduct<H, T>>
where
    T: ToMut<'a>,
    UntaggedCoproduct<H, T>: ToMut<'a, Output = Out>,
    Out: IndexedDrop,
{
    type Output = Coproduct<Out>;
    fn to_mut(&'a mut self) -> Self::Output {
        Coproduct {
            tag: self.tag,
            untagged: self.untagged.to_mut(),
        }
    }
}

impl<'a, H: 'a, T> ToMut<'a> for UntaggedCoproduct<H, T>
where
    T: ToMut<'a>,
{
    type Output = UntaggedCoproduct<&'a mut H, <T as ToMut<'a>>::Output>;

    #[inline(always)]
    fn to_mut(&'a mut self) -> Self::Output {
        union Transmuter<'a, H, T>
        where
            T: ToMut<'a>,
        {
            before: ManuallyDrop<&'a mut UntaggedCoproduct<H, T>>,
            after: ManuallyDrop<UntaggedCoproduct<&'a mut H, T::Output>>,
        }
        ManuallyDrop::into_inner(unsafe {
            Transmuter {
                before: ManuallyDrop::new(self),
            }
            .after
        })
    }
}

impl<'a> ToMut<'a> for CNil {
    type Output = CNil;

    fn to_mut(&'a mut self) -> Self::Output {
        match *self {}
    }
}

/// Trait for extracting a subset of the possible types in a coproduct.
///
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::subset`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. If you have a Coproduct of known type,
/// then `co.subset()` should "just work" even without the trait.
///
/// [`Coproduct::subset`]: enum.Coproduct.html#method.subset
pub trait CoproductSubsetter<Targets, Indices>: Sized {
    type Remainder;

    /// Extract a subset of the possible types in a coproduct (or get the remaining possibilities)
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.subset
    fn subset(self) -> Result<Targets, Self::Remainder>;
}

impl<H, T: IndexedDrop, THead, TTail: IndexedDrop, NHead: Counter, NTail, Rem>
    CoproductSubsetter<Coproduct<UntaggedCoproduct<THead, TTail>>, HCons<NHead, NTail>>
    for Coproduct<UntaggedCoproduct<H, T>>
where
    UntaggedCoproduct<H, T>: Taker<THead, NHead> + Drop1<NHead, Remainder = Rem>,
    Coproduct<Rem>: CoproductSubsetter<Coproduct<TTail>, NTail>,
    Rem: IndexedDrop,
{
    type Remainder = <Coproduct<Rem> as CoproductSubsetter<Coproduct<TTail>, NTail>>::Remainder;

    /// Attempt to extract a value from a subset of the types.
    fn subset(self) -> Result<Coproduct<UntaggedCoproduct<THead, TTail>>, Self::Remainder> {
        match self.uninject::<THead, NHead>() {
            Ok(good) => Ok(Coproduct::here(good)),
            Err(rest) => match rest.subset() {
                Ok(goods) => Ok(Coproduct::there(goods)),
                Err(bads) => Err(bads),
            },
        }
    }
}

impl<Choices> CoproductSubsetter<Coproduct<CNil>, HNil> for Choices {
    type Remainder = Self;

    #[inline(always)]
    fn subset(self) -> Result<Coproduct<CNil>, Self::Remainder> {
        Err(self)
    }
}

/// Trait for converting a coproduct into another that can hold its variants.
///
/// This trait is part of the implementation of the inherent method
/// [`Coproduct::embed`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// Coproducts of unknown type. If you have a Coproduct of known type,
/// then `co.embed()` should "just work" even without the trait.
///
/// [`Coproduct::embed`]: enum.Coproduct.html#method.embed
pub trait CoproductEmbedder<Out: IndexedDrop, Indices> {
    /// Convert a coproduct into another that can hold its variants.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: enum.Coproduct.html#method.embed
    fn embed(self) -> Coproduct<Out>;
}

impl CoproductEmbedder<CNil, HNil> for Coproduct<CNil> {
    fn embed(self) -> Coproduct<CNil> {
        match self.untagged {}
    }
}

impl<Head, Tail: IndexedDrop> CoproductEmbedder<UntaggedCoproduct<Head, Tail>, HNil>
    for Coproduct<CNil>
where
    Coproduct<CNil>: CoproductEmbedder<Tail, HNil>,
{
    fn embed(self) -> Coproduct<UntaggedCoproduct<Head, Tail>> {
        match self.untagged {}
    }
}

impl<Head, Tail, Out, NHead: Counter, NTail> CoproductEmbedder<Out, HCons<NHead, NTail>>
    for Coproduct<UntaggedCoproduct<Head, Tail>>
where
    Out: Injector<Head, NHead> + IndexedDrop,
    Tail: IndexedDrop,
    Coproduct<Tail>: CoproductEmbedder<Out, NTail>,
{
    fn embed(self) -> Coproduct<Out> {
        match self.take_head() {
            Ok(this) => Coproduct::inject(this),
            Err(those) => those.embed(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coproduct_inject() {
        type I32StrBool = Coprod!(i32, &'static str, bool);

        let co1 = I32StrBool::inject(3);
        assert_eq!(co1, Coproduct::here(3));
        let get_from_1a: Option<&i32> = co1.get();
        let get_from_1b: Option<&bool> = co1.get();
        assert_eq!(get_from_1a, Some(&3));
        assert_eq!(get_from_1b, None);

        let co2 = I32StrBool::inject(false);
        assert_eq!(
            co2,
            Coproduct::there(Coproduct::there(Coproduct::here(false)))
        );
        let get_from_2a: Option<&i32> = co2.get();
        let get_from_2b: Option<&bool> = co2.get();
        assert_eq!(get_from_2a, None);
        assert_eq!(get_from_2b, Some(&false));
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_coproduct_fold_consuming() {
        type I32F32StrBool = Coprod!(i32, f32, bool);

        let co1 = I32F32StrBool::inject(3);
        let folded = co1.fold(hlist![
            |i| format!("int {}", i),
            |f| format!("float {}", f),
            |b| (if b { "t" } else { "f" }).to_string(),
        ]);

        assert_eq!(folded, "int 3".to_string());
    }

    #[test]
    fn test_coproduct_poly_fold_consuming() {
        type I32F32StrBool = Coprod!(i32, f32, bool);

        impl Func<i32> for P {
            type Output = bool;
            fn call(args: i32) -> Self::Output {
                args > 100
            }
        }
        impl Func<bool> for P {
            type Output = bool;
            fn call(args: bool) -> Self::Output {
                args
            }
        }
        impl Func<f32> for P {
            type Output = bool;
            fn call(args: f32) -> Self::Output {
                args > 9000f32
            }
        }
        struct P;

        let co1 = I32F32StrBool::inject(3);
        let folded = co1.fold(Poly(P));

        assert_eq!(folded, false);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_coproduct_fold_non_consuming() {
        type I32F32Bool = Coprod!(i32, f32, bool);

        let co1 = I32F32Bool::inject(3);
        let co2 = I32F32Bool::inject(true);
        let co3 = I32F32Bool::inject(42f32);

        assert_eq!(
            co1.to_ref().fold(hlist![
                |&i| format!("int {}", i),
                |&f| format!("float {}", f),
                |&b| (if b { "t" } else { "f" }).to_string(),
            ]),
            "int 3".to_string()
        );
        assert_eq!(
            co2.to_ref().fold(hlist![
                |&i| format!("int {}", i),
                |&f| format!("float {}", f),
                |&b| (if b { "t" } else { "f" }).to_string(),
            ]),
            "t".to_string()
        );
        assert_eq!(
            co3.to_ref().fold(hlist![
                |&i| format!("int {}", i),
                |&f| format!("float {}", f),
                |&b| (if b { "t" } else { "f" }).to_string(),
            ]),
            "float 42".to_string()
        );
    }

    #[test]
    fn test_coproduct_uninject() {
        type I32StrBool = Coprod!(i32, &'static str, bool);

        let co1 = I32StrBool::inject(3);
        let co2 = I32StrBool::inject("hello");
        let co3 = I32StrBool::inject(false);

        let uninject_i32_co1: Result<i32, _> = co1.uninject();
        let uninject_str_co1: Result<&'static str, _> = co1.uninject();
        let uninject_bool_co1: Result<bool, _> = co1.uninject();
        assert_eq!(uninject_i32_co1, Ok(3));
        assert!(uninject_str_co1.is_err());
        assert!(uninject_bool_co1.is_err());

        let uninject_i32_co2: Result<i32, _> = co2.uninject();
        let uninject_str_co2: Result<&'static str, _> = co2.uninject();
        let uninject_bool_co2: Result<bool, _> = co2.uninject();
        assert!(uninject_i32_co2.is_err());
        assert_eq!(uninject_str_co2, Ok("hello"));
        assert!(uninject_bool_co2.is_err());

        let uninject_i32_co3: Result<i32, _> = co3.uninject();
        let uninject_str_co3: Result<&'static str, _> = co3.uninject();
        let uninject_bool_co3: Result<bool, _> = co3.uninject();
        assert!(uninject_i32_co3.is_err());
        assert!(uninject_str_co3.is_err());
        assert_eq!(uninject_bool_co3, Ok(false));
    }

    #[test]
    fn test_coproduct_subset() {
        type I32StrBool = Coprod!(i32, &'static str, bool);

        // CNil can be extracted from anything.
        let res: Result<Coproduct<CNil>, _> = I32StrBool::inject(3).subset();
        assert!(res.is_err());

        if false {
            #[allow(unreachable_code)]
            {
                // ...including CNil.
                #[allow(unused)]
                let cnil: Coproduct<CNil> = panic!();
                let _res: Result<Coproduct<CNil>, _> = cnil.subset();
                let _ = res;
            }
        }

        {
            // Order does not matter.
            let co = I32StrBool::inject(3);
            let res: Result<Coprod!(bool, i32), _> = co.subset();
            assert_eq!(res, Ok(Coproduct::there(Coproduct::here(3))));

            let co = I32StrBool::inject("4");
            let res: Result<Coprod!(bool, i32), _> = co.subset();
            assert_eq!(res, Err(Coproduct::here("4")));
        }
    }

    #[test]
    fn test_coproduct_embed() {
        // CNil can be embedded into any coproduct.
        if false {
            #[allow(unreachable_code)]
            {
                #[allow(unused)]
                let cnil: Coproduct<CNil> = panic!();
                let _: Coproduct<CNil> = cnil.embed();

                #[allow(unused)]
                let cnil: Coproduct<CNil> = panic!();
                let _: Coprod!(i32, bool) = cnil.embed();
            }
        }

        #[derive(Debug, PartialEq)]
        struct A;
        #[derive(Debug, PartialEq)]
        struct B;
        #[derive(Debug, PartialEq)]
        struct C;

        {
            // Order does not matter.
            let co_a = <Coprod!(C, A, B)>::inject(A);
            let co_b = <Coprod!(C, A, B)>::inject(B);
            let co_c = <Coprod!(C, A, B)>::inject(C);
            let out_a: Coprod!(A, B, C) = co_a.embed();
            let out_b: Coprod!(A, B, C) = co_b.embed();
            let out_c: Coprod!(A, B, C) = co_c.embed();
            assert_eq!(out_a, Coproduct::here(A));
            assert_eq!(out_b, Coproduct::there(Coproduct::here(B)));
            assert_eq!(
                out_c,
                Coproduct::there(Coproduct::there(Coproduct::here(C)))
            );
        }

        {
            // Multiple variants can resolve to the same output w/o type annotations
            type ABC = Coprod!(A, B, C);
            type BBB = Coprod!(B, B, B);

            let b1 = BBB::inject::<_, Here>(B);
            let b2 = BBB::inject::<_, There<Here>>(B);
            let out1: ABC = b1.embed();
            let out2: ABC = b2.embed();
            assert_eq!(out1, Coproduct::there(Coproduct::here(B)));
            assert_eq!(out2, Coproduct::there(Coproduct::here(B)));
        }
    }
}
