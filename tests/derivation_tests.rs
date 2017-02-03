extern crate frunk;
#[macro_use] // for the hlist macro
extern crate frunk_core;

use frunk::*; // for the Generic trait and HList

#[derive(Generic, Debug, PartialEq)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(Generic, Debug, PartialEq, Clone)]
struct Strategist<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(Generic, Debug, PartialEq)]
struct President<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[test]
fn test_pub_struct_from_generic() {
    let h = hlist!("Humpty", "Drumpty", 3);
    let p: Person = from_generic(h);
    assert_eq!(p,
               Person {
                   first_name: "Humpty",
                   last_name: "Drumpty",
                   age: 3,
               });
}

#[test]
fn test_pub_struct_into_generic() {
    let p = Person {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let h = into_generic(p);
    assert_eq!(h, hlist!("Humpty", "Drumpty", 3));
}

#[test]
fn test_pub_struct_conversion() {
    let a = Strategist {
        first_name: "Steve",
        last_name: "Cannon",
        age: 3,
    };
    let pres: President = convert_from(a);
    assert_eq!(pres,
               President {
                   first_name: "Steve",
                   last_name: "Cannon",
                   age: 3,
               })
}

#[test]
fn test_pub_struct_conversion_round_trip() {
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
