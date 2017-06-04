use frunk_core::hlist::*;
use std::ops::Deref;

/// Output is a parameter because we want to allow Semi to not
/// necessarily return Self (e.g. in the case of Self being a pointer)
///
/// RHS is also a parameter in case we need to chain more combinations together..
///
/// Also, using a type parameter instead of an associated type because
/// there was a weird diverging trait search going with an associated type.
///
/// This means that yes, we need to enforce things with Laws.
pub trait Semi<RHS = Self> {
    type Output;
    fn combine(self, other: RHS) -> Self::Output;
}

/// Allow the combination of any two HLists having the same structure
/// if all of the sub-element types are also Semiups
impl<H, T> Semi for HCons<H, T>
    where H: Semi,
          T: Semi
{
    type Output = HCons<<H as Semi>::Output, <T as Semi>::Output>;
    fn combine(self, other: Self) -> Self::Output {
        let tail_comb = self.tail.combine(other.tail);
        let h_comb = self.head.combine(other.head);
        HCons {
            head: h_comb,
            tail: tail_comb,
        }
    }
}

/// Since () + () = (), the same is true for HNil
impl Semi for HNil {
    type Output = HNil;
    fn combine(self, _: Self) -> Self::Output {
        self
    }
}

/// Allow the combination of any two HLists having the same structure
/// if all of the sub-element types are also Semiups
impl<'a, H, T> Semi for &'a HCons<H, T>
    where &'a H: Semi,
          &'a T: Semi
{
    type Output = HCons<<&'a H as Semi>::Output, <&'a T as Semi>::Output>;
    fn combine(self, other: Self) -> Self::Output {
        let tail_comb = self.tail.combine(&other.tail);
        let h_comb = self.head.combine(&other.head);
        HCons {
            head: h_comb,
            tail: tail_comb,
        }
    }
}

/// Since () + () = (), the same is true for HNil
impl<'a> Semi for &'a HNil {
    type Output = HNil;
    fn combine(self, _: Self) -> Self::Output {
        HNil
    }
}

impl<T> Semi<> for Option<T>
    where T: Semi,
          <T as Semi>::Output: From<T>
{
    type Output = Option<<T as Semi>::Output>;
    fn combine(self, other: Self) -> Self::Output {
        if let Some(s) = self {
            if let Some(o) = other {
                Some(s.combine(o))
            } else {
                Some(<T as Semi>::Output::from(s))
            }
        } else {
            other.map(<T as Semi>::Output::from)
        }
    }
}

