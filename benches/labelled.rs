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
    age: usize
}

#[derive(LabelledGeneric)]
struct SavedUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize
}

#[bench]
fn labelled_conversion(b: &mut Bencher) {
    b.iter(|| {
        let n_u = NewUser {
            first_name: "Joe",
            last_name: "Schmoe",
            age: 30
        };
        <SavedUser as LabelledGeneric>::sculpted_convert_from(n_u)
    })
}

#[bench]
fn name(b: &mut Bencher) {
    let field = field!((f,i,r,s,t,__,n,a,m,e), 30);
    b.iter(|| {
        field.name
    })
}