#[macro_use]
extern crate frunk_derives;
#[macro_use]
extern crate frunk_core;

use frunk_core::hlist::*;
use frunk_core::generic::Generic;

#[derive(Generic, Debug, PartialEq)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[test]
fn test_pub_struct_from_generic() {
    let h = hlist!("james", "may", 13);
    let p = <Person as Generic>::from_generic(h);
    assert_eq!(p,
               Person {
                   first_name: "james",
                   last_name: "may",
                   age: 13,
               });
}
