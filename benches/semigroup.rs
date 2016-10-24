#![feature(test)]

extern crate test;
extern crate frunk;

use frunk::semigroup::*;
use test::Bencher;

#[bench]
fn combine_i32(b: &mut Bencher) {
    let x: i32 = 10;
    let y: i32 = 50;
    b.iter(||x.combine(&y))
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
    b.iter(||x.combine(&y))
}

#[bench]
fn std_add_option_string(b: &mut Bencher) {
    b.iter(|| {
        // Run into "cannot move out of captured outer variable in an `FnMut` closure" error
        // if these are not declared inside the closure. Not entirely sure why
        let x: Option<String> = Some("hello".to_owned());
        let y: Option<String> = Some(" world".to_owned());
        x.and_then(|first| {
            y.map(|second| first + &second )
        })
    })
}