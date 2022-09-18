use frunk::{convert_from, from_generic, into_generic};
use frunk_core::hlist;

mod common;
use crate::common::*;

#[test]
fn test_struct_from_generic() {
    let h = hlist!("Humpty", "Drumpty", 3);
    let p: Person = from_generic(h);
    assert_eq!(
        p,
        Person {
            first_name: "Humpty",
            last_name: "Drumpty",
            age: 3,
        }
    );
}

#[test]
fn test_tuple_struct_from_generic() {
    let h = hlist!("Drumpty", 3);
    let p: TupleStruct = from_generic(h);
    assert_eq!(p, TupleStruct("Drumpty", 3));
}

#[test]
fn test_struct_into_generic() {
    let p = Person {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let h = into_generic(p);
    assert_eq!(h, hlist!("Humpty", "Drumpty", 3));
}

#[test]
fn test_struct_conversion() {
    let a = Strategist {
        first_name: "Steve",
        last_name: "Cannon",
        age: 3,
    };
    let pres: President = frunk::convert_from(a);
    assert_eq!(
        pres,
        President {
            first_name: "Steve",
            last_name: "Cannon",
            age: 3,
        }
    )
}

#[test]
fn test_struct_conversion_round_trip() {
    let a = Strategist {
        first_name: "Steve",
        last_name: "Cannon",
        age: 3,
    };
    let before = a.clone();
    let p: President = convert_from(a);
    let a_again: Strategist = convert_from(p);
    assert_eq!(a_again, before)
}

#[test]
fn test_mixed_conversions_round_trip() {
    // Both SavedUser and ApiUser derive both Generic and LabelledGeneric
    //
    // Because their field names are different, their LabelledGeneric representations
    // differ, so we can't use the LabelledGeneric typeclass to convert to and fro.
    // Instead, we'll use the Generic typeclass to get the job done.
    let u = SavedUser {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let before = u.clone();
    let au: ApiUser = convert_from(u);
    // let au2 = <ApiUser as LabelledGeneric>::convert_from(u); <-- will fail at compile time
    let u_again: SavedUser = convert_from(au);
    assert_eq!(u_again, before)
}
