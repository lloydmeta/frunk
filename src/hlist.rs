use std::ops::Add;

pub trait HList: Sized {
    fn length(&self) -> u32;

    fn push<H>(self, h: H) -> HCons<H, Self> {
        let l = self.length() + 1;
        HCons {
            head: h,
            tail: self,
            length: l,
        }
    }
}

/// Represents the right-most end of a heterogeneous list
///
/// Used to begin one:
///
/// ```
/// # use frust::hlist::*;
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

#[derive(PartialEq, Eq, Debug)]
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
    pub length: u32,
}

impl<H, T> HList for HCons<H, T> {
    fn length(&self) -> u32 {
        self.length
    }
}

impl<H, T> HCons<H, T> {
    /// Returns the head of the list and the tail of the list as a tuple2.
    /// The original list is consumed
    ///
    /// ```
    /// # #[macro_use] extern crate frust; use frust::hlist::*; fn main() {
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
/// # use frust::hlist::*;
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
/// # #[macro_use] extern crate frust; use frust::hlist::*; fn main() {
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

    // Just a single item
    ($single: expr) => {
        HNil.push($single)
    };

    ($last: expr, $( $repeated: expr ), +) => {
// Invoke recursive reversal of list that ends in the macro expansion implementation
// of the reversed list
        hlist!([($last),] => $( $repeated, )+);
    };

// ([accumulatedList], listToReverse); recursively calls hlist until the list to reverse
// + is empty (see next pattern)
    ([$(($acc: expr),)*] =>$next: expr, $( $repeated:expr, )*) => {
        hlist!([($next), $( ($acc) ,)*] => $( $repeated, ) *);
    };

// Finally expand into our implementation
    ([($h:expr), $( ($repeated: expr), )*] => ) => {
        HNil.push($h)
         $(.push($repeated))*
    }
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
        let length = self.length() + rhs.length();
        HCons {
            head: self.head,
            tail: self.tail + rhs,
            length: length,
        }
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
