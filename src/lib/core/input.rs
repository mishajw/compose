//! Inputs that control [`Player`](../trait.Player.html)s

use core::State;

/// An input that is is guarenteed to be between two values
pub trait Bounded: Send + Sync {
    /// Get the input value between the bounds given by `get_bounds`
    fn get(&mut self, state: &State) -> f32;

    /// Get the lower and upper bounds (respectively) of the number returned by
    /// `get`
    fn get_bounds(&self) -> (f32, f32);

    /// Get the value bounded between two values
    fn get_with_bounds(
        &mut self,
        state: &State,
        lower: f32,
        upper: f32,
    ) -> f32
    {
        let value = self.get(state);
        let (self_lower, self_upper) = self.get_bounds();
        let scaling_factor = (upper - lower) / (self_upper - self_lower);
        (value - self_lower) * scaling_factor + lower
    }
}

/// Input that is either true or false
pub trait Bool: Send + Sync {
    #[allow(missing_docs)]
    fn get(&mut self, state: &State) -> bool;
}