impl<'a, T> Semi for &'a Option<T>
    where &'a T: Semi,
          T: Clone + Sized,
          <&'a T as Semi>::Output: From<T>
{
    type Output = Option<<&'a T as Semi>::Output>;
    fn combine(self, other: Self) -> Self::Output {
        if let &Some(ref s) = self {
            if let &Some(ref o) = other {
                Some(s.combine(o))
            } else {
                (*self).clone().map(<&'a T as Semi>::Output::from)
            }
        } else {
            (*other).clone().map(<&'a T as Semi>::Output::from)
        }
    }
}

macro_rules! numeric_semi_imps {
  ($($tr:ty),*) => {
    $(
      impl Semi for $tr {
        type Output = $tr;
        fn combine(self, other: Self) -> Self::Output { self + other }
      }
      impl <'a> Semi<&'a $tr> for $tr {
          type Output = $tr;
        fn combine(self, other: &'a $tr) -> Self::Output { self + other }
      }
      impl <'a> Semi for &'a $tr {
          type Output = $tr;
        fn combine(self, other: Self) -> Self::Output { self + other }
      }
      impl <'a> Semi<$tr> for &'a $tr {
          type Output = $tr;
        fn combine(self, other: $tr) -> Self::Output { self + other }
      }
    )*
  }
}

numeric_semi_imps!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);

impl Semi for String {
    type Output = String;
    fn combine(self, other: Self) -> Self::Output {
        let mut s = self;
        s.push_str(&*other);
        s
    }
}

impl <'a> Semi<&'a str> for String {
    type Output = String;
    fn combine(self, other: &'a str) -> Self::Output {
        let mut s = self;
        s.push_str(other);
        s
    }
}

impl <'a> Semi for &'a str {
    type Output = String;
    fn combine(self, other: Self) -> Self::Output {
        let mut s = self.to_string();
        s.push_str(&*other);
        s
    }
}

impl <'a> Semi<String> for &'a str {
    type Output = String;
    fn combine(self, other: String) -> Self::Output {
        let mut s = self.to_string();
        s.push_str(&other[..]);
        s
    }
}

impl<T> Semi for Box<T> where T: Semi {
    type Output = Box<<T as Semi>::Output>;
    fn combine(self, other: Self) -> Self::Output {
        let s = *self;
        let o = *other;
        Box::new(s.combine(o))
    }
}

impl<'a, T> Semi for &'a Box<T> where &'a T: Semi {
    type Output = Box<<&'a T as Semi>::Output>;
    fn combine(self, other: Self) -> Self::Output {
        let s = self.deref();
        let o = other.deref();
        Box::new(s.combine(o))
    }
}

impl<T> Semi<Vec<T>> for Vec<T> {
    type Output = Self;
    fn combine(self, other: Self) -> Self::Output {
        let mut v = self;
        let mut o = other;
        v.append(&mut o);
        v
    }
}

impl<'a, T> Semi for &'a Vec<T> where T: Clone{
    type Output = Vec<T>;
    fn combine(self, other: Self) -> Self::Output {
        let mut v = self.clone();
        v.extend_from_slice(other);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! semi_tests {
      ($($name:ident, $comb: expr => $expected: expr, $tr:ty)+) => {
        $(
          #[test]
          fn $name() {
            let r: $tr = $comb;
            assert_eq!(r, $expected)
          }
        )*
      }
    }

    macro_rules! semi_tests_2 {
      ($($name:ident, $lhs: expr => $rhs: expr => $expected: expr, $tp: ty)+) => {
        $(
          #[test]
          fn $name() {
            let lhs: $tp = $lhs;
            let rhs: $tp = $rhs;
            let r = lhs.combine(rhs);
            assert_eq!(r, $expected)
          }
        )*
      }
    }

    semi_tests_2! {
        test_i8, 1 => 2 => 3, i8
        test_i8_2, &1 => &2 => 3, &i8
        test_i16, 1 => 2 => 3, i16
        test_i32, 1 => 2 => 3, i32
        test_u8, 1 => 2 => 3, u8
        test_u16, 1 => 2 => 3, u16
        test_u32, 1 => 2 => 3, u32
        test_usize, 1 => 2 => 3, usize
        test_isize, 1 => 2 => 3, isize
        test_f32, 1f32 => 2f32 => 3f32, f32
        test_f64, 1f64 => 2f64 => 3f64, f64
        test_option_i16, Some(1) => Some(2) => Some(3), Option<i16>
        test_option_i16_none1, None => Some(2) => Some(2), Option<i16>
        test_option_i16_none2, Some(2) => None => Some(2), Option<i16>
        // test_option_i16_ref, &Some(1) => &Some(2) => Some(3), &Option<i16> // Uncommenting this causes a diverging impl search
        // test_option_i16_none1_ref, &None => &Some(2) => Some(2), &Option<i16>
        // test_option_i16_none2_ref, &Some(2) => &None => Some(2), &Option<i16>
    }

    #[test]
    fn test_i32_3() {
        assert_eq!(3, 1.combine(2));
    }

    #[test]
    fn test_combine_hlist() {
        let h1 = hlist![Some(1), 3.3, 53i64, "hello".to_owned()];
        let h2 = hlist![Some(2), 1.2, 1i64, " world".to_owned()];
        let h3 = hlist![Some(3), 4.5, 54, "hello world".to_owned()];
        assert_eq!(h1.combine(h2), h3)
    }

    #[test]
    fn test_combine_hlist_2() {
        let h1 = hlist![Some(1), 3.3, 53i64, "hello"];
        let h2 = hlist![Some(2), 1.2, 1i64, " world"];
        let h3 = hlist![Some(3), 4.5, 54, "hello world".to_owned()]; // sadly types don't line up otherwise
        assert_eq!(h1.combine(h2), h3)
    }

    #[test]
    fn test_combine_str() {
        assert_eq!("hello".combine(" world"), "hello world")
    }

    #[test]
    fn test_combine_str_chained() {
        assert_eq!("hello".combine(" world").combine(" yay"), "hello world yay")
    }

    #[test]
    fn test_combine_i32_chained() {
        let i1: i32 = 50;
        let i2: i32 = 51;
        let i3: i32 = 52;
        let c: i32 = (&i1).combine(&i2).combine(&i3);
        assert_eq!(c, 153)
    }

    #[test]
    fn test_combine_option_str() {
        assert_eq!(Some("hello").combine(Some(" world")), Some("hello world".to_string()))
    }

}
