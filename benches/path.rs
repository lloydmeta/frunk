#![feature(test)]
#![allow(dead_code, unused)]

#[macro_use]
extern crate frunk;
#[macro_use]
extern crate frunk_core;
#[macro_use]
extern crate frunk_proc_macros;
extern crate test;

use frunk::labelled::Transmogrifier;
use std::convert::From;
use test::Bencher;

#[derive(LabelledGeneric)]
struct Inner5 {
    v: isize,
}

#[derive(LabelledGeneric)]
struct Inner4 {
    v: Inner5,
}

#[derive(LabelledGeneric)]
struct Inner3 {
    v: Inner4,
}
#[derive(LabelledGeneric)]
struct Inner2 {
    v: Inner3,
}

#[derive(LabelledGeneric)]
struct Outer {
    v: Inner2,
}

impl Outer {
    fn new() -> Outer {
        Outer {
            v: Inner2 {
                v: Inner3 {
                    v: Inner4 { v: Inner5 { v: 3 } },
                },
            },
        }
    }
}

#[bench]
fn normal_path_read_value(b: &mut Bencher) {
    b.iter(|| {
        let o = Outer::new();
        let v = o.v.v.v.v.v;
        let r = v + 1;
        r
    })
}

#[bench]
fn lens_path_read_value(b: &mut Bencher) {
    let p = path!(v.v.v.v.v);
    b.iter(|| {
        let o = Outer::new();
        let v = p.get(o);
        let r = v + 1;
        r
    })
}

#[bench]
fn normal_path_read_ref(b: &mut Bencher) {
    b.iter(|| {
        let o = Outer::new();
        let v = &o.v.v.v.v.v;
        let r = v + 1;
        r
    })
}

#[bench]
fn lens_path_read_ref(b: &mut Bencher) {
    let p = path!(v.v.v.v.v);
    b.iter(|| {
        let o = Outer::new();
        let v = p.get(&o);
        let r = v + 1;
        r
    })
}

#[bench]
fn normal_path_read_mut(b: &mut Bencher) {
    b.iter(|| {
        let mut o = Outer::new();
        let v = &mut o.v.v.v.v.v;
        *v = 999;
        let r = *v + 1;
        r
    })
}

#[bench]
fn lens_path_read_mut(b: &mut Bencher) {
    let p = path!(v.v.v.v.v);
    b.iter(|| {
        let mut o = Outer::new();
        *p.get(&mut o) = 9999;
        let r = p.get(&o) + 1;
        r
    })
}
