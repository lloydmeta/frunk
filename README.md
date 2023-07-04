# Frunk [![Crates.io](https://img.shields.io/crates/v/frunk.svg)](https://crates.io/crates/frunk) [![Continuous integration](https://github.com/lloydmeta/frunk/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/lloydmeta/frunk/actions/workflows/ci.yml?query=branch%3Amaster) [![Gitter](https://badges.gitter.im/lloydmeta/frunk.svg)](https://gitter.im/lloydmeta/frunk?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge) [![Frunk](https://docs.rs/frunk/badge.svg)](https://docs.rs/frunk)

> **frunk** *frəNGk*
>  * Functional programming toolbelt in Rust.
>  * Might seem funky at first, but you'll like it.
>  * Comes from: funktional (German) + Rust → Frunk

The general idea is to make things easier by providing FP tools in Rust to allow for stuff like this:

```rust
use frunk::monoid::combine_all;

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
       * 2.1.2 [Path (Lenses)](#path)
3. [Coproduct](#coproduct)
4. [Validated](#validated)
5. [Semigroup](#semigroup)
6. [Monoid](#monoid)
7. [Features](#features)
8. [Benchmarks](#benchmarks)
9. [Todo](#todo)
10. [Contributing](#contributing)
11. [Inspirations](#inspirations)
12. [Maintainers](#maintainers)

## Examples

### HList

Statically typed heterogeneous lists.

First, let's enable `hlist`:
```rust
use frunk::{HNil, HCons, hlist};
```

Some basics:
```rust
let h = hlist![1];
// Type annotations for HList are optional. Here we let the compiler infer it for us
// h has a static type of: HCons<i32, HNil>

// HLists have a head and tail
assert_eq!(hlist![1].head, 1);
assert_eq!(hlist![1].tail, HNil);

// You can convert a tuple to an HList and vice-versa
let h2 = hlist![ 42f32, true, "hello" ];
let t: (f32, bool, &str) = h2.into();
assert_eq!(t, (42f32, true, "hello"));

let t3 = (999, false, "world");
let h3: HList![ isize, bool, &str ] = t3.into();
assert_eq!(h3, hlist![ 999, false, "world" ]);
```

HLists have a `hlist_pat!` macro for pattern matching;
```rust
let h: HList!(&str, &str, i32, bool) = hlist!["Joe", "Blow", 30, true];
// We use the HList! type macro to make it easier to write
// a type signature for HLists, which is a series of nested HCons
// h has an expanded static type of: HCons<&str, HCons<&str, HCons<i32, HCons<bool, HNil>>>>

let hlist_pat!(f_name, l_name, age, is_admin) = h;
assert_eq!(f_name, "Joe");
assert_eq!(l_name, "Blow");
assert_eq!(age, 30);
assert_eq!(is_admin, true);

// You can also use into_tuple2() to turn the hlist into a nested pair
```

To traverse or build lists, you can also prepend/or pop elements at the front:
```rust
let list = hlist![true, "hello", Some(41)];
// h has a static type of: HCons<bool, HCons<&str, HCons<Option<{integer}>, HNil>>>
let (head1, tail1) = list.pop();
assert_eq!(head1, true);
assert_eq!(tail1, hlist!["hello", Some(41)]);
let list1 = tail1.prepend(head1);
assert_eq!(list, list1);

// or using macro sugar:
let hlist_pat![head2, ...tail2] = list; // equivalent to pop
let list2 = hlist![head2, ...tail2];    // equivalent to prepend
assert_eq!(list, list2);
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
        |acc, i| i + acc,
        |acc, _| if acc > 42f32 { 9000 } else { 0 },
        |acc, f| f + acc
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
let (reshaped, remainder): (HList![f32, i32, &str], _) = h.sculpt();
assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
assert_eq!(remainder, hlist![true]);
```

### Generic

`Generic` is a way of representing a type in ... a generic way. By coding around `Generic`, you can to write functions
that abstract over types and arity, but still have the ability to recover your original type afterwards. This can be a fairly powerful thing.

#### Setup

In order to derive the trait `Generic` (or `LabelledGeneric`) you will have to add `frunk_core` dependency

```toml
[dependencies]
frunk_core = { version = "$version" }
```

Frunk comes out of the box with a nice custom `Generic` derivation so that boilerplate is kept to a minimum.

Here are some examples:

#### HList ⇄ Struct

```rust
#[derive(Generic, Debug, PartialEq)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

let h = hlist!("Joe", "Blow", 30);
let p: Person = frunk::from_generic(h);
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

let a_person = ApiPerson {
                   FirstName: "Joe",
                   LastName: "Blow",
                   Age: 30,
};
let d_person: DomainPerson = frunk::convert_from(a_person); // done
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
let s_user: SavedUser = frunk::labelled_convert_from(n_user);

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
let d_user: DeletedUser = frunk::labelled_convert_from(s_user);

