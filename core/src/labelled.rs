//! This module holds the machinery behind LabelledGeneric.
//!
//! A LabelledGeneric instance is pretty much exactly the same as a Generic instance, except
//! that the generic representation should contain information about field names.
//!
//! Having a separate trait for LabelledGenerics gives us the freedom to derive both
//! lablled and non-labelled generic type class instances for our types.
//!
//! Asides from the main LabelledGeneric trait, this module holds helper methods that allow
//! users to use LabelledGeneric without using universal function call syntax.
//!
//! In addition, this module holds macro-generated enums that map to letters in field names (identifiers).

use std::marker::PhantomData;
use hlist::*;
use std::fmt;
use self::internal::*;

/// A trait that converts from a type to a labelled generic representation
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
/// #[derive(LabelledGeneric)]
/// struct NewUser<'a> {
///     first_name: &'a str,
///     last_name: &'a str,
///     age: usize,
/// }
///
/// #[derive(LabelledGeneric)]
/// struct SavedUser<'a> {
///     first_name: &'a str,
///     last_name: &'a str,
///     age: usize,
/// }
///
/// let n_user = NewUser {
///     first_name: "Joe",
///     last_name: "Blow",
///     age: 30,
/// };
///
/// let s_user = <SavedUser as LabelledGeneric>::convert_from(n_user); // done
/// ```
pub trait LabelledGeneric {
    /// The labelled generic representation type
    type Repr;

    /// Go from something to Repr
    fn into(self) -> Self::Repr;

    /// Go from labelled Repr to something
    fn from(r: Self::Repr) -> Self;

    /// From one type to another using a type with a compatible labelled generic representation
    fn convert_from<A>(a: A) -> Self
        where A: LabelledGeneric<Repr = Self::Repr>,
              Self: Sized
    {
        let repr = <A as LabelledGeneric>::into(a);
        <Self as LabelledGeneric>::from(repr)
    }

    /// Converts from another type A into Self assuming that A and Self have labelled generic representations
    /// that can be sculpted into each other.
    ///
    /// Note that this method tosses away the "remainder" of the sculpted representation. In other
    /// words, anything that is not needed from A gets tossed out.
    fn sculpted_convert_from<A, Indices>(a: A) -> Self
        where A: LabelledGeneric,
              Self: Sized,
    // The labelled representation of A must be sculpt-able into the labelled representation of Self
              <A as LabelledGeneric>::Repr: Sculptor<<Self as LabelledGeneric>::Repr, Indices> {
        let a_gen = <A as LabelledGeneric>::into(a);
        // We toss away the remainder.
        let (self_gen, _): (<Self as LabelledGeneric>::Repr, _) = a_gen.sculpt();
        <Self as LabelledGeneric>::from(self_gen)
    }
}

/// Given a labelled generic Representation of an A, returns A
pub fn from_labelled_generic<A, Repr>(gen: Repr) -> A
    where A: LabelledGeneric<Repr = Repr>
{
    <A as LabelledGeneric>::from(gen)
}

/// Given an A, returns its labelled generic Representation
pub fn into_labelled_generic<A, Repr>(a: A) -> Repr
    where A: LabelledGeneric<Repr = Repr>
{
    <A as LabelledGeneric>::into(a)
}

/// Converts one type into another assuming they have the same labelled generic Representation
pub fn labelled_convert_from<A, B, Repr>(a: A) -> B
    where A: LabelledGeneric<Repr = Repr>,
          B: LabelledGeneric<Repr = Repr>
{
    <B as LabelledGeneric>::convert_from(a)
}

/// Converts from one type into another assuming that their labelled generic representations
/// can be sculpted into each other.
///
/// The "Indices" type parameter allows the compiler to figure out that the two representations
/// can indeed be morphed into each other.
pub fn sculpted_convert_from<A, B, Indices>(a: A) -> B
    where A: LabelledGeneric,
          B: LabelledGeneric,
// The labelled representation of A must be sculpt-able into the labelled representation of B
          <A as LabelledGeneric>::Repr: Sculptor<<B as LabelledGeneric>::Repr, Indices> {
    <B as LabelledGeneric>::sculpted_convert_from(a)
}

/// Trait for getting the static string representation of a type
///
/// Used mostly for building the runtime representation of the name of a labelled type, one
/// char at a time.
pub trait AsStaticStr {
    fn get_char() -> &'static str;
}

