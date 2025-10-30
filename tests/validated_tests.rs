#![cfg(feature = "validated")]

extern crate frunk;
extern crate frunk_core;

use frunk::prelude::*;

mod common;
use crate::common::*;

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
    let person: Result<Person, _> = v.into_result().map(frunk::from_generic); // much simpler
    assert_eq!(
        person.unwrap(),
        Person {
            first_name: "James",
            last_name: "James",
            age: 32,
        }
    );
}
