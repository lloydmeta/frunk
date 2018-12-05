//! Defines a bunch of unit structs with unique types for use by unit tests in frunk.
//!
//! The theme is "I need a bunch of types and I need them NOW!"

#[allow(unused)]
pub mod unit_copy {
    macro_rules! make_unit_struct {
        ($($Name:ident)*) => {$(
            /// Unit-like struct that implements Copy, for tests.
            #[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
            pub struct $Name;
        )*};
    }
    // Feel free to add more as necessary
    make_unit_struct! {A B C D E F}
}

#[allow(unused)]
pub mod unit_move {
    macro_rules! make_unit_struct {
        ($($Name:ident)*) => {$(
            /// Unit-like struct that does not implement Copy, for tests.
            #[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
            pub struct $Name;
        )*};
    }
    // Feel free to add more as necessary
    make_unit_struct! {A B C D E F}
}

#[allow(unused)]
pub mod i32_copy {
    macro_rules! make_unit_struct {
        ($($Name:ident)*) => {$(
            /// Newtype around i32 that implements Copy, for tests.
            #[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
            pub struct $Name(pub i32);
        )*};
    }
    // Feel free to add more as necessary
    make_unit_struct! {X Y Z}
}

#[allow(unused)]
pub mod i32_move {
    macro_rules! make_unit_struct {
        ($($Name:ident)*) => {$(
            /// Newtype around i32 that does not implement Copy, for tests.
            #[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
            pub struct $Name(pub i32);
        )*};
    }
    // Feel free to add more as necessary
    make_unit_struct! {X Y Z}
}
