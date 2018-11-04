use core::input;
use core::CompositionState;
use core::Playable;
use core::Player;
use errors::*;
use inputs;
use spec::{FromSpec, Value};

use std::i32;

/// Play a wave from a wave function
pub struct Wave {
    input: Box<input::Bounded>,
}

impl Player for Wave {
    fn play(&mut self, state: &CompositionState) -> Playable {
        // The wave function needs to be cast to i32 and put in the correct
        // range
        Playable::new(self.input.get_with_bounds(
            state,
            i32::MIN as f32,
            i32::MAX as f32,
        ) as i32)
    }
}

impl FromSpec<Box<Player>> for Wave {
    fn name() -> &'static str { "wave" }

    fn from_spec(value: Value) -> Result<Box<Player>> {
        let input = inputs::Wave::from_spec(value)?;
        Ok(Box::new(Wave { input }))
    }
}
