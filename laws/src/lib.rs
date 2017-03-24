#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk Laws
//!
//! This library contains laws that can be used to test the implementation of
//! algebras declared in Frunk

extern crate frunk;
#[cfg(test)]
extern crate quickcheck;

pub mod semigroup_laws;
