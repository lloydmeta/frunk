#[macro_use] // for the hlist macro
extern crate frunk;
extern crate frunk_core;

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
}
