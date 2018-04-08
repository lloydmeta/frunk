#[macro_use] // for the hlist macro
extern crate frunk;
extern crate frunk_core;

#[derive(Generic, Debug, PartialEq)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

fn main() {
    let h = hlist!("Joe", "Blow", 30);
    let p: Person = frunk::from_generic(h);
    assert_eq!(
        p,
        Person {
            first_name: "Joe",
            last_name: "Blow",
            age: 30,
        }
    );
    println!("{}", p.first_name);
}
