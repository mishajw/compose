use core::Player;
use errors::*;
use inputs::BoolToBounded;
use players::Volume;
use spec::{create_bool_input, create_player, FromSpec, Value};

/// Toggle a player on and off
pub struct Toggle {}

impl Toggle {}

impl FromSpec<Box<Player>> for Toggle {
    fn name() -> &'static str { "toggle" }
    fn from_spec(value: Value) -> Result<Box<Player>> {
        let mut spec = value.as_spec()?;
        let child = create_player(&mut spec.use_spec("child")?)?;
        let bool_input = create_bool_input(&mut spec.use_spec("input")?)?;
        let bounded_input = BoolToBounded::new(bool_input);

        Ok(Box::new(Volume::new(child, Box::new(bounded_input))))
    }
}