// Create a bunch of enums that can be used to represent characters on the type level
macro_rules! create_enums_for {
    ($($i: ident)*) => {
        $(
            #[allow(non_snake_case, non_camel_case_types)]
            #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
            pub enum $i {}

            impl AsStaticStr for $i {
                fn get_char() -> &'static str { stringify!($i) }
            }
        )*
    }
}

// Same as above, but for identifiers that need to be prefixed with an underscore
macro_rules! create_enums_for_underlined {
    ($($i: ident)*) => {
        $(
            #[allow(non_snake_case, non_camel_case_types)]
            #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
            pub enum $i {}

            impl AsStaticStr for $i {
                fn get_char() -> &'static str { &stringify!($i)[1..2] }
            }
        )*
    }
}

// Add more as needed.
create_enums_for! { a b c d e f g h i j k l m n o p q r s t u v w x y z A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }
create_enums_for_underlined! { __ _1 _2 _3 _4 _5 _6 _7 _8 _9 _0  }

// Define these escape ones manually
#[allow(non_snake_case, non_camel_case_types)]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum _uc {}

/// Implementation for bookending unicode characters
impl AsStaticStr for _uc {

    /// The escape character for the beginning of a unicode sequence
    ///
    /// If this is changed, make sure to update the characters used in
    /// decoding (see self::internal)
    fn get_char() -> &'static str { "{" }
}
// Define these escape ones manually
#[allow(non_snake_case, non_camel_case_types)]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum uc_ {}

/// Implementation for bookending unicode characters
impl AsStaticStr for uc_ {

    /// The escape character for the end of a unicode sequence
    ///
    /// If this is changed, make sure to update the characters used in
    /// decoding (see self::internal)
    fn get_char() -> &'static str { "}" }
}

/// Trait for getting the String representation of a Labelled's Name type
pub trait Named {

    /// Returns the label
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # use frunk_core::labelled::*;
    /// # use frunk_core::hlist::*;
    /// # fn main() {
    /// let labelled = label::<Hlist![n, a, m, e], &str>("joe");
    /// assert_eq!(labelled.name(), "name".to_string())
    /// # }
    /// ```
    fn name(&self) -> String;
}

impl <Name: HasRuntimeString, Value> Named for Labelled<Name, Value> {

    fn name(&self) -> String {
        let raw = <Name as HasRuntimeString>::get_string();
        decode_unicode(raw)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Labelled<Name, Type> {
    name: PhantomData<Name>,
    pub value: Type,
}

impl <Name, Type> fmt::Debug for Labelled<Name, Type>
    where
        Type: fmt::Debug,
        Name: HasRuntimeString {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v_debug = format!("{:?}", self.value);
        write!(f, "Labelled{{ name: {}, value: {} }}", self.name(), v_debug)
    }
}

/// Helper function for building a new Labelled value.
///
/// Useful so that users don't need to deal with PhantomData directly.
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
/// # fn main() {
/// let f1 = label::<Hlist![a, g, e], i32>(3);
/// let f2 = label::<Hlist![a, g, e], i32>(3);
/// assert_eq!(f1, f2)
/// # }
/// ```
pub fn label<Label, Value>(value: Value) -> Labelled<Label, Value> {
    Labelled {
        name: PhantomData,
        value: value,
    }
}


#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct LabelledNew<Name, Type> {
    name_type_holder: PhantomData<Name>,
    pub name: &'static str,
    pub value: Type,
}

pub fn build_label_new<Label, Value>(value: Value, name: &'static str) -> LabelledNew<Label, Value> where Label: HList {
    LabelledNew {
        name_type_holder: PhantomData,
        name: name,
        value: value,
    }
}

/// Trait for turning a Labelled HList into an un-labelled HList
pub trait IntoUnlabelled {
    type Output;

    /// Turns the current HList into an unlabelled on.
    ///
    /// Effectively extracts the values held inside the individual Labelled
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # use frunk_core::labelled::*;
    /// # use frunk_core::hlist::*;
    /// # fn main() {
    ///
    /// let labelled_hlist = hlist![
    ///     label::<Hlist![n, a, m, e], _>("joe"),
    ///     label::<Hlist![a, g, e], _>(3)
    /// ];
    ///
    /// let unlabelled = labelled_hlist.into_unlabelled();
    ///
    /// assert_eq!(unlabelled, hlist!["joe", 3])
    /// # }
    /// ```
    fn into_unlabelled(self) -> Self::Output;
}

/// Implementation for HNil
impl IntoUnlabelled for HNil {
    type Output = HNil;
    fn into_unlabelled(self) -> Self::Output {
        self
    }
}

