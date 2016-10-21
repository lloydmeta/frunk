use std::ops::Add;

pub trait HList: Sized {
    fn length(&self) -> u32;

    fn push<H>(self, h: H) -> HCons<H, Self> {
        HCons {
            head: h,
            tail: self,
        }
    }
}

/// Represents the right-most end of a heterogeneous list
///
/// Used to begin one:
///
/// ```
/// # use frunk::hlist::*;
///
/// let hlist1 = h_cons(1, HNil);
/// let (h, _) = hlist1.pop();
/// assert_eq!(h, 1);
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct HNil;

impl HList for HNil {
    fn length(&self) -> u32 {
        0
    }
}

/// An HList is a heterogeneous list, one that is statically typed
/// at compile time.
///
/// In simple terms, it is just a really deeply nested Tuple2.
#[derive(PartialEq, Eq, Debug)]
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}

impl<H, T: HList> HList for HCons<H, T> {
    fn length(&self) -> u32 {
        1 + self.tail.length()
    }
}

impl<H, T> HCons<H, T> {
    /// Returns the head of the list and the tail of the list as a tuple2.
    /// The original list is consumed
    ///
    /// ```
    /// # #[macro_use] extern crate frunk; use frunk::hlist::*; fn main() {
    ///
    /// let hlist1 = hlist!("hi");
    /// let (h, _) = hlist1.pop();
    /// assert_eq!(h, "hi");
    /// # }
    /// ```
    pub fn pop(self) -> (H, T) {
        (self.head, self.tail)
    }
}


/// Takes an element and an Hlist and returns another one with
/// the element prepended to the original list. The original list
/// is consumed
///
/// ```
/// # use frunk::hlist::*;
///
/// let h_list = h_cons("what", h_cons(1.23f32, HNil));
/// let (h1, tail) = h_list.pop();
/// let (h2, _) = tail.pop();
/// assert_eq!(h1, "what");
/// assert_eq!(h2, 1.23f32);
/// ```
pub fn h_cons<H, T: HList>(h: H, tail: T) -> HCons<H, T> {
    tail.push(h)
}

/// Create an HList
///
/// ```
/// # #[macro_use] extern crate frunk; use frunk::hlist::*; fn main() {
///
/// let h = hlist![13.5f32, "hello", Some(41)];
/// let (h1, tail1) = h.pop();
/// let (h2, tail2) = tail1.pop();
/// let (h3, tail3) = tail2.pop();
/// assert_eq!(h1, 13.5f32);
/// assert_eq!(h2, "hello");
/// assert_eq!(h3, Some(41));
/// assert_eq!(tail3, HNil);
/// # }
/// ```
#[macro_export]
macro_rules! hlist {

    // Nothing
    () => { HNil };

    // Just a single item
    ($single: expr) => {
        HCons { head: $single, tail: HNil }
    };

    ($first: expr, $( $repeated: expr ), +) => {
// Invoke recursive reversal of list that ends in the macro expansion implementation
// of the reversed list
        HCons { head: $first, tail: hlist!($($repeated), *)}
    };

}

impl<RHS> Add<RHS> for HNil
    where RHS: HList
{
    type Output = RHS;

    fn add(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, T, RHS> Add<RHS> for HCons<H, T>
    where T: Add<RHS>,
          RHS: HList
{
    type Output = HCons<H, <T as Add<RHS>>::Output>;

    fn add(self, rhs: RHS) -> Self::Output {
        HCons {
            head: self.head,
            tail: self.tail + rhs,
        }
    }
}

pub trait IntoTuple2 {
    type HeadType;
    type TailOutput;

    /// Turns an HList into nested Tuple2s, which are less troublesome to pattern match
    /// and have a nicer type signature.
    ///
    /// ```
    /// # #[macro_use] extern crate frunk; use frunk::hlist::*; fn main() {
    /// let h = hlist![1, "hello", true, 42f32];
    ///
    /// // We now have a much nicer pattern matching experience
    /// let (first,(second,(third, fourth))) = h.into_tuple2();
    ///
    /// assert_eq!(first ,       1);
    /// assert_eq!(second, "hello");
    /// assert_eq!(third ,     true);
    /// assert_eq!(fourth,    42f32);
    /// # }
    /// ```
    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput);
}

impl<T1, T2> IntoTuple2 for HCons<T1, HCons<T2, HNil>> {
    type HeadType = T1;
    type TailOutput = T2;

    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.head)
    }
}

impl<T, Tail> IntoTuple2 for HCons<T, Tail>
    where Tail: IntoTuple2
{
    type HeadType = T;
    type TailOutput = (<Tail as IntoTuple2>::HeadType, <Tail as IntoTuple2>::TailOutput);

    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.into_tuple2())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hcons() {
        let hlist1 = h_cons(1, HNil);
        let (h, _) = hlist1.pop();
        assert_eq!(h, 1);

        let hlist2 = h_cons("hello", h_cons(1, HNil));
        let (h2, tail2) = hlist2.pop();
        let (h1, _) = tail2.pop();
        assert_eq!(h2, "hello");
        assert_eq!(h1, 1);
    }

    struct HasHList<T: HList>(T);

    #[test]
    fn test_contained_list() {
        let c = HasHList(h_cons(1, HNil));
        let retrieved = c.0;
        assert_eq!(retrieved.length(), 1);
        let new_list = h_cons(2, retrieved);
        assert_eq!(new_list.length(), 2);
    }

    #[test]
    fn test_macro() {
        assert_eq!(hlist![], HNil);
        let h = hlist![1, "2", 3];
        let (h1, tail1) = h.pop();
        assert_eq!(h1, 1);
        assert_eq!(tail1, hlist!["2", 3]);
        let (h2, tail2) = tail1.pop();
        assert_eq!(h2, "2");
        assert_eq!(tail2, hlist![3]);
        let (h3, tail3) = tail2.pop();
        assert_eq!(h3, 3);
        assert_eq!(tail3, HNil);
    }

    #[test]
    fn test_add() {
        let h1 = hlist![true, "hi"];
        let h2 = hlist![1, 32f32];
        let combined = h1 + h2;
        assert_eq!(combined, hlist![true, "hi", 1, 32f32])
    }

}
