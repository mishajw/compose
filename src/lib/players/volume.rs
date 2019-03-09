use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Input;
use core::Playable;
use core::Player;
use core::State;
use error::*;

field_decl!(CHILD, Box<Player>, "Child to change the volume of");
field_decl!(INPUT, Box<Input>, "Controls the volume level");

/// Adjust the volume of a child player
pub struct Volume {
    child: Box<Player>,
    input: Box<Input>,
}

impl Volume {
    #[allow(missing_docs)]
    pub fn player(child: Box<Player>, input: Box<Input>) -> Volume {
        Volume { child, input }
    }
}

impl Player for Volume {
    fn play(&mut self, state: &State) -> Playable {
        self.child.play(state) * self.input.get(state)
    }
}

impl Tree for Volume {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.child.to_tree(), self.input.to_tree()]
    }
}

impl SpecType for Volume {
    fn name() -> String { "volume".into() }

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
