//! Writing the output of a composition

use core::Playable;

/// Writes `Playable`s into an output one-by-one
pub trait Output {
    /// Write a playable to the output
    fn write(&mut self, playable: Playable);
}
