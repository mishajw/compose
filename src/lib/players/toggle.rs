use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::Consts;
use core::Input;
use core::Player;
use error::*;
use players::Volume;

field_decl!(CHILD, Box<Player>, "Child to toggle on and off");
field_decl!(INPUT, Box<Input>, "Controls the toggling");

/// Toggle a player on and off
pub struct Toggle {}

impl Toggle {
    #[allow(missing_docs)]
    pub fn new(child: Box<Player>, input: Box<Input>) -> Volume {
        Volume::player(child, input)
    }
}

impl SpecType<Volume> for Toggle {
    fn name() -> String { "toggle".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![CHILD.to_description(), INPUT.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Volume> {
        let child = CHILD.get(&mut spec, consts)?;
        let input = INPUT.get(&mut spec, consts)?;
        Ok(Toggle::new(child, input))
    }
}
