# Changelog
All notable changes to `frunk` and its subcrates will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]:
### Added
- This CHANGELOG file. :wink:

## [0.2.0] - 2018-04-20
### Added
- `frunk` now adheres to semantic versioning!
- Inherent method wrappers for many trait methods on HList and Coproduct.
- Re-exports for a variety of essential items at the root level of `frunk`.
- Module `frunk::prelude` for extension traits and similar.
- Methods `Coproduct::embed` and `Coproduct::subset`.
- `...tail` argument in each of the HList and Coproduct macros.
- The `Func` trait, an alternative to `Fn` that can be implemented on stable.
- The `Poly` wrapper type and `poly_fn!` macro, which use `Func` to provide
  order-free mapping.
- `map_repr` and `map_intern` for `Generic`, convenience methods for
  temporarily converting an object into its generic representation or
  a similar type.

### Changed
- A general paradigm shift from per-module glob imports to the more
  Rusty model of *`use` what you need.*  Many of the other changes listed
  help accomodate this.
- Renamed the `with-serde` feature to `serde`, in line with convention.
- Various unnecessary type parameters have been removed or replaced with
  associated types.  This affects `CoprodInjector`, `HMappable`,
  and `HFold{Left,Right}able`.
- Moved `hlist::IntoReverse` to accompany `Func` in the new `traits` module.
- Moved `hlist::{Here, There, Suffixed}` to a new `indices` module.
- The zero index `Here` is no longer an empty type.  We have... *plans.*
  :japanese_ogre:
- Character types were moved from `labelled` to `labelled::chars`.
- Mapping/folding by reference is now written as `list.to_ref().map(...)`,
  where formerly `as_ref()` was used. The new `to_ref()` trait composes
  orthogonally to many other features of HList and Coproduct, and relieves
  frunk from providing impls for `&HCons` that can be susceptible to the
  dreaded "Overflow evaluating `_: std::marker::Sized`" error.

### Removed
- The identity `AsRef` impls on HList and Coproduct, which were only
  present to support the old `as_ref().map(...)` pattern.

## 0.1.36 - 2018-02-25

### Features present
Prior to v0.2.0, `frunk` did not keep a detailed changelog, and did not adhere to semver.
The following list is a modest attempt to summarize the features of v0.1.36.

- The types `HList`s and `Coproduct`s, which serve as variadic product and sum types.
  - Many methods are provided through individual traits.
- The SYB traits `Generic` and `LabelledGeneric` for conversions between similarly-shaped structs.
  - The `frunk_derives` crate provides `#[derive(Generic, LabelledGeneric)]`.
- The functional traits `Semigroup` and `Monoid`, with a number of implementations.
  - A number of implementations are provided.
  - The `frunk_laws` crate provides `quickcheck`-compatible predicates for testing custom impls.
- The convenience macros `hlist!`, `Hlist!`, `hlist_pat!`, `field!`.
- The `Validated` type, a non-short-circuiting alternative to `Result`.

## Older versions

A raw overview of older versions is available in the form of commit logs:


* [0.1.36](https://github.com/lloydmeta/frunk/compare/v0.1.35...v0.1.36) - 2018-02-25
* [0.1.35](https://github.com/lloydmeta/frunk/compare/v0.1.34...v0.1.35) - 2018-02-11
* [0.1.34](https://github.com/lloydmeta/frunk/compare/v0.1.33...v0.1.34) - 2017-10-25
* [0.1.33](https://github.com/lloydmeta/frunk/compare/v0.1.32...v0.1.33) - 2017-09-27
* [0.1.32](https://github.com/lloydmeta/frunk/compare/v0.1.31...v0.1.32) - 2017-09-09
* [0.1.31](https://github.com/lloydmeta/frunk/compare/v0.1.30...v0.1.31) - 2017-09-04
* [0.1.30](https://github.com/lloydmeta/frunk/compare/v0.1.29...v0.1.30) - 2017-06-21
* [0.1.29](https://github.com/lloydmeta/frunk/compare/v0.1.28...v0.1.29) - 2017-06-03
* [0.1.28](https://github.com/lloydmeta/frunk/compare/v0.1.27...v0.1.28) - 2017-04-25
* [0.1.27](https://github.com/lloydmeta/frunk/compare/v0.1.25...v0.1.27) - 2017-04-23
* [0.1.25](https://github.com/lloydmeta/frunk/compare/v0.1.22...v0.1.25) - 2017-04-20
* [0.1.22](https://github.com/lloydmeta/frunk/compare/v0.1.20...v0.1.22) - 2017-03-22
* [0.1.20](https://github.com/lloydmeta/frunk/compare/v0.1.19...v0.1.20) - 2017-03-17
* [0.1.19](https://github.com/lloydmeta/frunk/compare/v0.1.18...v0.1.19) - 2017-03-16
* [0.1.18](https://github.com/lloydmeta/frunk/compare/v0.1.17...v0.1.18) - 2017-03-06
* [0.1.17](https://github.com/lloydmeta/frunk/compare/v0.1.16...v0.1.17) - 2017-03-04
* [0.1.16](https://github.com/lloydmeta/frunk/compare/v0.1.11...v0.1.16) - 2017-03-04
* [0.1.11](https://github.com/lloydmeta/frunk/compare/v0.1.10...v0.1.11) - 2017-03-01
* [0.1.10](https://github.com/lloydmeta/frunk/compare/v0.1.9...v0.1.10) - 2017-03-01

<!-- Here's the list of heading links.  Be sure to update with each release! -->
[Unreleased]: https://github.com/lloydmeta/frunk/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/lloydmeta/frunk/compare/v0.1.36...v0.2.0