/// Implementation when we have a non-empty HCons holding a label in its head
impl<Label, Value, Tail> IntoUnlabelled for HCons<Labelled<Label, Value>, Tail>
    where Tail: IntoUnlabelled
{
    type Output = HCons<Value, <Tail as IntoUnlabelled>::Output>;

    fn into_unlabelled(self) -> Self::Output {
        HCons {
            head: self.head.value,
            tail: self.tail.into_unlabelled(),
        }
    }
}

/// Returns a string resulting from the concatenation of the types provided
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # use frunk_core::labelled::*;
/// # use frunk_core::hlist::*;
/// # fn main() {
/// assert_eq!(type_string!(a), "a");
/// assert_eq!(type_string!(a,b,c,d), "abcd");
/// assert_eq!(type_string!(a,b,c,d,), "abcd");
/// assert_eq!(type_string!(), "")
/// # }
/// ```
#[macro_export]
macro_rules! type_string {
    // Nothing
    () => { "" };

    // Just a single item
    ($single: ty) => {
        stringify!($single)
    };

    ($($repeated: ty),+) => {
        concat!($( type_string!($repeated)), * )
    };
    // Trailing comma case
    ($($repeated: ty,)+) => {
        type_string!($($repeated), *)
    };

}

#[macro_export]
macro_rules! label_new {
    (($($repeated: ty),*), $value: expr) => {
        $crate::labelled::build_label_new::<Hlist!($($repeated),*),_>($value, type_string!($($repeated),*))
    };
    // trailing comma case
    (($($repeated: ty,)*), $value: expr) => {
        label_new!( ($($repeated),*), $value )
    };
    // We are provided a stable name
    (($($repeated: ty),*), $value: expr, $name: expr) => {
        $crate::labelled::build_label_new::<Hlist!($($repeated),*),_>($value, $name)
    };
    // trailing comma case
    (($($repeated: ty,)*), $value: expr, $name: expr) => {
        label_new!( ($($repeated),*), $value, $name )
    }

}

#[test]
fn test_label_new_building() {
    let l1 = label_new!((a, b, c), 3);
    assert_eq!(l1.value, 3);
    assert_eq!(l1.name, "abc");
    let l2 = label_new!((a, b, c,), 3);
    assert_eq!(l2.value, 3);
    assert_eq!(l2.name, "abc");

    // test named
    let l3 = label_new!((a,b,c), 3, "nope");
    assert_eq!(l3.value, 3);
    assert_eq!(l3.name, "nope");
    let l4 = label_new!((a,b,c,), 3, "nope");
    assert_eq!(l4.value, 3);
    assert_eq!(l4.name, "nope");
}



/// Holds logic internal to this module.
///
/// Do not use any of these traits or methods directly, as they may change or disappear at any time!
mod internal {

    use super::*;

    /// Trait for getting the runtime String representation for a type
    ///
    /// DO NOT implement this trait yourself unless you know what you are doing.
    pub trait HasRuntimeString {
        fn get_string() -> String;
    }

    impl HasRuntimeString for HNil {
        fn get_string() -> String { "".to_string() }
    }

    impl <Char, Tail> HasRuntimeString for HCons<Char, Tail>
        where Char: AsStaticStr,
              Tail: HasRuntimeString {
        fn get_string() -> String {
            format!("{}{}", <Char as AsStaticStr>::get_char(), <Tail as HasRuntimeString>::get_string() )
        }
    }

    // Decoder states
    #[derive(Eq, PartialEq)]
    #[doc(hidden)]
    enum UnicodeDecoderState {
        // Not processing unicode
        Inactive,
        // Just stepped into a unicode block of chars
        JustStarted,
        // Working on the current block of unicode chars
        BuildingUnicodeCodepoint,
    }

    const UNICODE_BEGINS_CHAR: char = '{';
    const UNICODE_ENDING_CHAR: char = '}';
    const UNICODE_CODEPOINT_START: char = 'u';
    const VALID_HEX_CHARS: &'static [char] = &['0','1','2','3','4','5','6','7','8','9', 'A','B','C','D','E','F', 'a','b','c','d','e','f'];

