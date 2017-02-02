#![feature(test)]

#[macro_use]
extern crate frunk;
extern crate test;

use frunk_core::hlist::*;
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
fn hlist_into_tuple2_match(b: &mut Bencher) {
    let h = hlist![1, 2, 3.3f32, "hi2", true];
    b.iter(|| {
        let (a, (b, (c, (d, e)))) = h.into_tuple2();
        (a, b, c, d, e)
    })
}


#[bench]
fn hlist_into_hlist_pat_match(b: &mut Bencher) {
    let h = hlist![1, 2, 3.3f32, "hi2", true];
    b.iter(|| {
        let hlist_pat!(a, b, c, d, e) = h;
        (a, b, c, d, e)
    })
}

#[bench]
fn hlist_append(b: &mut Bencher) {
    let h1 = hlist![1, 2, 3.3f32, "hi2", true];
    let h2 = hlist![true, "blue", "varcity"];
    b.iter(|| h1 + h2)
}
