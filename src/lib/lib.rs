//! Library for creating music from a tree of
//! [`Player`](core/trait.Player.html)s.
//!
//! See [`core`](core/) for an overview of the design.
//!
//! See [`spec`](spec/) for an overview of how to create a composition.

#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;

pub mod core;
pub mod errors;
pub mod spec;
