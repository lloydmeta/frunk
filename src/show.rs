use std::cell::*;
use std::hash::Hash;
use std::ops::Deref;
use std::collections::{HashSet, HashMap};

/// Typeclass for showing a given type
pub trait Show {
    fn show(&self) -> String;
}

macro_rules! simple_show_impls {
  ($($tr:ty),*) => {
    $(
      impl Show for $tr {
        fn show(&self) -> String { self.to_string() }
      }
    )*
  }
}

simple_show_impls!(i8,
                   i16,
                   i32,
                   i64,
                   u8,
                   u16,
                   u32,
                   u64,
                   isize,
                   usize,
                   f32,
                   f64,
                   bool);

impl Show for &'static str {
    fn show(&self) -> String {
        (*self).show()
    }
}

impl Show for str {
    fn show(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl Show for char {
    fn show(&self) -> String {
        format!("'{}'", self)
    }
}

impl<T: Show> Show for [T] {
    fn show(&self) -> String {
        let shown = self.iter()
                        .map(|i| i.show())
                        .collect::<Vec<String>>();
        format!("[{}]", shown.join(", "))
    }
}

impl Show for () {
    fn show(&self) -> String {
        String::from("()")
    }
}

impl<T: Show + Copy> Show for Cell<T> {
    fn show(&self) -> String {
        self.get().show()
    }
}

impl<'b, T: Show> Show for Ref<'b, T> {
    fn show(&self) -> String {
        self.deref().show()
    }
}

impl<'b, T: Show> Show for RefMut<'b, T> {
    fn show(&self) -> String {
        self.deref().show()
    }
}

impl<T: Show> Show for RefCell<T> {
    fn show(&self) -> String {
        format!("RefCell(value={})", self.borrow().show())
    }
}

impl<T: Show> Show for Option<T> {
    fn show(&self) -> String {
        match *self {
            Some(ref v) => format!("Some({})", v.show()),
            Option::None => String::from("None"),
        }
    }
}

impl<T> Show for HashSet<T>
    where T: Show + Eq + Hash
{
    fn show(&self) -> String {
        let shown = self.into_iter()
                        .map(|i| i.show())
                        .collect::<Vec<String>>();
        format!("HashSet({})", shown.join(", "))
    }
}

impl<K, V> Show for HashMap<K, V>
    where K: Show + Eq + Hash,
          V: Show
{
    fn show(&self) -> String {
        let shown = self.into_iter()
                        .map(|(k, v)| format!("{} -> {}", k.show(), v.show()))
                        .collect::<Vec<String>>();
        format!("HashMap({})", shown.join(", "))
    }
}

impl<T: Show> Show for Box<T> {
    fn show(&self) -> String {
        format!("Box({})", self.deref().show())
    }
}

macro_rules! tuple_impls {
    () => {}; // no more

    (($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident), )*) => {
// Invoke recursive reversal of list that ends in the macro expansion implementation
// of the reversed list
//
        tuple_impls!([($idx, $typ);] $( ($nidx => $ntyp), )*);
        tuple_impls!($( ($nidx => $ntyp), )*); // invoke macro on tail
    };

// ([accumulatedList], listToReverse); recursively calls tuple_impls until the list to reverse
// + is empty (see next pattern)
//
    ([$(($accIdx: tt, $accTyp: ident);)+]  ($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident), )*) => {
      tuple_impls!([($idx, $typ); $(($accIdx, $accTyp); )*] $( ($nidx => $ntyp), ) *);
    };

// Finally expand into our implementation
    ([($idx:tt, $typ:ident); $( ($nidx:tt, $ntyp:ident); )*]) => {
        impl<$typ: Show, $( $ntyp: Show),*> Show for ($typ, $( $ntyp ),*) {
            fn show(&self) -> String {
                let arr = vec![self.$idx.show(), $(self.$nidx.show(), ) *];
                format!("({})", arr.join(", "))
            }
        }
    }
}

