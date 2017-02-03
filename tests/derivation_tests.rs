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
