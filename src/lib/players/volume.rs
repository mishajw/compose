use core::input;
use core::CompositionState;
use core::Playable;
use core::Player;
use errors::*;
use spec::{create_bounded_input, create_player, FromSpec, Value};

/// Adjust the volume of a child player
pub struct Volume {
    child: Box<Player>,
    input: Box<input::Bounded>,
}

impl Volume {
    #[allow(missing_docs)]
    pub fn new(child: Box<Player>, input: Box<input::Bounded>) -> Self {
        Volume { child, input }
    }
}

impl Player for Volume {
    fn play(&mut self, state: &CompositionState) -> Playable {
        self.child.play(state) * self.input.get_with_bounds(state, 0.0, 1.0)
    }
}

impl FromSpec<Box<Player>> for Volume {
    fn name() -> &'static str { "volume" }
    fn from_spec(value: Value) -> Result<Box<Player>> {
        let mut spec = value.as_spec()?;
        Ok(Box::new(Volume::new(
            create_player(&mut spec.use_spec("child")?)?,
            create_bounded_input(&mut spec.use_spec("input")?)?,
        )))
    }
}
