use frunk::labelled::*;
use frunk::{hlist, LabelledGeneric};

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
struct InternalPhoneNumber {
    emergency: Option<usize>,
    main: usize,
    secondary: Option<usize>,
}

#[derive(LabelledGeneric)]
struct InternalAddress<'a> {
    is_whitelisted: bool,
    name: &'a str,
    phone: InternalPhoneNumber,
}

#[derive(LabelledGeneric)]
struct InternalPerson<'a> {
    name: &'a str,
    age: usize,
    address: InternalAddress<'a>,
    is_banned: bool,
}

#[derive(LabelledGeneric, Debug)]
struct ExternalPhoneNumber {
    main: usize,
}

#[derive(LabelledGeneric, Debug)]
struct ExternalAddress<'a> {
    name: &'a str,
    phone: ExternalPhoneNumber,
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

    let internal_person = InternalPerson {
        name: "John",
        age: 10,
        address: InternalAddress {
            is_whitelisted: true,
            name: "somewhere out there",
            phone: InternalPhoneNumber {
                main: 1234,
                secondary: None,
                emergency: Some(5678),
            },
        },
        is_banned: true,
    };

    let external_person: ExternalPerson = internal_person.transmogrify();
    println!("{:#?}", external_person);

    // mapping over labelled generic representation
    let peep = NewUser {
        first_name: "bo",
        last_name: "peep",
        age: 30,
    };
    let labelled_generic = frunk::into_labelled_generic(peep);

    let _ = labelled_generic.map(hlist![
        |f: Field<_, _>| println!("{}: {}", f.name, f.value),
        |l: Field<_, _>| println!("{}: {}", l.name, l.value),
        |a: Field<_, _>| println!("{}: {}", a.name, a.value),
    ]);
}
