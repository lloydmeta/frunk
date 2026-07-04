# AGENT.md

Agentic working instructions for the **frunk** repository.  Read this before making changes.  It captures the architecture, the type-level techniques that make the library work, the invariants that must not be broken, and the exact commands CI enforces.

## What frunk is

Frunk is a functional programming toolbelt for Rust: `HList`, `Coproduct`, `Generic`, `LabelledGeneric`, `Path` (lenses), `Validated`, `Semigroup` and `Monoid`.  The library's distinguishing feature is that most of it is implemented with **type-level programming** on stable Rust: no `unsafe`, no specialisation, no nightly-only features in the shipped code.  Correctness, type/memory safety and efficiency are the stated priorities, in that spirit.

Reference reading (the author's own write-ups, useful for understanding intent, not just mechanics):

* HLists: <https://beachape.com/blog/2016/10/23/rust-hlists-heterogenously-typed-list/>
* Generic: <https://beachape.com/blog/2017/02/04/rust-generic-not-generics/>
* LabelledGeneric: <https://beachape.com/blog/2017/03/04/labelledgeneric-in-rust-what-why-how/>
* Type-level recursion and sculpting: <https://beachape.com/blog/2017/03/12/gentle-intro-to-type-level-recursion-in-Rust-from-zero-to-frunk-hlist-sculpting/>
* Boilerplate-free struct transforms (transmogrify): <https://beachape.com/blog/2017/04/12/boilerplate-free-struct-transforms-in-rust/>
* Structural typing (paths): <https://beachape.com/blog/2021/05/25/structural-typing-in-rust/>

## Workspace layout

Frunk is a Cargo workspace.  Know which crate a change belongs in before editing.

* **`frunk`** (root, `src/`): the user-facing facade.  Re-exports `frunk_core` and `frunk_derives`, and adds `monoid`, `semigroup`, `validated`.  `#![no_std]` with an `alloc` feature.
* **`frunk_core`** (`core/`): the fundamental building blocks, intentionally minimal.  Modules: `hlist`, `coproduct`, `generic`, `labelled`, `path`, `indices`, `traits`, `tuples`, `macros`.  `#![no_std]` (+ `alloc`).
* **`frunk_derives`** (`derives/`): the `#[derive(Generic)]` and `#[derive(LabelledGeneric)]` custom derives.  `proc-macro = true`.
* **`frunk_proc_macros`** (`proc-macros/`): the `path!` and `Path!` function-like proc macros.
* **`frunk_proc_macro_helpers`** (`proc-macro-helpers/`): shared internals for the two proc-macro crates (type-level label encoding, HList/Coproduct AST builders, field-binding helpers).  Not published for direct use.
* **`frunk_laws`** (`laws/`): `quickcheck`-based property tests for the algebraic laws (`Semigroup`, `Monoid`).

Version note: `frunk`, `frunk_core`, `frunk_derives` share a version; `frunk_proc_macros`/`frunk_proc_macro_helpers` and `frunk_laws` are versioned independently.  Path dependencies also carry a `version` field, so a version bump in one crate must be propagated to its dependents.

## Build, test, lint

These are the exact checks CI (`.github/workflows/ci.yml`) runs on stable and nightly.  Run them locally before proposing a change; a green local run of these is the bar.

```sh
# formatting (must be clean)
cargo fmt --all -- --check

# clippy, warnings are errors, including test code
cargo clippy --all --tests -- -D warnings

# type-check and build (std)
cargo check
cargo build
cargo build --examples

# full test suite across the workspace (includes doctests)
cargo test --all --no-fail-fast

# docs must build clean, private items included
RUSTDOCFLAGS='-D warnings' cargo doc --all --no-deps --document-private-items
```

`no_std` is a first-class target and is checked in CI against an embedded target with default features off:

```sh
rustup target add thumbv6m-none-eabi
cargo check --target thumbv6m-none-eabi --no-default-features
cargo build --target thumbv6m-none-eabi --no-default-features
```

Benchmarks live in `benches/` and run with `cargo bench` (nightly-flavoured harness in CI).

## Core mental model

Internalise these five ideas; almost every file is an application of them.

### 1. HList = `HCons` / `HNil`

An `HList` is a heterogeneous, compile-time-typed list, structurally just an arbitrarily nested pair: `HCons<H, T>` (head + tail) terminated by `HNil`.  `Hlist![A, B, C]` is sugar for `HCons<A, HCons<B, HCons<C, HNil>>>`.  Construct with `hlist![a, b, c]`, pattern-match with `hlist_pat![a, b, c]`, and write the type with the `HList![..]` type macro.  All the interesting operations (`map`, `foldl`/`foldr`, `zip`, `sculpt`, `pluck`, `get`, `into_reverse`, `to_ref`/`to_mut`) are traits with a base case for `HNil` and a recursive case for `HCons`.

### 2. Type-level recursion driven by trait resolution, disambiguated by phantom index types

This is the technique that makes the whole library possible on stable Rust.  Traits like `Selector`, `Plucker`, `Sculptor`, `ByNameFieldPlucker`, and `Transmogrifier` carry an extra `Index` type parameter that the user never writes; it is solved by type inference.  The indices live in `core/src/indices.rs`: `Here` (target is the head), `There<T>` (target is `1 + T` deeper in the tail), plus `Suffixed`, `IdentityTransMog`, `DoTransmog`, and several `*IndicesWrapper` types for transmogrify.

The reason the index exists: without it, the "found it in the head" impl and the "recurse into the tail" impl would overlap, and stable Rust would reject them.  Encoding the position into the type signature (`Plucker<T, Here>` vs `Plucker<T, There<TailIndex>>`) makes the impls disjoint.  When you write a new recursive HList/Coproduct trait, follow this exact pattern: one impl for the head keyed on `Here`, one for the tail keyed on `There<TailIndex>` with a `where Tail: TheTrait<T, TailIndex>` bound.

### 3. Generic = an isomorphism between a struct and an HList

`#[derive(Generic)]` gives a type a `Repr` (an unlabelled HList of its field types) with `into`/`from`.  The two laws (documented on the trait) are `from(into(x)) == x` and `into(from(y)) == y`.  Because `Repr` is purely positional, two structs with the same field types in the same order share a `Repr`, so `convert_from` moves between them.  This is powerful but positional: it will happily convert between structs whose fields line up by type even if the meaning differs.

### 4. LabelledGeneric = Generic that also carries field names at the type level

`#[derive(LabelledGeneric)]` produces a `Repr` of `Field<Name, Type>` where `Name` is a **type-level string** built from zero-sized char enums in `frunk_core::labelled::chars` (e.g. `first_name` becomes the tuple type `(f, i, r, s, t, __, n, a, m, e)`).  This makes conversions name-aware:

* `labelled_convert_from` requires identical labelled `Repr`s (same names, same order): safer than `Generic::convert_from`.
* `transform_from` uses `Sculptor` to reorder and subset fields by name (drops the remainder).
* `Transmogrifier::transmogrify` recurses: it converts "similarly shaped" nested structures where the target's fields (and their subfields) are a subset of the source's, mapping through `Option`, `Box`, `Vec`, `VecDeque`, `LinkedList` on the way.

### 5. Coproduct and Path

* **Coproduct** (`core/src/coproduct.rs`): a type-level `enum`, `CNil` / `Coproduct<H, T>` with `Inl`/`Inr`, the dual of HList.  `inject`, `get`, `take`, `uninject`, `subset`, `embed`, `fold`.  `#[derive(LabelledGeneric)]` on an `enum` produces a Coproduct `Repr`.
* **Path** (`core/src/path.rs`, `path!`/`Path!` macros): lens-like structural typing.  `path!(a.b.c)` builds a value, `Path!(a.b.c)` the type for `where` bounds, both driven by `PathTraverser` + `ByNameFieldPlucker`.  Lets a function accept "any type shaped like `x.y: T`" without a bespoke trait.

## Non-negotiable invariants

Breaking any of these is a defect, not a style nit.

* **No `unsafe`.**  The library is entirely safe; keep it that way.
* **`no_std` cleanliness.**  Shipped code must compile with `--no-default-features`.  Anything from `alloc` (`Vec`, `Box`, `String`, `VecDeque`, `LinkedList`, other collections) must sit behind `#[cfg(feature = "alloc")]`.  The `std` feature is a deprecated alias for `alloc`; do not add new `std`-only code paths.  Test modules may use `std` (`#[cfg(test)] extern crate std;`).
* **Stable Rust, no specialisation.**  Disambiguate overlapping impls with phantom index types (see idea 2), not `min_specialization` or negative impls.  Nightly is in the CI matrix only to catch regressions, not to enable features.
* **Reference symmetry.**  Operations generally come in owned, `&` and `&mut` flavours.  The `LabelledGeneric` derive emits `LabelledGeneric` (owned) plus `IntoLabelledGeneric` for `&` and `&mut` (via the `_frunk_ref_` lifetime injected by `ref_generics`).  HList traits often pair with `ToRef`/`ToMut` impls.  When adding an operation, consider whether the borrowed variants are also expected.
* **Base case + recursive case.**  A new HList trait needs an `HNil` impl and an `HCons<H, T>` impl; a new Coproduct trait needs `CNil` and `Coproduct<H, T>`.  Missing the base case gives confusing unresolved-trait errors.
* **Doctests are the documentation and are tested.**  `cargo test --all` runs every ` ```rust ` block; `cargo doc -D warnings` must pass.  Keep examples compiling and meaningful; they are the primary user-facing docs and are not optional.
* **serde is opt-in.**  Serialize/Deserialize derives on data types are gated with `#[cfg(feature = "serde")]`.

## Editing conventions and patterns

* **Match the surrounding style.**  Self-documenting code over comments; immutability over mutability.  Follow existing naming (`THead`/`TTail` for target, `SHead`/`STail` for source, `Index`/`Indices` for phantom indices, `Repr` for representations).
* **Diagnostics.**  Key user-facing traits (`Sculptor`, `LabelledGeneric`, `ByNameFieldPlucker`, `Transmogrifier`) carry `#[diagnostic::on_unimplemented(..)]` to turn inference failures into readable errors.  When you add a public trait that users will hit through inference, add one too.
* **Recursion limit.**  Deep type-level recursion can exceed the compiler's `recursion_limit` (`frunk_derives` sets `#![recursion_limit = "128"]`).  Do not add gratuitous recursion depth; very large HLists or deep transmogrify chains may force downstream users to raise their own limit.
* **CHANGELOG.**  User-visible changes get a `CHANGELOG.md` entry.
* **Keep the two proc-macro crates in lockstep with core.**  The derives generate paths like `::frunk_core::labelled::...`; renaming or moving a core item breaks generated code that no test in `frunk_core` alone will catch.  Run the whole workspace test suite.

## Common tasks

* **Add an HList operation** (e.g. a new fold/map variant): add the trait to `core/src/hlist.rs` with `HNil` + `HCons` impls, expose it as an inherent method on `HCons` (and `HNil` if it applies) via the `gen_inherent_methods!` macro or a dedicated `impl`, and add doctests.  Consider `Poly`/`Func` support and `to_ref`/`to_mut` variants.
* **Add a recursive, index-driven trait**: mirror `Plucker`/`Sculptor`.  Add any new phantom index type to `core/src/indices.rs`, key the head impl on `Here`, the tail impl on `There<TailIndex>`.
* **Support a new field-name character in `LabelledGeneric`.**  This requires edits in **two** places that must stay in sync:
  1. add the char's enum to `create_enums_for! { .. }` in `core/src/labelled.rs` (module `chars`);
  2. add the char to `ALPHA_CHARS` or `UNDERSCORE_CHARS` in `proc-macro-helpers/src/lib.rs`, consistent with `encode_as_ident` (letters map to themselves; `_` and digits are prefixed with `_`; other Unicode is auto-encoded between the `_uc`/`uc_` markers).  A mismatch produces compile errors only in downstream derived code.
* **Add an algebra impl** (`Semigroup`/`Monoid` for a new type): implement in `src/semigroup.rs` / `src/monoid.rs` and add a law test in `frunk_laws` (`laws/src/`) using `quickcheck`.
* **Touch the derives**: edit `derives/src/derive_generic.rs` or `derive_labelled_generic.rs`; shared token-building lives in `proc-macro-helpers/src/lib.rs`.  Note the derive covers owned + `&` + `&mut` impls and (for `LabelledGeneric`) both structs and enums; `Generic` is structs/tuple structs only.

## File map

* `core/src/hlist.rs` - HList, `Selector`, `Plucker`, `Sculptor`, `HMappable`, `HFoldLeftable`/`HFoldRightable`, `HZippable`, `IntoReverse`, `LiftFrom`/`LiftInto`, `IntoTuple2`.
* `core/src/coproduct.rs` - `Coproduct`/`CNil` and their traits.
* `core/src/generic.rs` - `Generic`, `convert_from`, `into_generic`/`from_generic`, `map_repr`/`map_inter`.
* `core/src/labelled.rs` - `LabelledGeneric`, `Field`/`ValueField`, `chars`, `ByNameFieldPlucker`, `transform_from`, `Transmogrifier`.
* `core/src/path.rs` - `Path`, `PathTraverser`.
* `core/src/indices.rs` - phantom index types (`Here`, `There`, ...).
* `core/src/traits.rs` - `Poly`, `Func`, `ToRef`, `ToMut`, `IntoReverse`.
* `core/src/tuples.rs` - tuple <-> HList/Generic interop.
* `core/src/macros.rs` - `hlist!`, `hlist_pat!`, `HList!`, `Coprod!`, `field!`, `poly_fn!`.
* `derives/src/` - `Generic` and `LabelledGeneric` derives.
* `proc-macros/src/lib.rs` - `path!`, `Path!`.
* `proc-macro-helpers/src/lib.rs` - label encoding and AST builders shared by the two proc-macro crates.
* `src/{monoid,semigroup,validated}.rs` - algebras and the `Validated` error accumulator.
* `laws/src/` - property tests for the algebraic laws.
* `tests/`, `examples/`, `benches/` - integration tests, runnable examples, benchmarks.
