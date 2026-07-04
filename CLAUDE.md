# CLAUDE.md

The agentic instructions for this repository live in [`AGENT.md`](./AGENT.md).  Read it first.

@AGENT.md

## Quick reference

Before proposing a change, run what CI enforces (details and the `no_std` target in `AGENT.md`):

```sh
cargo fmt --all -- --check
cargo clippy --all --tests -- -D warnings
cargo test --all --no-fail-fast
RUSTDOCFLAGS='-D warnings' cargo doc --all --no-deps --document-private-items
```

Non-negotiables (see `AGENT.md` for the full list and the reasoning): no `unsafe`; stays `no_std`-clean with `alloc` items gated behind `#[cfg(feature = "alloc")]`; stable Rust with no specialisation; overlapping impls disambiguated by phantom index types; doctests must compile and pass.
