#![feature(test)]

#[macro_use]
extern crate frunk;
#[macro_use]
extern crate frunk_core;
extern crate test;

use frunk::*;
use test::Bencher;

#[derive(LabelledGeneric)]
struct NewUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(LabelledGeneric)]
struct SavedUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}
#[derive(LabelledGeneric)]
struct JumbledUser<'a> {
    age: usize,
    last_name: &'a str,
    first_name: &'a str,
}

#[bench]
fn labelled_conversion(b: &mut Bencher) {
    b.iter(|| {
        let n_u = NewUser {
            first_name: "Joe",
            last_name: "Schmoe",
            age: 30,
        };
        <SavedUser as LabelledGeneric>::convert_from(n_u)
    })
}
#[bench]
fn sculpted_conversion(b: &mut Bencher) {
    b.iter(|| {
        let n_u = NewUser {
            first_name: "Joe",
            last_name: "Schmoe",
            age: 30,
        };
        <JumbledUser as LabelledGeneric>::transform_from(n_u)
    })
}

#[bench]
fn name(b: &mut Bencher) {
    let field = field!((f, i, r, s, t, __, n, a, m, e), "Joe");
    b.iter(|| field.name)
}
