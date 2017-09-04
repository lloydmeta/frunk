#![feature(test)]

extern crate test;
extern crate frunk;

use frunk::monoid::*;
use test::Bencher;

#[bench]
fn combine_all_i32(b: &mut Bencher) {
    let v = vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        Some(9),
        Some(10),
    ];
    b.iter(|| combine_all(&v))
}

#[bench]
fn std_add_all_i32(b: &mut Bencher) {
    let v = vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        Some(9),
        Some(10),
    ];
    b.iter(|| {
        v.iter().fold(Some(0), |maybe_acc, maybe_n| {
            maybe_acc.and_then(|acc| maybe_n.map(|n| acc + n))
        })
    })
}
