# Frust [![Crates.io](https://img.shields.io/crates/v/frust.svg)]() [![Build Status](https://travis-ci.org/lloydmeta/frust.svg?branch=master)](https://travis-ci.org/lloydmeta/frust)

Functional programming in Rust. Still largely a WIP.

## General idea

Hopefully, make things easier by allowing stuff like this:

```rust
use frust::monoid::*;

let v = vec![Some(1), Some(3)];
assert_eq!(combine_all(&v), Some(4));

// Slightly more magical
let t1 =       (1, 2.5f32,                String::from("hi"),  Some(3));
let t2 =       (1, 2.5f32,            String::from(" world"),     None);
let t3 =       (1, 2.5f32,         String::from(", goodbye"), Some(10));
let tuples = vec![t1, t2, t3];

let expected = (3, 7.5f32, String::from("hi world, goodbye"), Some(13));
assert_eq!(combine_all(&tuples), expected);
```

## Examples

### Semigroup

Things that can be combined.

```rust
use frust::semigroup::*;

assert_eq!(Some(1).combine(&Some(2)), Some(3));

assert_eq!(All(3).combine(&All(5)), All(1)); // bit-wise && 
assert_eq!(All(true).combine(&All(false)), All(false));

let vec_of_some_strings = vec![Some(String::from("Hello")), Some(String::from(" World"))];
assert_eq!(combine_all(&vec_of_some_strings), Some(String::from("Hello World")));
```

### Monoid

Things that can be combined *and* have an empty/id value.

```rust
use frust::monoid::*;

let t1 = (1, 2.5f32, String::from("hi"), Some(3));
let t2 = (1, 2.5f32, String::from(" world"), None);
let t3 = (1, 2.5f32, String::from(", goodbye"), Some(10));
let tuples = vec![t1, t2, t3];

let expected = (3, 7.5f32, String::from("hi world, goodbye"), Some(13));
assert_eq!(combine_all(&tuples), expected)

let product_nums = vec![Product(2), Product(3), Product(4)];
assert_eq!(combine_all(&product_nums), Product(24))
```

### HList

Statically typed heterogeneous lists. Pop as much as you want from one of these; everything
remains typed.

```rust
#[macro_use] extern crate frust;
use frust::hlist::*;

let h = hlist![1, 2, 3];
let (h1, tail1) = h.pop();
assert_eq!(h1, 1);
assert_eq!(tail1, hlist![2, 3]);
```

## Todo

It makes sense to start by implementing things that are useful even for idiomatic
Rust usage (efficient, and safe). The following might be nice to have:
  
1. Validation (See cats)

These are not implemented at all, nor do I know for sure if they
are possible given that Rust has no support for Higher Kinded Types. In addition,
Rustaceans are used to calling `iter()` on collections to get a lazy view, 
manipulating their lists, and then doing a `collect()` at the end to keep things efficient.
The use of these following structures maybe limited in that context.

0. `Functor`
1. `Monad`
2. `Apply`
3. `Applicative`

`Show`, `Monoid`, `HList`, and `Semigroup` are at least partially (mostly?) implemented.

Benchmarks would be nice but they're an unstable feature, so perhaps in a different branch.

## Inspirations

Scalaz, Cats, Haskell, the usual suspects ;)

## Contributing

Yes please ! 

The following are considered important, in keeping with the spirit of Rust and functional programming:

- Safety (type and memory)
- Efficiency
- Correctness