pub trait HList {
    fn length(&self) -> u32;
}

pub trait HListPush: HList {
    fn push<H>(self, h: H) -> HCons<H, Self> where Self: Sized;
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
pub struct HNil;

impl HList for HNil {
    fn length(&self) -> u32 { 0 }
}

/// Represents a heterogeneous list.
///
/// To construct one, use the h_cons method
///
/// ```
/// # use frust::hlist::*;
///
/// let h_list = h_cons("hello", h_cons(1, HNil));
/// let (h1, tail) = h_list.pop();
/// let (h2, _) = tail.pop();
/// assert_eq!(h1, "hello");
/// assert_eq!(h2, 1);
/// ```
pub struct HCons<H, T: HListPush> {
    head: H,
    tail: T,
    length: u32
}

impl<H, T: HListPush> HList for HCons<H, T> {
    fn length(&self) -> u32 { self.length }
}

impl<H, T: HListPush> HCons<H, T> {

    /// Returns the head of the list and the tail of the list as a tuple2.
    /// The original list is consumed
    ///
    /// ```
    /// # use frust::hlist::*;
    ///
    /// let hlist1 = h_cons("hi", HNil);
    /// let (h, _) = hlist1.pop();
    /// assert_eq!(h, "hi");
    /// ```
    pub fn pop(self) -> (H, T) {
        (self.head, self.tail)
    }
}

impl HListPush for HNil {
    fn push<NewH>(self, h: NewH) -> HCons<NewH, HNil>
        where Self: Sized
    {
        HCons {
            head: h,
            tail: self,
            length: 1
        }
    }
}

impl<H, T: HListPush> HListPush for HCons<H, T> {
    fn push<NewH>(self, h: NewH) -> HCons<NewH, Self>
        where Self: Sized
    {
        let l =  self.length() + 1;
        HCons {
            head: h,
            tail: self,
            length: l
        }
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
pub fn h_cons<H, T: HListPush>(h: H, tail: T) -> HCons<H, T> {
    tail.push(h)
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

}
