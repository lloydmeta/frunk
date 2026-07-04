# Copilot instructions for reviewing frunk

Frunk is a functional programming toolbelt for Rust (`HList`, `Coproduct`, `Generic`, `LabelledGeneric`, `Path`, `Validated`, `Semigroup`, `Monoid`).  Almost all of it is implemented with type-level programming on **stable Rust**, with **no `unsafe`** and **no specialisation**.  The maintainers rank safety (type and memory), correctness and efficiency above all else.  Review with that priority order.

When reviewing a pull request, be thorough.  Anchor each comment to a file and line, explain the concrete failure it causes, and separate blocking correctness/safety issues from optional style nits.  The items below are the failure modes specific to this codebase; check every one that the diff touches.

## Safety and platform constraints (blocking)

* Flag any introduction of `unsafe`.  This library is entirely safe and must stay that way; an `unsafe` block needs an extremely strong, explicitly justified reason.
* The crates are `#![no_std]`.  Flag any use of `std::` in shipped code, and any use of an `alloc` type (`Vec`, `Box`, `String`, `VecDeque`, `LinkedList`, `BTreeMap`, and similar) that is **not** behind `#[cfg(feature = "alloc")]`.  Test-only code (`#[cfg(test)]`) may use `std`.
* The `std` cargo feature is a deprecated alias for `alloc`.  Flag new code paths gated only on `std`; they should use `alloc`.
* Flag reliance on nightly-only or unstable features (specialisation, negative impls, GATs beyond what stable allows) in shipped code.  Nightly appears in CI only to detect regressions.

## Type-level recursion and trait impls (blocking correctness)

* Overlapping trait impls on `HCons`/`Coproduct` are disambiguated by a phantom `Index` type parameter (see `core/src/indices.rs`: `Here`, `There<T>`, and friends).  For any new recursive trait over `HList` or `Coproduct`, check there is a **base case** (`HNil` or `CNil`) and a **recursive case** (`HCons<H, T>` / `Coproduct<H, T>`), and that the recursive impl threads an index (typically head keyed on `Here`, tail keyed on `There<TailIndex>` with a matching `where Tail: Trait<_, TailIndex>` bound).  A missing base case or a non-disjoint impl is a defect.
* Index type parameters are meant to be solved by inference, never written by users.  Flag public APIs that force callers to name an index type.
* Watch for changes that deepen type-level recursion (larger tuple arities, deeper transmogrify chains).  These can exceed the compiler's `recursion_limit` (the derives set `128`).  Call out anything likely to push downstream users past their limit.

## Generic and LabelledGeneric correctness (blocking)

* `Generic`/`LabelledGeneric` must form an isomorphism: `from(into(x)) == x` and `into(from(y)) == y`.  In derive or hand-written `into`/`from`, verify field order is consistent between the two directions; a reordering bug here silently corrupts data.
* `Generic` conversions are **positional** (types only) and `LabelledGeneric` conversions are **name-aware** (`Field<Name, Value>`).  Flag review-worthy conversions that use positional `Generic::convert_from` where field meanings could be transposed, and prefer the labelled path where names matter.
* `transform_from` (via `Sculptor`) and `transmogrify` (via `Transmogrifier`) intentionally **discard the sculpted remainder**.  Confirm that dropping the unused source fields is intended for the change under review.

## Field-name character encoding must stay in sync (blocking)

`LabelledGeneric` encodes field names as type-level strings built from char enums.  Two locations must agree:

* the char enums declared by `create_enums_for! { .. }` in `core/src/labelled.rs` (module `chars`), and
* the `ALPHA_CHARS` / `UNDERSCORE_CHARS` tables and `encode_as_ident` logic in `proc-macro-helpers/src/lib.rs`.

If a PR touches one, verify it updates the other consistently (letters map to themselves; `_` and digits are `_`-prefixed; other Unicode is sandwiched between `_uc`/`uc_`).  A mismatch compiles in `frunk_core` but breaks downstream derived code, so it will not be caught by that crate's own tests.

## Reference symmetry and derive codegen

* Operations usually come in owned, `&`, and `&mut` flavours.  The `LabelledGeneric` derive emits `LabelledGeneric` plus `IntoLabelledGeneric` for `&` and `&mut` (using the injected `'_frunk_ref_` lifetime).  When derive codegen or an HList trait changes, check the borrowed variants (`ToRef`/`ToMut`, the ref/mut impls) are updated in step.
* Generated code must reference core items by fully-qualified path (`::frunk_core::...`) and keep the `#[allow(non_snake_case, non_camel_case_types)]` hygiene attributes.  Flag renames/moves of `frunk_core` items that the derives reference but no `frunk_core`-only test would catch.

## Tests, docs, and API surface

* Doctests are the primary user documentation and are executed by `cargo test --all`.  New or changed public items should have doctests; flag public additions without them, and flag examples that will not compile.  `cargo doc` runs with `-D warnings`, so broken intra-doc links fail CI.
* New public traits that users hit through inference should carry `#[diagnostic::on_unimplemented(..)]` for readable errors (see `Sculptor`, `LabelledGeneric`, `ByNameFieldPlucker`, `Transmogrifier`).
* New public items in `frunk_core` should be re-exported appropriately from the `frunk` facade (`src/lib.rs`, and the `prelude` for extension traits), and user-facing behaviour changes should be reflected in `README.md` and `CHANGELOG.md`.
* New `Semigroup`/`Monoid` impls should come with `quickcheck` law tests in `frunk_laws` (`laws/src/`).
* serde `Serialize`/`Deserialize` derives on data types must be gated with `#[cfg(feature = "serde")]`.

## Workspace hygiene

* This is a Cargo workspace with internal path dependencies that also pin a `version`.  When a crate's version is bumped, check every dependent crate's dependency entry is updated to match.
* The change must pass `cargo fmt --all -- --check` and `cargo clippy --all --tests -- -D warnings` (clippy warnings are hard errors, including in tests).  Flag obvious formatting or clippy violations.
