#![feature(test)]
#![allow(dead_code, unused)]

#[macro_use]
extern crate frunk;
#[macro_use]
extern crate frunk_core;
extern crate test;

use frunk::labelled::Transmogrifier;
use std::convert::From;
use test::Bencher;

#[derive(LabelledGeneric)]
struct DeepLayer5a {
    a: i8,
    b: i32,
    c: f64,
    d: usize,
    e: String,
}

#[derive(LabelledGeneric)]
struct DeepLayer5b {
    f: i8,
    g: i32,
    h: f64,
    i: usize,
    j: String,
}

#[derive(LabelledGeneric)]
struct DeepLayer5c {
    k: i8,
    l: i32,
    m: f64,
    n: usize,
    o: String,
}

#[derive(LabelledGeneric)]
struct DeepLayer5d {
    p: i8,
    q: i32,
    r: f64,
    s: usize,
    t: String,
}

#[derive(LabelledGeneric)]
struct DeepLayer5e {
    u: i8,
    v: i32,
    w: f64,
    x: usize,
    y: String,
}

impl DeepLayer5a {
    fn new() -> DeepLayer5a {
        DeepLayer5a {
            a: 6,
            b: 7,
            c: 8f64,
            d: 9,
            e: "hello world".to_string(),
        }
    }
}

impl DeepLayer5b {
    fn new() -> DeepLayer5b {
        DeepLayer5b {
            f: 6,
            g: 7,
            h: 8f64,
            i: 9,
            j: "hello world".to_string(),
        }
    }
}

impl DeepLayer5c {
    fn new() -> DeepLayer5c {
        DeepLayer5c {
            k: 6,
            l: 7,
            m: 8f64,
            n: 9,
            o: "hello world".to_string(),
        }
    }
}

impl DeepLayer5d {
    fn new() -> DeepLayer5d {
        DeepLayer5d {
            p: 6,
            q: 7,
            r: 8f64,
            s: 9,
            t: "hello world".to_string(),
        }
    }
}

impl DeepLayer5e {
    fn new() -> DeepLayer5e {
        DeepLayer5e {
            u: 6,
            v: 7,
            w: 8f64,
            x: 9,
            y: "hello world".to_string(),
        }
    }
}

// Layer 5 reversed

#[derive(LabelledGeneric)]
struct DeepLayer5aReversed {
    e: String,
    d: usize,
    c: f64,
    b: i32,
    a: i8,
}

#[derive(LabelledGeneric)]
struct DeepLayer5bReversed {
    j: String,
    i: usize,
    h: f64,
    g: i32,
    f: i8,
}

#[derive(LabelledGeneric)]
struct DeepLayer5cReversed {
    o: String,
    n: usize,
    m: f64,
    l: i32,
    k: i8,
}

#[derive(LabelledGeneric)]
struct DeepLayer5dReversed {
    t: String,
    s: usize,
    r: f64,
    q: i32,
    p: i8,
}

#[derive(LabelledGeneric)]
struct DeepLayer5eReversed {
    y: String,
    x: usize,
    w: f64,
    v: i32,
    u: i8,
}

impl DeepLayer5aReversed {
    fn new() -> DeepLayer5aReversed {
        DeepLayer5aReversed {
            a: 6,
            b: 7,
            c: 8f64,
            d: 9,
            e: "hello world".to_string(),
        }
    }
}

impl DeepLayer5bReversed {
    fn new() -> DeepLayer5bReversed {
        DeepLayer5bReversed {
            f: 6,
            g: 7,
            h: 8f64,
            i: 9,
            j: "hello world".to_string(),
        }
    }
}

impl DeepLayer5cReversed {
    fn new() -> DeepLayer5cReversed {
        DeepLayer5cReversed {
            k: 6,
            l: 7,
            m: 8f64,
            n: 9,
            o: "hello world".to_string(),
        }
    }
}

impl DeepLayer5dReversed {
    fn new() -> DeepLayer5dReversed {
        DeepLayer5dReversed {
            p: 6,
            q: 7,
            r: 8f64,
            s: 9,
            t: "hello world".to_string(),
        }
    }
}

impl DeepLayer5eReversed {
    fn new() -> DeepLayer5eReversed {
        DeepLayer5eReversed {
            u: 6,
            v: 7,
            w: 8f64,
            x: 9,
            y: "hello world".to_string(),
        }
    }
}

