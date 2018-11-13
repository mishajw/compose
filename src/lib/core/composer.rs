//! Create compositions

use core::CompositionConsts;
use core::CompositionState;
use core::Output;
use core::Player;

/// Create a composition from the `root_player` into the `output`s
pub fn compose(
    root_player: &mut Player,
    mut outputs: Vec<Box<Output>>,
    consts: CompositionConsts,
)
{
    let mut state = CompositionState::initial(consts);
    loop {
        let played = root_player.play(&state);
        for output in outputs.iter_mut() {
            output.write(played);
        }
        state.increment();
    }
}
