use core::input;
use core::CompositionState;
use core::Playable;
use core::Player;
use errors::*;
use inputs;
use spec::{FromSpec, Spec};

/// Play a wave from a wave function
pub struct Wave {
    input_fn: Box<input::Continuous>,
}

impl Player for Wave {
    fn play(&self, state: &CompositionState) -> Playable {
        // The wave function needs to be cast to i32 and put in the correct
        // range
        Playable::new(
            (self.input_fn.get(state) * ::std::i32::MAX as f32) as i32,
        )
    }
}

impl FromSpec<Box<Player>> for Wave {
    fn name() -> &'static str { "wave" }

    fn from_spec(spec: &mut Spec) -> Result<Box<Player>> {
        let input_fn = inputs::Wave::from_spec(spec)?;
        spec.ensure_all_used()?;
        Ok(Box::new(Wave { input_fn }))
    }
}
