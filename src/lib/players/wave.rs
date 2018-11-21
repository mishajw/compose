use core::input;
use core::spec::create;
use core::spec::Value;
use core::CompositionState;
use core::Playable;
use core::Player;
use errors::*;
use inputs;

use core::spec::Spec;
use core::CompositionConsts;
use players::Speed;
use std::i32;

/// Play a wave from a wave function
pub struct Wave {
    input: Box<input::Bounded>,
}

impl Wave {
    #[allow(missing_docs)]
    pub fn new(input: Box<input::Bounded>, frequency: f32) -> Box<Player> {
        Speed::new(Box::new(Wave { input }), frequency)
    }
}

impl Player for Wave {
    fn play(&mut self, state: &CompositionState) -> Playable {
        // The wave function needs to be cast to i32 and put in the correct
        // range
        Playable::new(self.input.get_with_bounds(
            state,
            i32::MIN as f32 * state.consts.loudness_factor,
            i32::MAX as f32 * state.consts.loudness_factor,
        ) as i32)
    }
}

impl create::FromSpec<Box<Player>> for Wave {
    fn name() -> &'static str { "wave" }

    fn from_spec(
        value: Value,
        _consts: &CompositionConsts,
    ) -> Result<Box<Player>>
    {
        let mut spec: Spec = value.as_type()?;
        let function = inputs::Function::from_string(
            spec.consume_with_default("fn", "sine".into())?,
        )?;
        let frequency: f32 = spec.consume("frequency")?;
        Ok(Wave::new(function, frequency))
    }
}
