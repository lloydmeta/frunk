extern crate frunk;
#[macro_use] // for the hlist macro
extern crate frunk_core;

use frunk::*; // for the Generic trait and HList
use frunk::validated::*;

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

#[derive(Generic, Debug, PartialEq)]
struct TupleStruct<'a>(&'a str, i32);

#[test]
fn test_struct_from_generic() {
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
    let pres: President = convert_from(a);
    assert_eq!(pres,
               President {
                   first_name: "Steve",
                   last_name: "Cannon",
                   age: 3,
               })
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


// Working with Validated

#[derive(PartialEq, Eq, Debug)]
pub enum YahNah {
    Yah,
    Nah,
}

/// Our Errors
#[derive(PartialEq, Eq, Debug)]
pub enum Nope {
    NameNope,
    AgeNope,
    EmailNope,
}

fn get_name(yah_nah: YahNah) -> Result<&'static str, Nope> {
    match yah_nah {
        YahNah::Yah => Result::Ok("James"),
        _ => Result::Err(Nope::NameNope),
    }
}

fn get_age(yah_nah: YahNah) -> Result<usize, Nope> {
    match yah_nah {
        YahNah::Yah => Result::Ok(32),
        _ => Result::Err(Nope::AgeNope),
    }
}

#[test]
fn test_to_result_ok() {
    let v = get_name(YahNah::Yah).into_validated() + get_name(YahNah::Yah) + get_age(YahNah::Yah);
    let person: Result<Person, _> = v.into_result().map(|h| from_generic(h)); // much simpler
    assert_eq!(person.unwrap(),
               Person {
                   first_name: "James",
                   last_name: "James",
                   age: 32,
               });
}
