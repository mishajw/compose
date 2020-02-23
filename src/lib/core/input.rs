use core::tree::Tree;
use core::State;

/// Input that control [`Player`](../trait.Player.html)s
pub trait Input: Tree + Send + Sync {
    #[allow(missing_docs)]
    fn get(&mut self, state: &State) -> f64;

    /// Casts float input to a boolean input. Is false if == 0
    fn get_bool(&mut self, state: &State) -> bool {
        self.get(state) != 0.0
    }
}
