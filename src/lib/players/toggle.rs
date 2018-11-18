use core::input;
use core::Player;
use errors::*;
use inputs::BoolToBounded;
use inputs::SmoothBool;
use players::Volume;
use spec::{create_bool_input, create_player, FromSpec, Spec, Value};

/// Toggle a player on and off
pub struct Toggle {}

impl Toggle {
    #[allow(missing_docs)]
    pub fn from_bool(
        child: Box<Player>,
        bool_input: Box<input::Bool>,
    ) -> Box<Player>
    {
        let bounded_input = BoolToBounded::new(bool_input);
        Volume::new(child, Box::new(bounded_input))
    }

    #[allow(missing_docs)]
    pub fn from_bounded(
        child: Box<Player>,
        bounded_input: Box<input::Bounded>,
    ) -> Box<Player>
    {
        Volume::new(child, bounded_input)
    }
}

impl FromSpec<Box<Player>> for Toggle {
    fn name() -> &'static str { "toggle" }
    fn from_spec(value: Value) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let child = create_player(&mut spec.consume("child")?)?;
        let mut bool_spec = spec.consume("input")?;

        if let Some(smooth_bool_spec) =
            spec.consume_optional::<Spec>("smooth")?
        {
            let smooth_bool = SmoothBool::from_spec(Value::Spec(
                smooth_bool_spec.with("input".into(), bool_spec),
            ))?;
            Ok(Toggle::from_bounded(child, smooth_bool))
        } else {
            let bool_input = create_bool_input(&mut bool_spec)?;
            Ok(Toggle::from_bool(child, bool_input))
        }
    }
}
