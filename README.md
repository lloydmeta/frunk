# Frust [![Crates.io](https://img.shields.io/crates/v/frust.svg)](https://crates.io/crates/frust) [![Build Status](https://travis-ci.org/lloydmeta/frust.svg?branch=master)](https://travis-ci.org/lloydmeta/frust)

Functional programming in Rust. Still largely a WIP.

General idea is to hopefully, make things easier by allowing stuff like this:

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

## Table of Contents

1. [Semigroup](#semigroup)
2. [Monoid](#monoid)
3. [HList](#hlist)
4. [Validated](#validated)
5. [Todo](#todo)
5. [Contributing](#contributing)
6. [Inspirations](#inspirations)

## Examples

### Semigroup

Things that can be combined.

```rust
use frust::semigroup::*;

assert_eq!(Some(1).combine(&Some(2)), Some(3));

assert_eq!(All(3).combine(&All(5)), All(1)); // bit-wise && 
assert_eq!(All(true).combine(&All(false)), All(false));
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

let h = hlist![true, "hello", Some(41)];
let (h1, tail1) = h.pop();
assert_eq!(h1, true);
assert_eq!(tail1, hlist!["hello", Some(41)]);
```

### Validated

A `Validated` is a way of running a bunch of operations that can go wrong(for example,
returning `Result<T, E>`s) and, in the case of one or more things going wrong, having all the errors
returned to you all at once. In the case that everything went swimmingly, you get
an `HList` of all your results. Mapping `Result`s is different because it will stop
at the first error, which can be annoying in the very common case (outlined best by [the Cats project](http://typelevel.org/cats/tut/validated.html)). 

Here is an example of how it can be used.

```rust
#[derive(PartialEq, Eq, Debug)]
struct Person {
    age: i32,
    name: String,
}

fn get_name() -> Result<String, Error> { /* elided */ }

fn get_age() -> Result<i32, Error> { /* elided */ }

// Build up a `Validated` by adding `Result<T, E>` to it
let validation = get_name().into_validated() + get_age();
// When needed, turn the `Validated` back into a Result and map! 
let try_person = validation.into_result()
                           .map(|hlist| {
                               let (name, (age, _)) = hlist.into_tuple2();
                               Person {
                                   name: name,
                                   age: age,
                               }
                           });

assert_eq!(person,
           Result::Ok(Person {
               name: "James".to_owned(),
               age: 32,
           }));

/// This next pair of functions always return Recover::Err 
fn get_name_faulty() -> Result<String, String> {
    Result::Err("crap name".to_owned())
}

fn get_age_faulty() -> Result<i32, String> {
    Result::Err("crap age".to_owned())
}

let validation2 = get_name_faulty().into_validated() + get_age_faulty();
let try_person2 = validation2.into_result()
                             .map(|hlist| {
                                 let (name, (age, _)) = hlist.into_tuple2();
                                 Person {
                                     name: name,
                                     age: age,
                                 }
                             });

// Notice that we have an accumulated list of errors!
assert_eq!(try_person2,
           Result::Err(vec!["crap name".to_owned(), "crap age".to_owned()]));
    
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

## Contributing

Yes please ! 

The following are considered important, in keeping with the spirit of Rust and functional programming:

- Safety (type and memory)
- Efficiency
- Correctness

## Inspirations

Scalaz, Cats, Haskell, the usual suspects ;)