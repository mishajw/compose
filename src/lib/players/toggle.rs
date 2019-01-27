use core::input;
use core::spec::FromValue;
use core::spec::{Spec, Value};
use core::Consts;
use core::Player;
use error::*;
use inputs::BoolToBounded;
use players::Volume;

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

impl FromValue<Volume> for Toggle {
    fn name() -> &'static str { "toggle" }
    fn from_value(value: Value, consts: &Consts) -> Result<Volume> {
        let mut spec: Spec = value.into_type(consts)?;
        let child: Box<Player> = spec.consume("child", consts)?;
        let bool_input: Box<input::Bool> = spec.consume("input", consts)?;
        Ok(Toggle::from_bool(child, bool_input))
    }
}
