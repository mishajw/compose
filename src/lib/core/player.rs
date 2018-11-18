//! Produces music

use core::CompositionState;
use core::Playable;

/// Creates music from scratch, or other `Player`s
pub trait Player: Send + Sync {
    /// Create the next `Playable`, given some progress through the composition
    fn play(&mut self, state: &CompositionState) -> Playable;
}
