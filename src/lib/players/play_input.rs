use core::input;
use core::tree::Tree;
use core::Playable;
use core::Player;
use core::State;

use std::i32;

/// Play directly from a bounded input
pub struct PlayInput {
    input: Box<input::Bounded>,
}

impl PlayInput {
    #[allow(missing_docs)]
    pub fn new(input: Box<input::Bounded>) -> Box<Player> {
        Box::new(PlayInput { input })
    }
}

impl Player for PlayInput {
    fn play(&mut self, state: &State) -> Playable {
        Playable::new(self.input.get_with_bounds(
            state,
            i32::MIN as f32 * state.consts.loudness_factor,
            i32::MAX as f32 * state.consts.loudness_factor,
        ) as i32)
    }
}

impl Tree for PlayInput {
    fn to_tree<'a>(&'a self) -> &'a Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.input.to_tree()]
    }
}
