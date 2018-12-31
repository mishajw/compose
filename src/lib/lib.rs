//! Library for creating music from a tree of
//! [`Player`](core/trait.Player.html)s.
//!
//! See [`core`](core/) for an overview of the design.
//!
//! See [`players`](players/) for what kind of music can be created.
//!
//! See [`spec`](spec/) for an overview of how to create a composition.

#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate portaudio;
extern crate yaml_rust;
#[macro_use]
extern crate log;
extern crate hound;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod core;
pub mod error;
pub mod inputs;
pub mod macros;
pub mod outputs;
pub mod players;
