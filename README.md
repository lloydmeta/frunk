# Frunk [![Crates.io](https://img.shields.io/crates/v/frunk.svg)](https://crates.io/crates/frunk) [![Build Status](https://travis-ci.org/lloydmeta/frunk.svg?branch=master)](https://travis-ci.org/lloydmeta/frunk) [![Join the chat at https://gitter.im/lloydmeta/frunk](https://img.shields.io/badge/gitter-join_chat-1dce73.svg?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4NCjxzdmcgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cmVjdCB4PSIwIiB5PSI1IiBmaWxsPSIjZmZmIiB3aWR0aD0iMSIgaGVpZ2h0PSI1Ii8%2BPHJlY3QgeD0iMiIgeT0iNiIgZmlsbD0iI2ZmZiIgd2lkdGg9IjEiIGhlaWdodD0iNyIvPjxyZWN0IHg9IjQiIHk9IjYiIGZpbGw9IiNmZmYiIHdpZHRoPSIxIiBoZWlnaHQ9IjciLz48cmVjdCB4PSI2IiB5PSI2IiBmaWxsPSIjZmZmIiB3aWR0aD0iMSIgaGVpZ2h0PSI0Ii8%2BPC9zdmc%2B&logoWidth=8)](https://gitter.im/lloydmeta/frunk?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

> **frunk** *frəNGk*
>  * Functional programming toolbelt in Rust.
>  * Might seem funky at first, but you'll like it.
>  * Comes from: funktional (German) + Rust → Frunk

The general idea is to make things easier by providing FP tools in Rust to allow for stuff like this:

```rust
use frunk::monoid::*;

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

For a deep dive, RustDocs are available for:
* Code on [Master](https://beachape.com/frunk)
* Latest [published release](https://docs.rs/frunk)

## Table of Contents

1. [HList](#hlist)
2. [Generic](#generic)
3. [Validated](#validated)
4. [Semigroup](#semigroup)
5. [Monoid](#monoid)
6. [Todo](#todo)
7. [Contributing](#contributing)
8. [Inspirations](#inspirations)

## Examples

### HList

Statically typed heterogeneous lists.

First, let's enable `hlist`:
```rust
#[macro_use] extern crate frunk; // allows us to use the handy hlist! macro
use frunk_core::hlist::*;
```

Some basics:
```rust
let h = hlist![1];
// Type annotations for HList are optional. Here we let the compiler infer it for us
// h has a static type of: HCons<i32, HNil>

// HLists have a head and tail
assert_eq!(hlist![1].head, 1);
assert_eq!(hlist![1].tail, HNil);
```

HLists have a `hlist_pat!` macro for pattern matching;
```rust
let h: Hlist!(&str, &str, i32, bool) = hlist!["Joe", "Blow", 30, true];
// We use the Hlist! type macro to make it easier to write 
// a type signature for HLists, which is a series of nested HCons
// h has an expanded static type of: HCons<&str, HCons<&str, HCons<i32, HCons<bool, HNil>>>>

let hlist_pat!(f_name, l_name, age, is_admin) = h;
assert_eq!(f_name, "Joe");
assert_eq!(l_name, "Blow");
assert_eq!(age, 30);
assert_eq!(is_admin, true);

// You can also use into_tuple2() to turn the hlist into a nested pair
```

You can also traverse HLists using `.pop()`
```rust
let h = hlist![true, "hello", Some(41)];
// h has a static type of: HCons<bool, HCons<&str, HCons<Option<{integer}>, HNil>>>
let (h1, tail1) = h.pop();
assert_eq!(h1, true);
assert_eq!(tail1, hlist!["hello", Some(41)]);
```

### Generic

`Generic` is a way of representing a type in ... a generic way. By coding around `Generic`, you can to write functions 
that abstract over types and arity, but still have the ability to recover your original type afterwards. This can be a fairly powerful thing.

Frunk comes out of the box with a nice custom `Generic` derivation so that boilerplate is kept to a minimum. 

Here are some examples:

#### HList ⇄ Struct

```rust
extern crate frunk;
#[macro_use] // for the hlist macro
extern crate frunk_core;
use frunk::*; // for the Generic trait and HList

#[derive(Generic, Debug, PartialEq)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

let h = hlist!("Joe", "Blow", 30);
let p: Person = from_generic(h);
assert_eq!(p,
           Person {
               first_name: "Joe",
               last_name: "Blow",
               age: 30,
           });
```

This also works the other way too; just pass a struct to `into_generic` and get its generic representation.

#### Converting between Structs 

Sometimes you may have 2 different types that are structurally the same (e.g. different domains but the same data). This can
happen when you have a model for deserialising from an external API and another one for your domain logic.

Generic comes with a handy `convert_from` method that helps:

```rust
// Assume we have all the imports needed
#[derive(Generic)]
struct ApiPerson<'a> {
    FirstName: &'a str,
    LastName: &'a str,
    Age: usize,
}

