//! Create compositions

use core::CompositionState;
use core::Output;
use core::Player;

/// Create a composition from the `root_player` into the `output`s
pub fn compose(root_player: &Player, outputs: Vec<&Output>, frequency: u64) {
    let mut state = CompositionState::initial(frequency);

    loop {
        let played = root_player.play(&state);
        for output in &outputs {
            output.write(played.clone());
        }
        state.increment();
    }
}