impl From<DeepLayer5a> for DeepLayer5aReversed {
    fn from(o: DeepLayer5a) -> Self {
        DeepLayer5aReversed {
            a: o.a,
            b: o.b,
            c: o.c,
            d: o.d,
            e: o.e,
        }
    }
}

impl From<DeepLayer5b> for DeepLayer5bReversed {
    fn from(o: DeepLayer5b) -> Self {
        DeepLayer5bReversed {
            f: o.f,
            g: o.g,
            h: o.h,
            i: o.i,
            j: o.j,
        }
    }
}

impl From<DeepLayer5c> for DeepLayer5cReversed {
    fn from(o: DeepLayer5c) -> Self {
        DeepLayer5cReversed {
            k: o.k,
            l: o.l,
            m: o.m,
            n: o.n,
            o: o.o,
        }
    }
}

impl From<DeepLayer5d> for DeepLayer5dReversed {
    fn from(o: DeepLayer5d) -> Self {
        DeepLayer5dReversed {
            p: o.p,
            q: o.q,
            r: o.r,
            s: o.s,
            t: o.t,
        }
    }
}

impl From<DeepLayer5e> for DeepLayer5eReversed {
    fn from(o: DeepLayer5e) -> Self {
        DeepLayer5eReversed {
            u: o.u,
            v: o.v,
            w: o.w,
            x: o.x,
            y: o.y,
        }
    }
}

// Layer 4

#[derive(LabelledGeneric)]
struct DeepLayer4a {
    a: DeepLayer5a,
    b: DeepLayer5b,
    c: DeepLayer5c,
    d: DeepLayer5d,
    e: DeepLayer5e,
}

#[derive(LabelledGeneric)]
struct DeepLayer4b {
    f: DeepLayer5a,
    g: DeepLayer5b,
    h: DeepLayer5c,
    i: DeepLayer5d,
    j: DeepLayer5e,
}

#[derive(LabelledGeneric)]
struct DeepLayer4c {
    k: DeepLayer5a,
    l: DeepLayer5b,
    m: DeepLayer5c,
    n: DeepLayer5d,
    o: DeepLayer5e,
}

#[derive(LabelledGeneric)]
struct DeepLayer4d {
    p: DeepLayer5a,
    q: DeepLayer5b,
    r: DeepLayer5c,
    s: DeepLayer5d,
    t: DeepLayer5e,
}

#[derive(LabelledGeneric)]
struct DeepLayer4e {
    u: DeepLayer5a,
    v: DeepLayer5b,
    w: DeepLayer5c,
    x: DeepLayer5d,
    y: DeepLayer5e,
}

impl DeepLayer4a {
    fn new() -> DeepLayer4a {
        DeepLayer4a {
            a: DeepLayer5a::new(),
            b: DeepLayer5b::new(),
            c: DeepLayer5c::new(),
            d: DeepLayer5d::new(),
            e: DeepLayer5e::new(),
        }
    }
}

impl DeepLayer4b {
    fn new() -> DeepLayer4b {
        DeepLayer4b {
            f: DeepLayer5a::new(),
            g: DeepLayer5b::new(),
            h: DeepLayer5c::new(),
            i: DeepLayer5d::new(),
            j: DeepLayer5e::new(),
        }
    }
}

impl DeepLayer4c {
    fn new() -> DeepLayer4c {
        DeepLayer4c {
            k: DeepLayer5a::new(),
            l: DeepLayer5b::new(),
            m: DeepLayer5c::new(),
            n: DeepLayer5d::new(),
            o: DeepLayer5e::new(),
        }
    }
}

impl DeepLayer4d {
    fn new() -> DeepLayer4d {
        DeepLayer4d {
            p: DeepLayer5a::new(),
            q: DeepLayer5b::new(),
            r: DeepLayer5c::new(),
            s: DeepLayer5d::new(),
            t: DeepLayer5e::new(),
        }
    }
}

impl DeepLayer4e {
    fn new() -> DeepLayer4e {
        DeepLayer4e {
            u: DeepLayer5a::new(),
            v: DeepLayer5b::new(),
            w: DeepLayer5c::new(),
            x: DeepLayer5d::new(),
            y: DeepLayer5e::new(),
        }
    }
}

// Layer 4 Reversed