#[derive(Generic)]
struct DomainPerson<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

let a_person = ApiPersion {
                   first_name: "Joe",
                   last_name: "Blow",
                   age: 30,
};
let d_person: DomainPersion = convert_from(a_person); // done
```


### Validated

`Validated` is a way of running a bunch of operations that can go wrong (for example,
functions returning `Result<T, E>`) and, in the case of one or more things going wrong, 
having all the errors returned to you all at once. In the case that everything went well, you get
an `HList` of all your results. 

Mapping (and otherwise working with plain) `Result`s is different because it will 
stop at the first error, which can be annoying in the very common case (outlined 
best by [the Cats project](http://typelevel.org/cats/tut/validated.html)). 

To use `Validated`, first:
```rust
#[macro_use] extern crate frunk; // allows us to use the handy hlist! macro
use frunk_core::hlist::*;
use frunk::validated::*;
```

Assuming we have a `Person` struct defined
```rust
#[derive(PartialEq, Eq, Debug)]
struct Person {
    age: i32,
    name: String,
    street: String,
}
```

Here is an example of how it can be used in the case that everything goes smoothly.

```rust
fn get_name() -> Result<String, Error> { /* elided */ }
fn get_age() -> Result<i32, Error> { /* elided */ }
fn get_street() -> Result<String, Error> { /* elided */ }

// Build up a `Validated` by adding in any number of `Result`s
let validation = get_name().into_validated() + get_age() + get_street();
// When needed, turn the `Validated` back into a Result and map as usual
let try_person = validation.into_result()
                           // Destructure our hlist
                           .map(|hlist_pat!(name, age, street)| { 
                               Person {
                                   name: name,
                                   age: age,
                                   street: street,
                               }
                           });

assert_eq!(try_person.unwrap(),
           Person {
               name: "James".to_owned(),
               age: 32,
               street: "Main".to_owned(),
           }));
}
```

If, on the other hand, our `Result`s are faulty:
```rust
/// This next pair of functions always return Recover::Err 
fn get_name_faulty() -> Result<String, String> {
    Result::Err("crap name".to_owned())
}

fn get_age_faulty() -> Result<i32, String> {
    Result::Err("crap age".to_owned())
}

let validation2 = get_name_faulty().into_validated() + get_age_faulty();
let try_person2 = validation2.into_result()
                             .map(|_| unimplemented!());

// Notice that we have an accumulated list of errors!
assert_eq!(try_person2.unwrap_err(),
           vec!["crap name".to_owned(), "crap age".to_owned()]); 
```

### Semigroup

Things that can be combined.

```rust
use frunk::semigroup::*;

assert_eq!(Some(1).combine(&Some(2)), Some(3));

assert_eq!(All(3).combine(&All(5)), All(1)); // bit-wise && 
assert_eq!(All(true).combine(&All(false)), All(false));
```

### Monoid

Things that can be combined *and* have an empty/id value.

```rust
use frunk::monoid::*;

let t1 = (1, 2.5f32, String::from("hi"), Some(3));
let t2 = (1, 2.5f32, String::from(" world"), None);
let t3 = (1, 2.5f32, String::from(", goodbye"), Some(10));
let tuples = vec![t1, t2, t3];

let expected = (3, 7.5f32, String::from("hi world, goodbye"), Some(13));
assert_eq!(combine_all(&tuples), expected)

let product_nums = vec![Product(2), Product(3), Product(4)];
assert_eq!(combine_all(&product_nums), Product(24))
```

## Todo

### Stabilise interface, general cleanup

Before a 1.0 release, would be best to revisit the design of the interfaces
and do some general code (and test cleanup).

### Benchmarks

Benchmarks are available in `./benches` and can be run with:

`$ rustup run nightly cargo bench`

It would be nice to use something like [bench-cmp](https://github.com/BurntSushi/cargo-benchcmp) to compare
before and after, but for some reason, there is no output. Should investigate why.

### Not yet implemented 

Given that Rust has no support for Higher Kinded Types, I'm not sure if these
are even possible to implement. In addition, Rustaceans are used to calling `iter()` 
on collections to get a lazy view, manipulating their elements with `map`
or `and_then`, and then doing a `collect()` at the end to keep things
efficient. The usefulness of these following structures maybe limited in that context.

0. `Functor`
1. `Monad`
2. `Apply`
3. `Applicative`

## Contributing

Yes please ! 

The following are considered important, in keeping with the spirit of Rust and functional programming:

- Safety (type and memory)
- Efficiency
- Correctness

## Inspirations

Scalaz, Shapeless, Cats, Haskell, the usual suspects ;)
