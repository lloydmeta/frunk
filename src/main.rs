#[macro_use]
extern crate frunk_derives;
extern crate frunk_core;


#[derive(Generic)]
struct FrenchToast<'a> {
    hi: i32,
    bye: i32,
    name: &'a str
}

#[derive(Generic)]
struct Thing<'a>(i32, &'a str);


fn main() {

}
