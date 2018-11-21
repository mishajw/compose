use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::CompositionConsts;
use core::CompositionState;
use core::Playable;
use core::Player;
use errors::*;

/// Adjust the volume of a child player
pub struct Volume {
    child: Box<Player>,
    input: Box<input::Bounded>,
}

impl Volume {
    #[allow(missing_docs)]
    pub fn new(child: Box<Player>, input: Box<input::Bounded>) -> Box<Player> {
        Box::new(Volume { child, input })
    }
}

impl Player for Volume {
    fn play(&mut self, state: &CompositionState) -> Playable {
        self.child.play(state) * self.input.get_with_bounds(state, 0.0, 1.0)
    }
}

impl create::FromSpec<Box<Player>> for Volume {
    fn name() -> &'static str { "volume" }
    fn from_spec(
        value: Value,
        consts: &CompositionConsts,
    ) -> Result<Box<Player>>
    {
        let mut spec: Spec = value.as_type()?;
        Ok(Volume::new(
            create::create_player(&mut spec.consume("child")?, consts)?,
            create::create_bounded_input(&mut spec.consume("input")?, consts)?,
        ))
    }
}
