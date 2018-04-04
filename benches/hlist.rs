#![feature(test)]

#[macro_use]
extern crate frunk_core;
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

#[bench]
fn hlist_mapping_consuming(b: &mut Bencher) {
    let h1 = hlist![1, 2, 3.3f32, "hi2", true];
    b.iter(|| {
        h1.map(
            hlist![
              |i| i + 1,
              |i| i + 2,
              |i| i + 3f32,
              |s| s,
              |b: bool| !b,
            ]
        );
    });
}

#[bench]
fn hlist_mapping_non_consuming(b: &mut Bencher) {
    let h1 = hlist![1, 2, 3.3f32, "hi2", true];
    b.iter(|| {
        h1.to_ref().map(
            hlist![
              |&i| i + 1,
              |&i| i + 2,
              |&i| i + 3f32,
              |&s| s,
              |&b: &bool| !b,
            ]
        );
    });
}
