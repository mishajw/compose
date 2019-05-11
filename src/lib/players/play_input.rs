use core::tree::Tree;
use core::Consts;
use core::Input;
use core::Playable;
use core::Player;
use core::State;
use inputs::InputMod;

use std::i32;

/// Play directly from a bounded input
pub struct PlayInput {
    input: Box<Input>,
}

impl PlayInput {
    #[allow(missing_docs)]
    pub fn new(input: Box<Input>, consts: &Consts) -> Box<Player> {
        let mult = consts.loudness_factor * i32::MAX as f64;
        Box::new(PlayInput {
            input: Box::new(InputMod::new(input, 0.0, mult)),
        })
    }
}

impl Player for PlayInput {
    fn play(&mut self, state: &State) -> Playable {
        Playable::new(self.input.get(state) as i32)
    }
}

impl Tree for PlayInput {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children(&self) -> Vec<&Tree> {
        vec![self.input.to_tree()]
    }
}
