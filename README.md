# Frust [![Crates.io](https://img.shields.io/crates/v/frust.svg)]() [![Build Status](https://travis-ci.org/lloydmeta/frust.svg?branch=master)](https://travis-ci.org/lloydmeta/frust)

Functional programming in Rust. Still largely a WIP.

## General idea

Hopefully, make things easier by allowing stuff like this:

```rust
use frust::monoid::*;

let v = vec![Some(1), Some(3)];
assert_eq!(combine_all(&v), Some(4));

// Slightly more magical
let t1 = (1, 2.5f32, String::from("hi"), Some(3));
let t2 = (1, 2.5f32, String::from(" world"), None);
let t3 = (1, 2.5f32, String::from(", goodbye"), Some(10));
let tuples = vec![t1, t2, t3];

let expected = (3, 7.5f32, String::from("hi world, goodbye"), Some(133));
assert_eq!(combine_all(&tuples), expected);
```

## Todo

Lots, but for starters, these are not implemented at all, nor do I know for sure if they
are possible given that Rust has no support for Higher Kinded Types.

0. `Functor`
1. `Monad`
2. `Apply`
3. `Applicative`

`Show`, `Monoid`, and `Semigroup` are at least partially (mostly?) implemented.

Benchmarks would be nice but they're an unstable feature, so perhaps in a different branch.

## Inspirations

Scalaz, Cats, Haskell, the usual suspects ;)

## Contributing

Yes please ! 

The following are considered important, in keeping with the spirit of Rust and functional programming:

- Safety (type and memory)
- Efficiency
- Correctness