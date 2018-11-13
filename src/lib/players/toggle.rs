use core::input;
use core::Player;
use errors::*;
use inputs::BoolToBounded;
use players::Volume;
use spec::{create_bool_input, create_player, FromSpec, Spec, Value};

/// Toggle a player on and off
pub struct Toggle {}

impl Toggle {
    #[allow(missing_docs)]
    pub fn new(
        child: Box<Player>,
        bool_input: Box<input::Bool>,
    ) -> Box<Player>
    {
        let bounded_input = BoolToBounded::new(bool_input);
        Box::new(Volume::new(child, Box::new(bounded_input)))
    }
}

impl FromSpec<Box<Player>> for Toggle {
    fn name() -> &'static str { "toggle" }
    fn from_spec(value: Value) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let child = create_player(&mut spec.consume("child")?)?;
        let bool_input = create_bool_input(&mut spec.consume("input")?)?;
        Ok(Toggle::new(child, bool_input))
    }
}
