use core::input;
use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

field_decl!(CHILD, Box<Player>, "Child to change the volume of");
field_decl!(INPUT, Box<input::Bounded>, "Controls the volume level");

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

impl SpecType for Volume {
    fn name() -> &'static str { "volume" }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![CHILD.to_description(), INPUT.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let child = CHILD.get(&mut spec, consts)?;
        let input = INPUT.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(Volume::player(child, input))
    }
}
