#![feature(test)]

#[macro_use] extern crate frunk;
extern crate test;

use frunk::validated::*;
use test::Bencher;

fn yah_nah(yah: bool) -> Result<i32, String> {
    if yah {
        Result::Ok(32)
    } else {
        Result::Err("Shiz".to_owned())
    }
}

#[bench]
fn ok_result_into_validated(b: &mut Bencher) {
    let r = yah_nah(true);
    b.iter(|| {
        r.clone().into_validated()
    })
}

#[bench]
fn error_result_into_validated(b: &mut Bencher) {
    let r = yah_nah(false);
    b.iter(|| {
        r.clone().into_validated()
    })
}

#[bench]
fn adding_result_to_validated_all_good(b: &mut Bencher) {
    let v = yah_nah(true).into_validated();
    b.iter(|| {
        v.clone() + yah_nah(true) + yah_nah(true) + yah_nah(true) + yah_nah(true)
    })
}

#[bench]
fn adding_result_to_validated_all_bad(b: &mut Bencher) {
    let v = yah_nah(false).into_validated();
    b.iter(|| {
        v.clone() + yah_nah(false) + yah_nah(false) + yah_nah(false) + yah_nah(false)
    })
}

#[bench]
fn adding_result_to_validated_mixed(b: &mut Bencher) {
    let v = yah_nah(true).into_validated();
    b.iter(|| {
        v.clone() + yah_nah(false) + yah_nah(true) + yah_nah(false) + yah_nah(true)
    })
}

#[bench]
fn adding_validateds(b: &mut Bencher) {
    let v1 = yah_nah(true).into_validated();
    let v2 = yah_nah(true).into_validated();
    let v3 = yah_nah(true).into_validated();
    let v4 = yah_nah(true).into_validated();
    b.iter(|| {
        v1.clone() + v2.clone() + v3.clone() + v4.clone()
    })
}

#[bench]
fn validated_to_result(b: &mut Bencher) {
    let v1 = yah_nah(true).into_validated();
    b.iter(|| {
        v1.clone().into_result()
    })
}