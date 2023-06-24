# Changelog
All notable changes to `frunk` and its subcrates will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]:
- avoid clippy::unneeded_field_pattern (https://github.com/lloydmeta/frunk/pull/216)

## [0.4.2]:
- Removed `proc-macro-hack` machinery (`proc-macros-impl`) (https://github.com/lloydmeta/frunk/pull/214)
- Add function for explicitly extending an hlist (https://github.com/lloydmeta/frunk/pull/209)

## [0.4.1]:
- Documentation fix for `Hcons::sculpt` (https://github.com/lloydmeta/frunk/pull/194)
- Optimise Semigroup for HashSet and HashMap (https://github.com/lloydmeta/frunk/pull/196)
- Update to 2021 edition (https://github.com/lloydmeta/frunk/pull/200)
- Add `extract` to get value out of 1-type coproduct (https://github.com/lloydmeta/frunk/pull/201)
- Fix needless borrow (https://github.com/lloydmeta/frunk/pull/202)
- Add `Coproduct::map` (https://github.com/lloydmeta/frunk/pull/204)

## [0.4.0]:
- [Breaking change] Rename `Hlist!` type macro to `HList!` (https://github.com/lloydmeta/frunk/issues/132)
- [Breaking change] Remove deprecated `HList.length()` (https://github.com/lloydmeta/frunk/issues/125)
- [Breaking change] `HFoldRightable` rework: now `HFoldRightable::foldr` does not differ from `HFoldLeftable::foldl` in **calling**, like `std::iter::DoubleEndedIterator::rfold` does not differ from `std::iter::Iterator::fold`. Note: though `foldr` **behavior** wasn't changed, all old `foldr` calls would either stop compiling or produce wrong results (https://github.com/lloydmeta/frunk/issues/171)
- [Breaking change] Bump quote, syn and proc-macro2 to 1 (https://github.com/lloydmeta/frunk/pull/183)
- Fix unicode identifiers support https://github.com/lloydmeta/frunk/pull/186

## [0.3.2] - 2021-04-16
- Allow folding hlist with a single Poly (https://github.com/lloydmeta/frunk/pull/170)

## [0.3.1] - 2019-12-21
- Refactoring derives (https://github.com/lloydmeta/frunk/pull/157)
- Add support for deriving LabelledGeneric on enums (https://github.com/lloydmeta/frunk/pull/158)
- Added HZippable (https://github.com/lloydmeta/frunk/pull/160)
- Add a type macro for paths (https://github.com/lloydmeta/frunk/pull/161)

## [0.3.0] - 2019-03-23
### Added
- More transmogrifications supported out of the box (https://github.com/lloydmeta/frunk/pull/152)
  - `Box`, `Option`, `Vec` and more.
- More idiomatic Debug impl for Field Debug impls should use DebugStruct #153
- [no-std] support https://github.com/lloydmeta/frunk/pull/148
  - Note: this is a breaking change, see [the PR](https://github.com/lloydmeta/frunk/pull/148) for details

## [0.2.4] - 2019-02-10
### Added
- Added `ToMut` trait, which allows borrowing mutably from a Coproduct or HList.
- Added support for `#[derive(LabelledGeneric)]` on tuple structs
- Added `Path` model and `PathTraverser` trait, which allows for composable lens-like-usage

### Changed
- Make macros call themselves recursively with `$crate::`

## [0.2.3]
- Skipped due to [release mis-steps](https://github.com/lloydmeta/frunk/pull/150#issue-251682325)

## [0.2.2] - 2018-10-21
- Added support for [transmogrifying (recursively sculpting)](https://docs.rs/frunk/0.2.2/frunk/labelled/trait.Transmogrifier.html) one data type into another

## [0.2.1] - 2018-09-29
- Upgraded to `syn` 0.15 and `quote` to 0.6

## [0.2.0] - 2018-04-20
### Added
- :confetti_ball: Forces joined with new collaborators [@Centril] and [@ExpHP]!
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
- *Lots* of documentation improvements!

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

<!-- Misc links -->
[@lloydmeta]: https://github.com/lloydmeta
[@ExpHP]: https://github.com/ExpHP
[@Centril]: https://github.com/Centril

<!-- Here's the list of heading links.  Be sure to update with each release! -->
[Unreleased]: https://github.com/lloydmeta/frunk/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/lloydmeta/frunk/compare/v0.1.36...v0.2.0
