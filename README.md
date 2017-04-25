# Frunk [![Crates.io](https://img.shields.io/crates/v/frunk.svg)](https://crates.io/crates/frunk) [![Build Status](https://travis-ci.org/lloydmeta/frunk.svg?branch=master)](https://travis-ci.org/lloydmeta/frunk) [![Gitter](https://badges.gitter.im/lloydmeta/frunk.svg)](https://gitter.im/lloydmeta/frunk?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge) [![Frunk](https://docs.rs/frunk/badge.svg)](https://docs.rs/frunk)

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
    * 2.1 [LabelledGeneric](#labelledgeneric)
3. [Coproduct](#coproduct)
4. [Validated](#validated)
5. [Semigroup](#semigroup)
6. [Monoid](#monoid)
7. [Todo](#todo)
8. [Contributing](#contributing)
9. [Inspirations](#inspirations)

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

You can reverse, map, and fold over them too:

```rust
// Reverse
let h1 = hlist![true, "hi"];
assert_eq!(h1.into_reverse(), hlist!["hi", true]);

// Fold (foldl and foldr exist)
let h2 = hlist![1, false, 42f32];
let folded = h2.foldr(
    hlist![
        |i, acc| i + acc,
        |_, acc| if acc > 42f32 { 9000 } else { 0 },
        |f, acc| f + acc
    ],
    1f32
);
assert_eq!(folded, 9001)

// Map
let h3 = hlist![9000, "joe", 41f32];
let mapped = h3.map(hlist![
    |n| n + 1,
    |s| s,
    |f| f + 1f32]);
assert_eq!(mapped, hlist![9001, "joe", 42f32]);
```

You can pluck a type out of an `HList` using `pluck()`, which also gives you back the remainder after plucking that type
out. This method is checked at compile-time to make sure that the type you ask for *can* be extracted.

```rust
let h = hlist![1, "hello", true, 42f32];
let (t, remainder): (bool, _) = h.pluck();
assert!(t);
assert_eq!(remainder, hlist![1, "hello", 42f32])
```

Similarly, you can re-shape, or sculpt, an `Hlist`, there is a `sculpt()` method, which allows you to re-organise and/or
cull the elements by type. Like `pluck()`, `sculpt()` gives you back your target with the remainder data in a pair. This
method is also checked at compile time to make sure that it won't fail at runtime (the types in your requested target shape
must be a subset of the types in the original `HList`.

```rust
let h = hlist![9000, "joe", 41f32, true];
let (reshaped, remainder): (Hlist![f32, i32, &str], _) = h.sculpt();
assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
assert_eq!(remainder, hlist![true]);
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

Sometimes you may have 2 different types that are structurally the same (e.g. different domains but the same data). Use cases include: 

 * You have a models for deserialising from an external API and equivalents for your app logic
 * You want to represent different stages of the same data using types (see [this question on StackOverflow](http://stackoverflow.com/questions/31949455/transform-one-case-class-into-another-when-the-argument-list-is-the-same))

Generic comes with a handy `convert_from` method that helps make this painless:

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

#### LabelledGeneric

In addition to `Generic`, there is also `LabelledGeneric`, which, as the name implies, relies on a generic representation
that is _labelled_. This means that if two structs derive `LabelledGeneric`, you can convert between them only if their
field names match!

Here's an example:
  
```rust
// Suppose that again, we have different User types representing the same data
// in different stages in our application logic.

#[derive(LabelledGeneric)]
struct NewUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(LabelledGeneric)]
struct SavedUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

let n_user = NewUser {
    first_name: "Joe",
    last_name: "Blow",
    age: 30
};

// Convert from a NewUser to a Saved using LabelledGeneric
//
// This will fail if the fields of the types converted to and from do not
// have the same names or do not line up properly :)
//
// Also note that we're using a helper method to avoid having to use universal
// function call syntax
let s_user: SavedUser = labelled_convert_from(n_user);

assert_eq!(s_user.first_name, "Joe");
assert_eq!(s_user.last_name, "Blow");
assert_eq!(s_user.age, 30);

// Uh-oh ! last_name and first_name have been flipped!
#[derive(LabelledGeneric)]
struct DeletedUser<'a> {
    last_name: &'a str,
    first_name: &'a str,
    age: usize,
}

//  This would fail at compile time :)
let d_user = <DeletedUser as LabelledGeneric>::convert_from(s_user);

// This will, however, work, because we make use of the Sculptor type-class
// to type-safely reshape the representations to align/match each other.
let d_user: DeletedUser = transform_from(s_user); 
```

For more information how Generic and Field work, check out their respective Rustdocs:
  * [Generic](https://beachape.com/frunk/frunk_core/generic/index.html)
  * [Labelled](https://beachape.com/frunk/frunk_core/labelled/index.html)
  
### Coproduct

If you've ever wanted to have an adhoc union / sum type of types that you do not control, you may want
to take a look at `Coproduct`. In Rust, thanks to `enum`, you could potentially declare one every time you
want a sum type to do this, but there is a light-weight way of doing it through Frunk:

```rust
#[macro_use] extern crate frunk; // for the Coprod! type macro
use frunk::coproduct::*;

// Declare the types we want in our Coproduct
type I32Bool = Coprod!(i32, f32, bool);

let co1 = I32Bool::inject(3);
let get_from_1a: Option<&i32> = co1.get();
let get_from_1b: Option<&bool> = co1.get();

assert_eq!(get_from_1a, Some(&3));
// None because co1 does not contain a bool, it contains an i32
assert_eq!(get_from_1b, None);

// This will fail at compile time because i8 is not in our Coproduct type
let nope_get_from_1b: Option<&i8> = co1.get(); // <-- will fail
// It's also impossible to inject something into a coproduct that is of the wrong type
// (not contained in the coproduct type)
let nope_co = I32Bool::inject(42f32); // <-- will fail

// We can fold our Coproduct into a single value by handling all types in it
assert_eq!(
    co1.fold(hlist![|i| format!("int {}", i),
                    |f| format!("float {}", f),
                    |b| (if b { "t" } else { "f" }).to_string()]), 
    "int 3".to_string());
```

For more information, check out the [docs for Coproduct](https://beachape.com/frunk/frunk/coproduct/index.html) 

### Validated

`Validated` is a way of running a bunch of operations that can go wrong (for example,
functions returning `Result<T, E>`) and, in the case of one or more things going wrong, 
having all the errors returned to you all at once. In the case that everything went well, you get
an `HList` of all your results. 

Mapping (and otherwise working with plain) `Result`s is different because it will 
stop at the first error, which can be annoying in the very common case (outlined 
best by [the Cats project](http://typelevel.org/cats/datatypes/validated.html)). 

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
