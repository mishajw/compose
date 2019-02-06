use core::spec::FromValue;
use core::spec::Value;
use core::spec::Spec;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

/// Plays the step it's played on
pub struct Linear {
    scale: i32
}

impl Linear {
    #[allow(missing_docs)]
    pub fn player(scale: i32) -> Linear { Linear { scale } }
}

impl Player for Linear {
    fn play(&mut self, state: &State) -> Playable {
        Playable::new(state.tick as i32 * self.scale )
    }
}

impl Tree for Linear {
    fn to_tree(&self) -> &Tree { self as &Tree }
}

impl FromValue for Linear {
    fn name() -> &'static str { "linear" }

    fn from_value(value: Value, consts: &Consts) -> Result<Self> {
        let mut spec: Spec = value.into_type(consts)?;
        let scale: i32 = spec.consume_with_default("scale", 1, consts)?;
        Ok(Linear::player(scale))
    }
}