#[derive(LabelledGeneric)]
struct DeepLayer4aReversed {
    e: DeepLayer5eReversed,
    d: DeepLayer5dReversed,
    c: DeepLayer5cReversed,
    b: DeepLayer5bReversed,
    a: DeepLayer5aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer4bReversed {
    j: DeepLayer5eReversed,
    i: DeepLayer5dReversed,
    h: DeepLayer5cReversed,
    g: DeepLayer5bReversed,
    f: DeepLayer5aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer4cReversed {
    o: DeepLayer5eReversed,
    n: DeepLayer5dReversed,
    m: DeepLayer5cReversed,
    l: DeepLayer5bReversed,
    k: DeepLayer5aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer4dReversed {
    t: DeepLayer5eReversed,
    s: DeepLayer5dReversed,
    r: DeepLayer5cReversed,
    q: DeepLayer5bReversed,
    p: DeepLayer5aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer4eReversed {
    y: DeepLayer5eReversed,
    x: DeepLayer5dReversed,
    w: DeepLayer5cReversed,
    v: DeepLayer5bReversed,
    u: DeepLayer5aReversed,
}

impl DeepLayer4aReversed {
    fn new() -> DeepLayer4aReversed {
        DeepLayer4aReversed {
            a: DeepLayer5aReversed::new(),
            b: DeepLayer5bReversed::new(),
            c: DeepLayer5cReversed::new(),
            d: DeepLayer5dReversed::new(),
            e: DeepLayer5eReversed::new(),
        }
    }
}

impl DeepLayer4bReversed {
    fn new() -> DeepLayer4bReversed {
        DeepLayer4bReversed {
            f: DeepLayer5aReversed::new(),
            g: DeepLayer5bReversed::new(),
            h: DeepLayer5cReversed::new(),
            i: DeepLayer5dReversed::new(),
            j: DeepLayer5eReversed::new(),
        }
    }
}

impl DeepLayer4cReversed {
    fn new() -> DeepLayer4cReversed {
        DeepLayer4cReversed {
            k: DeepLayer5aReversed::new(),
            l: DeepLayer5bReversed::new(),
            m: DeepLayer5cReversed::new(),
            n: DeepLayer5dReversed::new(),
            o: DeepLayer5eReversed::new(),
        }
    }
}

impl DeepLayer4dReversed {
    fn new() -> DeepLayer4dReversed {
        DeepLayer4dReversed {
            p: DeepLayer5aReversed::new(),
            q: DeepLayer5bReversed::new(),
            r: DeepLayer5cReversed::new(),
            s: DeepLayer5dReversed::new(),
            t: DeepLayer5eReversed::new(),
        }
    }
}

impl DeepLayer4eReversed {
    fn new() -> DeepLayer4eReversed {
        DeepLayer4eReversed {
            u: DeepLayer5aReversed::new(),
            v: DeepLayer5bReversed::new(),
            w: DeepLayer5cReversed::new(),
            x: DeepLayer5dReversed::new(),
            y: DeepLayer5eReversed::new(),
        }
    }
}

impl From<DeepLayer4a> for DeepLayer4aReversed {
    fn from(o: DeepLayer4a) -> Self {
        DeepLayer4aReversed {
            a: o.a.into(),
            b: o.b.into(),
            c: o.c.into(),
            d: o.d.into(),
            e: o.e.into(),
        }
    }
}

impl From<DeepLayer4b> for DeepLayer4bReversed {
    fn from(o: DeepLayer4b) -> Self {
        DeepLayer4bReversed {
            f: o.f.into(),
            g: o.g.into(),
            h: o.h.into(),
            i: o.i.into(),
            j: o.j.into(),
        }
    }
}

impl From<DeepLayer4c> for DeepLayer4cReversed {
    fn from(o: DeepLayer4c) -> Self {
        DeepLayer4cReversed {
            k: o.k.into(),
            l: o.l.into(),
            m: o.m.into(),
            n: o.n.into(),
            o: o.o.into(),
        }
    }
}

impl From<DeepLayer4d> for DeepLayer4dReversed {
    fn from(o: DeepLayer4d) -> Self {
        DeepLayer4dReversed {
            p: o.p.into(),
            q: o.q.into(),
            r: o.r.into(),
            s: o.s.into(),
            t: o.t.into(),
        }
    }
}

impl From<DeepLayer4e> for DeepLayer4eReversed {
    fn from(o: DeepLayer4e) -> Self {
        DeepLayer4eReversed {
            u: o.u.into(),
            v: o.v.into(),
            w: o.w.into(),
            x: o.x.into(),
            y: o.y.into(),
        }
    }
}

// Layer 3

#[derive(LabelledGeneric)]
struct DeepLayer3a {
    a: DeepLayer4a,
    b: DeepLayer4b,
    c: DeepLayer4c,
    d: DeepLayer4d,
    e: DeepLayer4e,
}

#[derive(LabelledGeneric)]
struct DeepLayer3b {
    f: DeepLayer4a,
    g: DeepLayer4b,
    h: DeepLayer4c,
    i: DeepLayer4d,
    j: DeepLayer4e,
}

#[derive(LabelledGeneric)]
struct DeepLayer3c {
    k: DeepLayer4a,
    l: DeepLayer4b,
    m: DeepLayer4c,
    n: DeepLayer4d,
    o: DeepLayer4e,
}

#[derive(LabelledGeneric)]
struct DeepLayer3d {
    p: DeepLayer4a,
    q: DeepLayer4b,
    r: DeepLayer4c,
    s: DeepLayer4d,
    t: DeepLayer4e,
}

#[derive(LabelledGeneric)]
struct DeepLayer3e {
    u: DeepLayer4a,
    v: DeepLayer4b,
    w: DeepLayer4c,
    x: DeepLayer4d,
    y: DeepLayer4e,
}

impl DeepLayer3a {
    fn new() -> DeepLayer3a {
        DeepLayer3a {
            a: DeepLayer4a::new(),
            b: DeepLayer4b::new(),
            c: DeepLayer4c::new(),
            d: DeepLayer4d::new(),
            e: DeepLayer4e::new(),
        }
    }
}

impl DeepLayer3b {
    fn new() -> DeepLayer3b {
        DeepLayer3b {
            f: DeepLayer4a::new(),
            g: DeepLayer4b::new(),
            h: DeepLayer4c::new(),
            i: DeepLayer4d::new(),
            j: DeepLayer4e::new(),
        }
    }
}

impl DeepLayer3c {
    fn new() -> DeepLayer3c {
        DeepLayer3c {
            k: DeepLayer4a::new(),
            l: DeepLayer4b::new(),
            m: DeepLayer4c::new(),
            n: DeepLayer4d::new(),
            o: DeepLayer4e::new(),
        }
    }
}

impl DeepLayer3d {
    fn new() -> DeepLayer3d {
        DeepLayer3d {
            p: DeepLayer4a::new(),
            q: DeepLayer4b::new(),
            r: DeepLayer4c::new(),
            s: DeepLayer4d::new(),
            t: DeepLayer4e::new(),
        }
    }
}

impl DeepLayer3e {
    fn new() -> DeepLayer3e {
        DeepLayer3e {
            u: DeepLayer4a::new(),
            v: DeepLayer4b::new(),
            w: DeepLayer4c::new(),
            x: DeepLayer4d::new(),
            y: DeepLayer4e::new(),
        }
    }
}

// Layer 3 Reversed

#[derive(LabelledGeneric)]
struct DeepLayer3aReversed {
    e: DeepLayer4eReversed,
    d: DeepLayer4dReversed,
    c: DeepLayer4cReversed,
    b: DeepLayer4bReversed,
    a: DeepLayer4aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer3bReversed {
    j: DeepLayer4eReversed,
    i: DeepLayer4dReversed,
    h: DeepLayer4cReversed,
    g: DeepLayer4bReversed,
    f: DeepLayer4aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer3cReversed {
    o: DeepLayer4eReversed,
    n: DeepLayer4dReversed,
    m: DeepLayer4cReversed,
    l: DeepLayer4bReversed,
    k: DeepLayer4aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer3dReversed {
    t: DeepLayer4eReversed,
    s: DeepLayer4dReversed,
    r: DeepLayer4cReversed,
    q: DeepLayer4bReversed,
    p: DeepLayer4aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer3eReversed {
    y: DeepLayer4eReversed,
    x: DeepLayer4dReversed,
    w: DeepLayer4cReversed,
    v: DeepLayer4bReversed,
    u: DeepLayer4aReversed,
}

impl DeepLayer3aReversed {
    fn new() -> DeepLayer3aReversed {
        DeepLayer3aReversed {
            a: DeepLayer4aReversed::new(),
            b: DeepLayer4bReversed::new(),
            c: DeepLayer4cReversed::new(),
            d: DeepLayer4dReversed::new(),
            e: DeepLayer4eReversed::new(),
        }
    }
}

impl DeepLayer3bReversed {
    fn new() -> DeepLayer3bReversed {
        DeepLayer3bReversed {
            f: DeepLayer4aReversed::new(),
            g: DeepLayer4bReversed::new(),
            h: DeepLayer4cReversed::new(),
            i: DeepLayer4dReversed::new(),
            j: DeepLayer4eReversed::new(),
        }
    }
}

impl DeepLayer3cReversed {
    fn new() -> DeepLayer3cReversed {
        DeepLayer3cReversed {
            k: DeepLayer4aReversed::new(),
            l: DeepLayer4bReversed::new(),
            m: DeepLayer4cReversed::new(),
            n: DeepLayer4dReversed::new(),
            o: DeepLayer4eReversed::new(),
        }
    }
}

impl DeepLayer3dReversed {
    fn new() -> DeepLayer3dReversed {
        DeepLayer3dReversed {
            p: DeepLayer4aReversed::new(),
            q: DeepLayer4bReversed::new(),
            r: DeepLayer4cReversed::new(),
            s: DeepLayer4dReversed::new(),
            t: DeepLayer4eReversed::new(),
        }
    }
}

impl DeepLayer3eReversed {
    fn new() -> DeepLayer3eReversed {
        DeepLayer3eReversed {
            u: DeepLayer4aReversed::new(),
            v: DeepLayer4bReversed::new(),
            w: DeepLayer4cReversed::new(),
            x: DeepLayer4dReversed::new(),
            y: DeepLayer4eReversed::new(),
        }
    }
}

impl From<DeepLayer3a> for DeepLayer3aReversed {
    fn from(o: DeepLayer3a) -> Self {
        DeepLayer3aReversed {
            a: o.a.into(),
            b: o.b.into(),
            c: o.c.into(),
            d: o.d.into(),
            e: o.e.into(),
        }
    }
}

impl From<DeepLayer3b> for DeepLayer3bReversed {
    fn from(o: DeepLayer3b) -> Self {
        DeepLayer3bReversed {
            f: o.f.into(),
            g: o.g.into(),
            h: o.h.into(),
            i: o.i.into(),
            j: o.j.into(),
        }
    }
}

impl From<DeepLayer3c> for DeepLayer3cReversed {
    fn from(o: DeepLayer3c) -> Self {
        DeepLayer3cReversed {
            k: o.k.into(),
            l: o.l.into(),
            m: o.m.into(),
            n: o.n.into(),
            o: o.o.into(),
        }
    }
}

impl From<DeepLayer3d> for DeepLayer3dReversed {
    fn from(o: DeepLayer3d) -> Self {
        DeepLayer3dReversed {
            p: o.p.into(),
            q: o.q.into(),
            r: o.r.into(),
            s: o.s.into(),
            t: o.t.into(),
        }
    }
}

impl From<DeepLayer3e> for DeepLayer3eReversed {
    fn from(o: DeepLayer3e) -> Self {
        DeepLayer3eReversed {
            u: o.u.into(),
            v: o.v.into(),
            w: o.w.into(),
            x: o.x.into(),
            y: o.y.into(),
        }
    }
}

// Layer 2

#[derive(LabelledGeneric)]
struct DeepLayer2a {
    a: DeepLayer3a,
    b: DeepLayer3b,
    c: DeepLayer3c,
    d: DeepLayer3d,
    e: DeepLayer3e,
}

#[derive(LabelledGeneric)]
struct DeepLayer2b {
    f: DeepLayer3a,
    g: DeepLayer3b,
    h: DeepLayer3c,
    i: DeepLayer3d,
    j: DeepLayer3e,
}

#[derive(LabelledGeneric)]
struct DeepLayer2c {
    k: DeepLayer3a,
    l: DeepLayer3b,
    m: DeepLayer3c,
    n: DeepLayer3d,
    o: DeepLayer3e,
}

#[derive(LabelledGeneric)]
struct DeepLayer2d {
    p: DeepLayer3a,
    q: DeepLayer3b,
    r: DeepLayer3c,
    s: DeepLayer3d,
    t: DeepLayer3e,
}

#[derive(LabelledGeneric)]
struct DeepLayer2e {
    u: DeepLayer3a,
    v: DeepLayer3b,
    w: DeepLayer3c,
    x: DeepLayer3d,
    y: DeepLayer3e,
}

impl DeepLayer2a {
    fn new() -> DeepLayer2a {
        DeepLayer2a {
            a: DeepLayer3a::new(),
            b: DeepLayer3b::new(),
            c: DeepLayer3c::new(),
            d: DeepLayer3d::new(),
            e: DeepLayer3e::new(),
        }
    }
}

impl DeepLayer2b {
    fn new() -> DeepLayer2b {
        DeepLayer2b {
            f: DeepLayer3a::new(),
            g: DeepLayer3b::new(),
            h: DeepLayer3c::new(),
            i: DeepLayer3d::new(),
            j: DeepLayer3e::new(),
        }
    }
}

impl DeepLayer2c {
    fn new() -> DeepLayer2c {
        DeepLayer2c {
            k: DeepLayer3a::new(),
            l: DeepLayer3b::new(),
            m: DeepLayer3c::new(),
            n: DeepLayer3d::new(),
            o: DeepLayer3e::new(),
        }
    }
}

impl DeepLayer2d {
    fn new() -> DeepLayer2d {
        DeepLayer2d {
            p: DeepLayer3a::new(),
            q: DeepLayer3b::new(),
            r: DeepLayer3c::new(),
            s: DeepLayer3d::new(),
            t: DeepLayer3e::new(),
        }
    }
}

impl DeepLayer2e {
    fn new() -> DeepLayer2e {
        DeepLayer2e {
            u: DeepLayer3a::new(),
            v: DeepLayer3b::new(),
            w: DeepLayer3c::new(),
            x: DeepLayer3d::new(),
            y: DeepLayer3e::new(),
        }
    }
}

// Layer 2 Reversed

#[derive(LabelledGeneric)]
struct DeepLayer2aReversed {
    e: DeepLayer3eReversed,
    d: DeepLayer3dReversed,
    c: DeepLayer3cReversed,
    b: DeepLayer3bReversed,
    a: DeepLayer3aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer2bReversed {
    j: DeepLayer3eReversed,
    i: DeepLayer3dReversed,
    h: DeepLayer3cReversed,
    g: DeepLayer3bReversed,
    f: DeepLayer3aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer2cReversed {
    o: DeepLayer3eReversed,
    n: DeepLayer3dReversed,
    m: DeepLayer3cReversed,
    l: DeepLayer3bReversed,
    k: DeepLayer3aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer2dReversed {
    t: DeepLayer3eReversed,
    s: DeepLayer3dReversed,
    r: DeepLayer3cReversed,
    q: DeepLayer3bReversed,
    p: DeepLayer3aReversed,
}

#[derive(LabelledGeneric)]
struct DeepLayer2eReversed {
    y: DeepLayer3eReversed,
    x: DeepLayer3dReversed,
    w: DeepLayer3cReversed,
    v: DeepLayer3bReversed,
    u: DeepLayer3aReversed,
}

impl DeepLayer2aReversed {
    fn new() -> DeepLayer2aReversed {
        DeepLayer2aReversed {
            a: DeepLayer3aReversed::new(),
            b: DeepLayer3bReversed::new(),
            c: DeepLayer3cReversed::new(),
            d: DeepLayer3dReversed::new(),
            e: DeepLayer3eReversed::new(),
        }
    }
}

impl DeepLayer2bReversed {
    fn new() -> DeepLayer2bReversed {
        DeepLayer2bReversed {
            f: DeepLayer3aReversed::new(),
            g: DeepLayer3bReversed::new(),
            h: DeepLayer3cReversed::new(),
            i: DeepLayer3dReversed::new(),
            j: DeepLayer3eReversed::new(),
        }
    }
}

impl DeepLayer2cReversed {
    fn new() -> DeepLayer2cReversed {
        DeepLayer2cReversed {
            k: DeepLayer3aReversed::new(),
            l: DeepLayer3bReversed::new(),
            m: DeepLayer3cReversed::new(),
            n: DeepLayer3dReversed::new(),
            o: DeepLayer3eReversed::new(),
        }
    }
}

impl DeepLayer2dReversed {
    fn new() -> DeepLayer2dReversed {
        DeepLayer2dReversed {
            p: DeepLayer3aReversed::new(),
            q: DeepLayer3bReversed::new(),
            r: DeepLayer3cReversed::new(),
            s: DeepLayer3dReversed::new(),
            t: DeepLayer3eReversed::new(),
        }
    }
}

impl DeepLayer2eReversed {
    fn new() -> DeepLayer2eReversed {
        DeepLayer2eReversed {
            u: DeepLayer3aReversed::new(),
            v: DeepLayer3bReversed::new(),
            w: DeepLayer3cReversed::new(),
            x: DeepLayer3dReversed::new(),
            y: DeepLayer3eReversed::new(),
        }
    }
}

impl From<DeepLayer2a> for DeepLayer2aReversed {
    fn from(o: DeepLayer2a) -> Self {
        DeepLayer2aReversed {
            a: o.a.into(),
            b: o.b.into(),
            c: o.c.into(),
            d: o.d.into(),
            e: o.e.into(),
        }
    }
}

impl From<DeepLayer2b> for DeepLayer2bReversed {
    fn from(o: DeepLayer2b) -> Self {
        DeepLayer2bReversed {
            f: o.f.into(),
            g: o.g.into(),
            h: o.h.into(),
            i: o.i.into(),
            j: o.j.into(),
        }
    }
}

impl From<DeepLayer2c> for DeepLayer2cReversed {
    fn from(o: DeepLayer2c) -> Self {
        DeepLayer2cReversed {
            k: o.k.into(),
            l: o.l.into(),
            m: o.m.into(),
            n: o.n.into(),
            o: o.o.into(),
        }
    }
}

impl From<DeepLayer2d> for DeepLayer2dReversed {
    fn from(o: DeepLayer2d) -> Self {
        DeepLayer2dReversed {
            p: o.p.into(),
            q: o.q.into(),
            r: o.r.into(),
            s: o.s.into(),
            t: o.t.into(),
        }
    }
}

impl From<DeepLayer2e> for DeepLayer2eReversed {
    fn from(o: DeepLayer2e) -> Self {
        DeepLayer2eReversed {
            u: o.u.into(),
            v: o.v.into(),
            w: o.w.into(),
            x: o.x.into(),
            y: o.y.into(),
        }
    }
}

// Layer 1

#[derive(LabelledGeneric)]
struct DeepLayer1a {
    a: DeepLayer2a,
    b: DeepLayer2b,
    c: DeepLayer2c,
    d: DeepLayer2d,
    e: DeepLayer2e,
}

impl DeepLayer1a {
    fn new() -> DeepLayer1a {
        DeepLayer1a {
            a: DeepLayer2a::new(),
            b: DeepLayer2b::new(),
            c: DeepLayer2c::new(),
            d: DeepLayer2d::new(),
            e: DeepLayer2e::new(),
        }
    }
}

// Layer 1 Reversed

#[derive(LabelledGeneric)]
struct DeepLayer1aReversed {
    e: DeepLayer2eReversed,
    d: DeepLayer2dReversed,
    c: DeepLayer2cReversed,
    b: DeepLayer2bReversed,
    a: DeepLayer2aReversed,
}

impl DeepLayer1aReversed {
    fn new() -> DeepLayer1aReversed {
        DeepLayer1aReversed {
            a: DeepLayer2aReversed::new(),
            b: DeepLayer2bReversed::new(),
            c: DeepLayer2cReversed::new(),
            d: DeepLayer2dReversed::new(),
            e: DeepLayer2eReversed::new(),
        }
    }
}

impl From<DeepLayer1a> for DeepLayer1aReversed {
    fn from(o: DeepLayer1a) -> Self {
        DeepLayer1aReversed {
            a: o.a.into(),
            b: o.b.into(),
            c: o.c.into(),
            d: o.d.into(),
            e: o.e.into(),
        }
    }
}

#[bench]
fn manual_deep_from(b: &mut Bencher) {
    b.iter(|| {
        let d_1 = DeepLayer3a::new();
        let reversed: DeepLayer3aReversed = d_1.into();
        reversed
    })
}

#[bench]
fn transmogrify_deep(b: &mut Bencher) {
    b.iter(|| {
        let d_1 = DeepLayer3a::new();
        let reversed: DeepLayer3aReversed = d_1.transmogrify();
        reversed
    })
}
