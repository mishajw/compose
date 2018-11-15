use core::input;
use core::Player;
use core::Time;
use errors::*;
use inputs::SmoothBool;
use players::Volume;
use spec::create_bounded_input;
use spec::{create_bool_input, create_player, FromSpec, Spec, Value};

/// Toggle a player on and off, smoothing the transition
pub struct SmoothToggle {}

impl SmoothToggle {
    #[allow(missing_docs)]
    pub fn new(
        child: Box<Player>,
        bool_input: Box<input::Bool>,
        smooth_fn: Box<input::Bounded>,
        smooth_in_duration: Time,
        smooth_out_duration: Time,
    ) -> Box<Player>
    {
        let bounded_input = SmoothBool::new(
            bool_input,
            smooth_fn,
            smooth_in_duration,
            smooth_out_duration,
        );
        Box::new(Volume::new(child, bounded_input))
    }

    #[allow(missing_docs)]
    pub fn default_fn(
        child: Box<Player>,
        bool_input: Box<input::Bool>,
        smooth_in_duration: Time,
        smooth_out_duration: Time,
    ) -> Box<Player>
    {
        let bounded_input = SmoothBool::default_fn(
            bool_input,
            smooth_in_duration,
            smooth_out_duration,
        );
        Box::new(Volume::new(child, bounded_input))
    }
}

impl FromSpec<Box<Player>> for SmoothToggle {
    fn name() -> &'static str { "smooth-toggle" }
    fn from_spec(value: Value) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let child = create_player(&mut spec.consume("child")?)?;
        let bool_input = create_bool_input(&mut spec.consume("input")?)?;
        let smooth_in_duration =
            Time::from_spec(spec.consume("smooth-in-duration")?)?;
        let smooth_out_duration =
            Time::from_spec(spec.consume("smooth-out-duration")?)?;
        match spec.consume_optional::<Spec>("smooth-fn")? {
            Some(mut smooth_fn) => {
                let smooth_fn = create_bounded_input(&mut smooth_fn)?;
                Ok(SmoothToggle::new(
                    child,
                    bool_input,
                    smooth_fn,
                    smooth_in_duration,
                    smooth_out_duration,
                ))
            }
            None => Ok(SmoothToggle::default_fn(
                child,
                bool_input,
                smooth_in_duration,
                smooth_out_duration,
            )),
        }
    }
}
