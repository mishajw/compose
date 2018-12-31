use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::Consts;
use core::Player;
use error::*;
use inputs::BoolToBounded;
use inputs::SmoothBool;
use players::Volume;

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

impl create::FromSpec<Box<Player>> for Toggle {
    fn name() -> &'static str { "toggle" }
    fn from_spec(value: Value, consts: &Consts) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let child = create::create_player(&mut spec.consume("child")?, consts)?;
        let mut bool_spec = spec.consume("input")?;

        if let Some(smooth_bool_spec) =
            spec.consume_optional::<Spec>("smooth")?
        {
            let smooth_bool = SmoothBool::from_spec(
                Value::Spec(smooth_bool_spec.with("input".into(), bool_spec)),
                consts,
            )?;
            Ok(Toggle::from_bounded(child, smooth_bool))
        } else {
            let bool_input = create::create_bool_input(&mut bool_spec, consts)?;
            Ok(Toggle::from_bool(child, bool_input))
        }
    }
}
