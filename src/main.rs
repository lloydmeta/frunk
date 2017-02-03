#[macro_use]
extern crate frunk_derives;

trait HelloWorld {
    fn hello_world();
}

#[derive(Generic, HelloWorld)]
struct FrenchToast {
    hi: i32,
    bye: i32
}

#[derive(HelloWorld)]
struct Waffles;

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
}
