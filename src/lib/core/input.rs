//! Inputs that control [`Player`](../trait.Player.html)s

use core::CompositionState;

/// Continuous unbounded input
pub trait Continuous {
    /// Get the next value in the input
    fn get(&self, state: &CompositionState) -> f32;
}