    /// [DO NOT call this method] Given a String, tries to figure out if there are sequences
    /// inside that are encoded as unicode, according to Frunk rules for type-level Strings.
    ///
    /// This inherently makes a lot of assumptions, one of which is that it will not fail ;)
    /// The other is about how different names are encoded into type-level strings, and then
    /// encoded into type-level unicode.
    ///
    /// For more details, see the test immediately following
    pub fn decode_unicode(s_in: String) -> String {
        if !s_in.contains(UNICODE_BEGINS_CHAR) && !s_in.contains(UNICODE_ENDING_CHAR){
            s_in
        } else {
            let mut state = UnicodeDecoderState::Inactive;
            let mut final_string = String::new();
            let mut hex_buffer = String::new();
            let mut u16_buffer: Vec<u16> = vec![];
            for c in s_in.chars() {
                if c == UNICODE_BEGINS_CHAR {
                    state = UnicodeDecoderState::JustStarted;
                } else if state == UnicodeDecoderState::JustStarted && c == UNICODE_CODEPOINT_START {
                    // Start building
                    state = UnicodeDecoderState::BuildingUnicodeCodepoint;
                } else if state == UnicodeDecoderState::BuildingUnicodeCodepoint && c == UNICODE_CODEPOINT_START {
                    // Just finished a codepoint, so convert the hex to u16,
                    // put it into the u16 buffer and clear the hex buffer
                    let as_hex = hex_str_to_u16(&hex_buffer[..]);
                    u16_buffer.push(as_hex);
                    hex_buffer = String::new();
                } else if c == UNICODE_ENDING_CHAR {
                    // We finished the whole block

                    // Convert what's left in the hex buffer to u16,
                    // push it into the u16 buffer, and clear the hex buffer
                    let as_u16 = hex_str_to_u16(&hex_buffer[..]);
                    u16_buffer.push(as_u16);
                    hex_buffer = String::new();

                    // Turn the whole u16 buffer into a single String
                    let new_char = u16_vec_to_string(&u16_buffer[..]);
                    // Push the newly minted char into the final string, an reset
                    // buffer and state
                    final_string = format!("{}{}", final_string, new_char);
                    u16_buffer = vec![];
                    state = UnicodeDecoderState::Inactive;
                } else if state == UnicodeDecoderState::BuildingUnicodeCodepoint && VALID_HEX_CHARS.contains(&c) {
                    hex_buffer.push(c);
                } else if state != UnicodeDecoderState::Inactive {
                    // Something went wrong, let's try to salvage the hex_buffer and reset state
                    final_string = format!("{}{}", final_string, hex_buffer);
                    hex_buffer = String::new();
                    u16_buffer = vec![];
                    state = UnicodeDecoderState::Inactive;
                } else {
                    final_string.push(c);
                }
            }
            final_string
        }
    }

    #[test]
    fn test_decode_unicode(){
        let s = "John_Appleby_123456";
        let processed = decode_unicode(s.to_string());
        assert_eq!(processed, s.to_string());

        let has_unicode = "John_appleby_{u2764ufe0f}_happy";
        let processed_unicode = decode_unicode(has_unicode.to_string());
        assert_eq!(processed_unicode, "John_appleby_\u{2764}\u{fe0f}_happy");

        let all_letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";
        let processed_letters = decode_unicode(all_letters.to_string());
        assert_eq!(processed_letters, all_letters.to_string())
    }

    fn hex_str_to_u16(hex_str: &str) -> u16 {
        u16::from_str_radix(hex_str, 16).unwrap()
    }

    fn u16_vec_to_string(v: &[u16]) -> String {
        String::from_utf16(v).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_construction() {
        let f1 = label::<Hlist!(a, g, e), i32>(3);
        let f2 = label::<Hlist!(a, g, e), i32>(3);
        assert_eq!(f1, f2)
    }

    #[test]
    fn test_anonymous_record_useage() {
        let record = hlist![
            label::<Hlist![n, a, m, e], _>("Joe"),
            label::<Hlist![a, g, e], _>(30)
        ];
        let (name, _): (Labelled<Hlist![n, a, m, e], _>, _) = record.pluck();
        assert_eq!(name.value, "Joe")
    }

    #[test]
    fn test_unlabelling() {
        let labelled_hlist = hlist![
            label::<Hlist![n, a, m, e], &str>("joe"),
            label::<Hlist![a, g, e], i32>(3)];
        let unlabelled = labelled_hlist.into_unlabelled();
        assert_eq!(unlabelled, hlist!["joe", 3])
    }

    #[test]
    fn test_get_string() {
        let labelled = label::<Hlist![n, a, m, e, _1, _2, _3], &str>("joe");
        assert_eq!(labelled.name(), "name123".to_string());

        println!("YO {:?}", labelled)
    }
}
