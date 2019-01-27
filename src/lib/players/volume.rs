use core::input;
use core::spec::FromValue;
use core::spec::{Spec, Value};
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

/// Adjust the volume of a child player
pub struct Volume {
    child: Box<Player>,
    input: Box<input::Bounded>,
}

impl Volume {
    #[allow(missing_docs)]
    pub fn player(child: Box<Player>, input: Box<input::Bounded>) -> Volume {
        Volume { child, input }
    }
}

impl Player for Volume {
    fn play(&mut self, state: &State) -> Playable {
        self.child.play(state) * self.input.get_with_bounds(state, 0.0, 1.0)
    }
}

impl Tree for Volume {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.child.to_tree(), self.input.to_tree()]
    }
}

impl FromValue for Volume {
    fn name() -> &'static str { "volume" }
    fn from_value(value: Value, consts: &Consts) -> Result<Self> {
        let mut spec: Spec = value.into_type(consts)?;
        let child = spec.consume("child", consts)?;
        let input = spec.consume("input", consts)?;
        spec.ensure_all_used()?;
        Ok(Volume::player(child, input))
    }
}
