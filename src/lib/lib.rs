//! Library for creating music from a tree of
//! [`Player`](core/trait.Player.html)s.
//!
//! See [`core`](core/) for an overview of the design.
//!
//! See [`players`](players/) for what kind of music can be created.
//!
//! See [`spec`](spec/) for an overview of how to create a composition.

#![warn(missing_docs)]
// TODO: Rename `new` functions so we don't have to silence this warning.
#![allow(clippy::new_ret_no_self)]

/// Default configuration file path
pub const DEFAULT_CONFIG_PATH: &str = "./composer.config";

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
extern crate num;
extern crate rand;
extern crate rustfft;
extern crate sfml;

#[macro_use]
pub mod core;

pub mod error;
mod fourier;
pub mod gui;
pub mod inputs;
pub mod macros;
pub mod outputs;
pub mod players;
pub mod pycomposer;
mod sample_bucketer;

pub use fourier::fourier;
pub use sample_bucketer::SampleBucketer;
