#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| 0)
}
