#![feature(test)]

#[macro_use] extern crate frunk;
extern crate test;

use frunk::hlist::*;
use test::Bencher;

#[bench]
fn creating_hlist(b: &mut Bencher) {
    b.iter(|| hlist![1, 2, 3.3f32, "hi2", true])
}

#[bench]
fn creating_tuple2(b: &mut Bencher) {
    b.iter(|| (1, (2, (3.3f32, ("hi2", true)))))
}

#[bench]
fn hlist_into_tuple2(b: &mut Bencher) {
    let h = hlist![1, 2, 3.3f32, "hi2", true];
    b.iter(|| h.into_tuple2())
}

#[bench]
fn hlist_append(b: &mut Bencher) {
    let h1 = hlist![1, 2, 3.3f32, "hi2", true];
    let h2 = hlist![true, "blue", "varcity"];
    b.iter(|| h1 + h2 )
}