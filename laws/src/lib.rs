#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Frunk Laws
//!
//! This library contains laws that can be used to test the implementation of
//! algebras declared in Frunk

extern crate frunk;
extern crate quickcheck;

pub mod semigroup_laws;
pub mod monoid_laws;
pub mod wrapper;
