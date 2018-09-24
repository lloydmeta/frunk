#[macro_use] // for the hlist macro
extern crate frunk;
extern crate frunk_core;

use frunk::labelled::Transmogrifier;

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
struct DeletedUser<'a> {
    last_name: &'a str,
    first_name: &'a str,
    age: usize,
}

#[derive(LabelledGeneric)]
struct InternalAddress<'a> {
    is_banned: bool,
    name: &'a str,
}

#[derive(LabelledGeneric)]
struct InternalPerson<'a> {
    name: &'a str,
    age: usize,
    address: InternalAddress<'a>,
}

#[derive(LabelledGeneric, Debug)]
struct ExternalAddress<'a> {
    name: &'a str,
}

#[derive(LabelledGeneric, Debug)]
struct ExternalPerson<'a> {
    age: usize,
    address: ExternalAddress<'a>,
    name: &'a str,
}

fn main() {
    let n_user = NewUser {
        first_name: "Joe",
        last_name: "Blow",
        age: 30,
    };

    let s_user: SavedUser = frunk::labelled_convert_from(n_user);

    assert_eq!(s_user.first_name, "Joe");
    assert_eq!(s_user.last_name, "Blow");
    assert_eq!(s_user.age, 30);

    let d_user: DeletedUser = frunk::transform_from(s_user);

    assert_eq!(d_user.first_name, "Joe");
    println!("{}", d_user.last_name);

    let internal_user = InternalPerson {
        name: "John",
        age: 10,
        address: InternalAddress {
            is_banned: true,
            name: "somewhere out there",
        },
    };

    let external_user: ExternalPerson = internal_user.transmogrify();
    println!("{:?}", external_user);

}
