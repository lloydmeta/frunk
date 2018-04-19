#[macro_use] // for the hlist macro
extern crate frunk;
extern crate frunk_core;

#[derive(Generic, Debug, PartialEq)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(Generic, Debug, PartialEq)]
struct Person2<'a> {
    name_first: &'a str,
    name_last: &'a str,
    age_of_person: usize,
}

fn main() {
    let repr = hlist!("Joe", "Blow", 30);
    let person: Person = frunk::from_generic(repr);
    assert_eq!(
        person,
        Person {
            first_name: "Joe",
            last_name: "Blow",
            age: 30,
        }
    );
    println!("{}", person.first_name);

    let older_person = frunk::map_repr(person, |repr| {
        let hlist_pat![first, last, age] = repr;
        hlist![first, last, age * 2]
    });
    assert_eq!(older_person.age, 60);

    let oldest_person = frunk::map_inter(older_person, |p| {
        Person2 { age_of_person: 90, ..p }
    });
    assert_eq!(oldest_person.age, 90);
}
