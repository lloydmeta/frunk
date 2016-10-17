# Frust [![Crates.io](https://img.shields.io/crates/v/frust.svg)]() [![Build Status](https://travis-ci.org/lloydmeta/frust.svg?branch=master)](https://travis-ci.org/lloydmeta/frust)

Functional programming in Rust. Mostly WIP for the purpose of learning stuff.

## General idea

Hopefully, make things easier by allowing stuff like this:

```rust
use frust::monoid::*;

let v = vec![Some(1), Some(3)];
assert_eq!(combine_all(&v), Some(4));


// the following will not compile because Display is not implemented
// print!("{}", Some(Box::new(1)));

use frust::show::*;
print!("{}", Some(Box::new(1)).show());
assert_eq!(Some(Box::new(1)).show(), "Some(Box(1))");

// Show is implemented for Tuples too
 
// print!("{}", (1,2, 3f32)); <-- won't compile
print!("{}", (1,2, 3f32).show()); // <-- will compile
```

## Todo

Lots, but for starters, these are not implemented at all, nor do I know for sure if they
are possible given that Rust has no support for Higher Kinded Types.

0. `Functor`
1. `Monad`
2. `Apply`
3. `Applicative`

`Show`, `Monoid`, and `Semigroup` are at least partially implemented.

## Inspirations

Scalaz, Cats, Haskell, the usual suspects ;)
