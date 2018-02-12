#![feature(test)]

extern crate frunk;
extern crate test;

use frunk::semigroup::*;
use test::Bencher;

#[bench]
fn combine_i32(b: &mut Bencher) {
    let x: i32 = 10;
    let y: i32 = 50;
    b.iter(|| x.combine(&y))
}

#[bench]
fn std_add_i32(b: &mut Bencher) {
    let x: i32 = 10;
    let y: i32 = 50;
    b.iter(|| x + y)
}

#[bench]
fn combine_option_string(b: &mut Bencher) {
    let x: Option<String> = Some("hello".to_owned());
    let y: Option<String> = Some(" world".to_owned());
    b.iter(|| x.combine(&y))
}

#[bench]
fn std_add_option_string(b: &mut Bencher) {
    let x: Option<String> = Some("hello".to_owned());
    let y: Option<String> = Some(" world".to_owned());
    b.iter(|| {
        // cloning is required otherwise we get `cannot move out of captured outer variable in an `FnMut` closure` errors
        let a = x.clone();
        let b = y.clone();
        a.and_then(|first| b.map(|second| first + &second))
    })
}
