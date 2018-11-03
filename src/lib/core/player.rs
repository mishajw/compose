//! Produces music

use core::CompositionState;
use core::Playable;

/// Creates music from scratch, or other `Player`s
pub trait Player {
    /// Create the next `Playable`, given some progress through the composition
    fn play(&self, state: &CompositionState) -> Playable;
}
