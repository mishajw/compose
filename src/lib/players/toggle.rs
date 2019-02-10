use core::input;
use core::spec::FieldDeclaration;
use core::spec::FieldDescription;
use core::spec::Spec;
use core::spec::SpecType;
use core::Consts;
use core::Player;
use error::*;
use inputs::BoolToBounded;
use players::Volume;

field_decl!(CHILD, Box<Player>, "Child to toggle on and off");
field_decl!(INPUT, Box<input::Bool>, "Controls the toggling");

/// Toggle a player on and off
pub struct Toggle {}

impl Toggle {
    #[allow(missing_docs)]
    pub fn from_bool(
        child: Box<Player>,
        bool_input: Box<input::Bool>,
    ) -> Volume
    {
        let bounded_input = BoolToBounded::new(bool_input);
        Volume::player(child, Box::new(bounded_input))
    }

    #[allow(missing_docs)]
    pub fn from_bounded(
        child: Box<Player>,
        bounded_input: Box<input::Bounded>,
    ) -> Volume
    {
        Volume::player(child, bounded_input)
    }
}

impl SpecType<Volume> for Toggle {
    fn name() -> &'static str { "toggle" }

    fn field_descriptions() -> Vec<FieldDescription> {
        vec![CHILD.to_description(), INPUT.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Volume> {
        let child = CHILD.get(&mut spec, consts)?;
        let input = INPUT.get(&mut spec, consts)?;
        Ok(Toggle::from_bool(child, input))
    }
}
