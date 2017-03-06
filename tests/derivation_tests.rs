extern crate frunk;
#[macro_use] // for the hlist macro
extern crate frunk_core;

use frunk_core::labelled::*;
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

#[derive(LabelledGeneric, Debug, PartialEq, Clone)]
struct NewUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(LabelledGeneric, Generic, Debug, PartialEq, Clone)]
struct SavedUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[allow(non_snake_case)]
#[derive(LabelledGeneric, Generic, Debug, PartialEq)]
struct ApiUser<'a> {
    FirstName: &'a str,
    LastName: &'a str,
    Age: usize,
}

#[allow(non_snake_case)]
#[derive(LabelledGeneric, Debug, PartialEq)]
struct SuperLongField {
    abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789: i32
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
    let pres = <President as Generic>::convert_from(a);
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
    let p: President = <President as Generic>::convert_from(a);
    let a_again =  <Strategist as Generic>::convert_from(p);
    assert_eq!(a_again, before)
}

#[test]
fn test_struct_from_labelled_generic() {
    let h = hlist![label!((f, i, r, s, t, __, n, a, m, e), "Humpty"),
                   label!((l, a, s, t, __, n, a, m, e), "Drumpty"),
                   label!((a, g, e), 3)];
    let u: NewUser = from_labelled_generic(h);
    assert_eq!(u,
               NewUser {
                   first_name: "Humpty",
                   last_name: "Drumpty",
                   age: 3,
               });
}

#[test]
fn test_labelled_generic_names(){
    let u = NewUser {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let h = into_labelled_generic(u);
    let l_name: &Labelled<(l,a,s,t,__,n,a,m,e), _> = h.get();
    assert_eq!(l_name.name, "last_name")
}

#[test]
fn test_struct_into_labelled_generic() {
    let u = NewUser {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let h = into_labelled_generic(u);
    assert_eq!(h,
               hlist![label!((f, i, r, s, t, __, n, a, m, e), "Humpty", "first_name"),
                      label!((l, a, s, t, __, n, a, m, e),"Drumpty", "last_name"),
                      label!((a, g, e), 3)]);
}

#[test]
fn test_stuct_conversion_round_trip_labelled() {
    let u = NewUser {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let before = u.clone();
    let su = <SavedUser as LabelledGeneric>::convert_from(u);
    let u_again = <NewUser as LabelledGeneric>::convert_from(su);
    assert_eq!(u_again, before)
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
    let au = <ApiUser as Generic>::convert_from(u);
    // let au2 = <ApiUser as LabelledGeneric>::convert_from(u); <-- will fail at compile time
    let u_again = <SavedUser as Generic>::convert_from(au);
    assert_eq!(u_again, before)
}


#[derive(LabelledGeneric)]
struct NormalUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize
}
// Fields are jumbled :(
#[derive(LabelledGeneric)]
struct JumbledUser<'a> {
    last_name: &'a str,
    age: usize,
    first_name: &'a str,
}

#[test]
fn test_reshaped_labelled_generic_conversion() {

    let n_u = NormalUser {
        first_name: "Moe",
        last_name: "Ali",
        age: 30
    };
    // Convert to labelled-generic representation
    let n_gen = into_labelled_generic(n_u);
    // Reshape the labelled generic to fit the JumbledUser's generic Repr
    let (jumbled_gen, _): (<JumbledUser as LabelledGeneric>::Repr, _) = n_gen.sculpt();
    let j_u: JumbledUser = from_labelled_generic(jumbled_gen); // Done

    assert_eq!(j_u.first_name, "Moe");
    assert_eq!(j_u.last_name, "Ali");
    assert_eq!(j_u.age, 30)
}

#[test]
fn test_aligned_labelled_convert_from() {

    let n_u = NormalUser {
        first_name: "Moe",
        last_name: "Ali",
        age: 30
    };
    // even less boilerplate than before
    let j_u: JumbledUser = sculpted_convert_from(n_u); // Done

    assert_eq!(j_u.first_name, "Moe");
    assert_eq!(j_u.last_name, "Ali");
    assert_eq!(j_u.age, 30)
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