// This will, however, work, because we make use of the Sculptor type-class
// to type-safely reshape the representations to align/match each other.
let d_user: DeletedUser = frunk::transform_from(s_user);
```

##### Transmogrifying

Sometimes you need might have one data type that is "similar in shape" to another data type, but it
is similar _recursively_ (e.g. it has fields that are structs that have fields that are a superset of
the fields in the target type, so they are transformable recursively).  `.transform_from` can't help you
there because it doesn't deal with recursion, but the `Transmogrifier` can help if both are `LabelledGeneric`
by `transmogrify()`ing from one to the other.

What is "transmogrifying"? In this context, it means to recursively tranform some data of type A into data
of type B, in a typesafe way, as long as A and B are "similarly-shaped".  In other words, as long as B's
fields and their subfields are subsets of A's fields and their respective subfields, then A can be turned
into B.

As usual, the goal with Frunk is to do this:
* Using stable (so no specialisation, which would have been helpful, methinks)
* Typesafe
* No usage of `unsafe`

Here is an example:

```rust
use frunk::labelled::Transmogrifier;

#[derive(LabelledGeneric)]
struct InternalPhoneNumber {
    emergency: Option<usize>,
    main: usize,
    secondary: Option<usize>,
}

#[derive(LabelledGeneric)]
struct InternalAddress<'a> {
    is_whitelisted: bool,
    name: &'a str,
    phone: InternalPhoneNumber,
}

#[derive(LabelledGeneric)]
struct InternalUser<'a> {
    name: &'a str,
    age: usize,
    address: InternalAddress<'a>,
    is_banned: bool,
}

#[derive(LabelledGeneric, PartialEq, Debug)]
struct ExternalPhoneNumber {
    main: usize,
}

#[derive(LabelledGeneric, PartialEq, Debug)]
struct ExternalAddress<'a> {
    name: &'a str,
    phone: ExternalPhoneNumber,
}

#[derive(LabelledGeneric, PartialEq, Debug)]
struct ExternalUser<'a> {
    age: usize,
    address: ExternalAddress<'a>,
    name: &'a str,
}

let internal_user = InternalUser {
    name: "John",
    age: 10,
    address: InternalAddress {
        is_whitelisted: true,
        name: "somewhere out there",
        phone: InternalPhoneNumber {
            main: 1234,
            secondary: None,
            emergency: Some(5678),
        },
    },
    is_banned: true,
};

/// Boilerplate-free conversion of a top-level InternalUser into an
/// ExternalUser, taking care of subfield conversions as well.
let external_user: ExternalUser = internal_user.transmogrify();

let expected_external_user = ExternalUser {
    name: "John",
    age: 10,
    address: ExternalAddress {
        name: "somewhere out there",
        phone: ExternalPhoneNumber {
            main: 1234,
        },
    }
};

assert_eq!(external_user, expected_external_user);
```

Note that as of writing, there are a couple of known limitations with `transmogrify()`,
some of which may be addressed in the future:

* If one of the fields is an identical type **and** derives `LabelledGeneric`,
  the compiler will tell you that it can't "infer an index" for `transmogrify()`; this
  is because `impl`s of the `Transmogrifier` trait will clash. This may or may not
  change in the future (perhaps if we move to a pure procedural macro powered way of doing
  things?)
* For types that contain many multiple deeply-nested fields that require `transmogfiy()`ing,
  using this technique will likely increase your compile time.
* If you've balked at the the compile-time errors with `transform_from` when a transform is deemed
  impossible (e.g. missing field), the errors for `transmogrify()` are worse to the degree that
  recursive `transmogrify()` is required for your types.

For more information how Generic and Field work, check out their respective Rustdocs:
  * [Generic](https://beachape.com/frunk/frunk_core/generic/index.html)
  * [Labelled](https://beachape.com/frunk/frunk_core/labelled/index.html)

#### Path

One of the other things that `LabelledGeneric`-deriving structs can do is be generically traversed
using `Path` and its companion trait `PathTraverser`. In some circles, this functionality is also
called a Lens.

`Path`-based traversals are
* Easy to use through the procedural macro `path!` (`frunk_proc_macros`)
  * Traversing multiple levels is familiar; just use dot `.` syntax (`path!(nested.attribute.value)`)
* Compile-time safe
* Composable (add one to the other using `+`)
* Allows you to get by value, get by reference or get by mutable reference, depending on the type
  of thing you pass it.

```rust
#[derive(LabelledGeneric)]
struct Dog<'a> {
    name: &'a str,
    dimensions: Dimensions,
}

#[derive(LabelledGeneric)]
struct Cat<'a> {
    name: &'a str,
    dimensions: Dimensions,
}

#[derive(LabelledGeneric)]
struct Dimensions {
    height: usize,
    width: usize,
    unit: SizeUnit,
}

#[derive(Debug, Eq, PartialEq)]
enum SizeUnit {
    Cm,
    Inch,
}

let mut dog = Dog {
    name: "Joe",
    dimensions: Dimensions {
        height: 10,
        width: 5,
        unit: SizeUnit::Inch,
    },
};

