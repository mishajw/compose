//! Produces music

use core::tree::Tree;
use core::Playable;
use core::State;

/// Creates music from scratch, or other `Player`s
pub trait Player: Tree + Send + Sync {
    /// Create the next `Playable`, given some progress through the composition
    fn play(&mut self, state: &State) -> Playable;
}