tuple_impls! {
    (20 => U),
    (19 => T),
    (18 => S),
    (17 => R),
    (16 => Q),
    (15 => P),
    (14 => O),
    (13 => N),
    (12 => M),
    (11 => L),
    (10 => K),
    (9 => J),
    (8 => I),
    (7 => H),
    (6 => G),
    (5 => F),
    (4 => E),
    (3 => D),
    (2 => C),
    (1 => B),
    (0 => A),
}

#[cfg(test)]
mod tests {
    use std::cell::*;
    use super::Show;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::collections::{HashSet, HashMap};

    macro_rules! simple_show_tests {
      ($($x: expr; $tr:ty; $name:ident),+) => {
        $(
          #[test]
          fn $name() {
            let i: $tr = $x;
            assert_eq!(i.show(), $x.to_string())
          }
        )*
      }
    }

    simple_show_tests!(
        1; i8; test_i8,
        1; i16; test_i16,
        1; i32; test_i32,
        1; i64; test_i64,
        1; u8; test_u8,
        1; u16; test_u16,
        1; u32; test_u32,
        1; u64; test_u64,
        1; usize; test_usize,
        1; isize; test_isize,
        1f32; f32; test_f32,
        1f64; f64; test_f64,
        true; bool; test_bool
    );

    #[test]
    fn test_str() {
        let i = "hello";
        assert_eq!(i.show(), "\"hello\"")
    }

    #[test]
    fn test_char() {
        let i: char = 't';
        assert_eq!(i.show(), "'t'")
    }

    #[test]
    fn test_string() {
        let mut i = String::new();
        i.push_str("hello");
        assert_eq!(i.show(), "\"hello\"")
    }

    #[test]
    fn test_tuple() {
        let i = (1, 'a', 1.3f32, "hello");
        assert_eq!(i.show(), "(1, 'a', 1.3, \"hello\")")
    }

    #[test]
    fn test_slice() {
        let i = ["hello", "world"];
        assert_eq!(i.show(), "[\"hello\", \"world\"]")
    }

    #[test]
    fn test_vec() {
        let i = vec!["hello", "world"];
        assert_eq!(i.show(), "[\"hello\", \"world\"]")
    }

    #[test]
    fn test_unit() {
        let i = ();
        assert_eq!(i.show(), "()")
    }

    #[test]
    fn test_some() {
        let i = Some("one");
        assert_eq!(i.show(), "Some(\"one\")")
    }

    #[test]
    fn test_none() {
        let i: Option<i8> = None;
        assert_eq!(i.show(), "None")
    }

    #[test]
    fn test_box() {
        let i = Box::new(1);
        assert_eq!(i.show(), "Box(1)");
        assert_eq!(Some(Box::new(1)).show(), "Some(Box(1))");
    }

    #[test]
    fn test_cell() {
        let i = Cell::new(1);
        assert_eq!(i.show(), "1")
    }

    #[test]
    fn test_refcell() {
        let i = RefCell::new(1);
        assert_eq!(i.show(), "RefCell(value=1)")
    }

    #[test]
    fn test_rc() {
        let i = Rc::new(1);
        assert_eq!(i.show(), "1")
    }

    #[test]
    fn test_arc() {
        let i = Arc::new(1);
        assert_eq!(i.show(), "1")
    }

    #[test]
    fn test_hashset() {
        let mut i = HashSet::new();
        i.insert("hello");
        i.insert("world");
        let shown = i.show();
        assert!(shown.contains("HashSet("));
        assert!(shown.contains("\"hello\""));
        assert!(shown.contains("\"world\""));
    }

    #[test]
    fn test_hashmap() {
        let mut i: HashMap<i8, bool> = HashMap::new();
        i.insert(1, true);
        i.insert(2, false);
        let shown = i.show();
        assert!(shown.contains("HashMap("));
        assert!(shown.contains("1 -> true"));
        assert!(shown.contains("2 -> false"));
    }
}