let cat = Cat {
    name: "Schmoe",
    dimensions: Dimensions {
        height: 7,
        width: 3,
        unit: SizeUnit::Cm,
    },
};

// generic, re-usable, compsable paths
let dimensions_lens = path!(dimensions);
let height_lens = dimensions_lens + path!(height); // compose multiple
let unit_lens = path!(dimensions.unit); // dot syntax to just do the whole thing at once

assert_eq!(*height_lens.get(&dog), 10);
assert_eq!(*height_lens.get(&cat), 7);
assert_eq!(*unit_lens.get(&dog), SizeUnit::Inch);
assert_eq!(*unit_lens.get(&cat), SizeUnit::Cm);

// modify by passing a &mut
*height_lens.get(&mut dog) = 13;
assert_eq!(*height_lens.get(&dog), 13);
```

There's also a `Path!` type-level macro for declaring shape-constraints. This allows you to write adhoc shape-dependent
functions for `LabelledGeneric` types.

```rust
// Prints height as long as `A` has the right "shape" (e.g.
// has `dimensions.height: usize` and `dimension.unit: SizeUnit)
fn print_height<'a, A, HeightIdx, UnitIdx>(obj: &'a A) -> ()
where
    &'a A: PathTraverser<Path!(dimensions.height), HeightIdx, TargetValue = &'a usize>
        + PathTraverser<Path!(dimensions.unit), UnitIdx, TargetValue = &'a SizeUnit>,
{
    println!(
        "Height [{} {:?}]",
        path!(dimensions.height).get(obj),
        path!(dimensions.unit).get(obj)
    );
}
```

See `examples/paths.rs` to see how this works.

### Coproduct

If you've ever wanted to have an adhoc union / sum type of types that you do not control, you may want
to take a look at `Coproduct`. In Rust, thanks to `enum`, you could potentially declare one every time you
want a sum type to do this, but there is a light-weight way of doing it through Frunk:

```rust
use frunk::prelude::*; // for the fold method

// Declare the types we want in our Coproduct
type I32F32Bool = Coprod!(i32, f32, bool);

let co1 = I32F32Bool::inject(3);
let get_from_1a: Option<&i32> = co1.get();
let get_from_1b: Option<&bool> = co1.get();

assert_eq!(get_from_1a, Some(&3));
// None because co1 does not contain a bool, it contains an i32
assert_eq!(get_from_1b, None);

// This will fail at compile time because i8 is not in our Coproduct type
let nope_get_from_1b: Option<&i8> = co1.get(); // <-- will fail
// It's also impossible to inject something into a coproduct that is of the wrong type
// (not contained in the coproduct type)
let nope_co = I32F32Bool::inject(42f64); // <-- will fail

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
use frunk::prelude::*; // for Result::into_validated
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
use frunk::Semigroup;
use frunk::semigroup::All;

assert_eq!(Some(1).combine(&Some(2)), Some(3));

assert_eq!(All(3).combine(&All(5)), All(1)); // bit-wise &&
assert_eq!(All(true).combine(&All(false)), All(false));
```

### Monoid

Things that can be combined *and* have an empty/id value.

```rust
use frunk::monoid::combine_all;

let t1 = (1, 2.5f32, String::from("hi"), Some(3));
let t2 = (1, 2.5f32, String::from(" world"), None);
let t3 = (1, 2.5f32, String::from(", goodbye"), Some(10));
let tuples = vec![t1, t2, t3];

let expected = (3, 7.5f32, String::from("hi world, goodbye"), Some(13));
assert_eq!(combine_all(&tuples), expected)

let product_nums = vec![Product(2), Product(3), Product(4)];
assert_eq!(combine_all(&product_nums), Product(24))
```

### Features

Frunk comes with support for deriving [serde](https://github.com/serde-rs/serde) serializer/deserializers for its core
data structures. This can be enabled by adding the `serde` feature flag.

For example, if you'd like to use just `frunk_core` with serde

```toml
[dependencies]
frunk_core = { version = "$version", features = ["serde"] }
```

Or, if you'd like to use `frunk` with serde, you need to explicitly include `frunk_core` as well

```toml
[dependencies]
frunk = { version = "$version", features = ["serde"] }
frunk_core = { version = "$version", features = ["serde"] }
```

### Benchmarks

Benchmarks are available in `./benches` and can be run with:

`$ rustup run nightly cargo bench`

Benchmarks on `master` are also [auto-generated, uploaded and available online](https://beachape.com/frunk/dev/bench).

## Todo

### Stabilise interface, general cleanup

Before a 1.0 release, would be best to revisit the design of the interfaces
and do some general code (and test cleanup).

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

## Maintainers

A.k.a. people whom you can bug/tag/@ on Gitter :D

1. [lloydmeta](https://github.com/lloydmeta)
2. [Centril](https://github.com/centril)
3. [ExpHP](https://github.com/ExpHP)
