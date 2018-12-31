use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::Consts;
use core::State;
use core::Time;
use error::*;
use inputs::Function;

/// Smooth a bool transition
pub struct SmoothBool {
    bool_input: Box<input::Bool>,
    smooth_fn: Box<input::Bounded>,
    smooth_in_duration: Time,
    smooth_out_duration: Time,
    activation: f32,
}

impl SmoothBool {
    #[allow(missing_docs)]
    pub fn new(
        bool_input: Box<input::Bool>,
        smooth_fn: Box<input::Bounded>,
        smooth_in_duration: Time,
        smooth_out_duration: Time,
    ) -> Box<input::Bounded>
    {
        Box::new(SmoothBool {
            bool_input,
            smooth_fn,
            smooth_in_duration,
            smooth_out_duration,
            activation: 0.0,
        })
    }

    #[allow(missing_docs)]
    pub fn default_fn(
        bool_input: Box<input::Bool>,
        smooth_in_duration: Time,
        smooth_out_duration: Time,
    ) -> Box<input::Bounded>
    {
        SmoothBool::new(
            bool_input,
            Function::with_mod(Box::new(|x| x), 0.0, 1.0, Time::Seconds(1.1)),
            smooth_in_duration,
            smooth_out_duration,
        )
    }
}

impl input::Bounded for SmoothBool {
    fn get(&mut self, state: &State) -> f32 {
        let input = self.bool_input.get(state);
        if input && self.activation < 1.0 {
            self.activation +=
                1.0 / self.smooth_in_duration.to_ticks(&state.consts) as f32;
            self.activation = self.activation.min(1.0);
        } else if !input && self.activation > 0.0 {
            self.activation -=
                1.0 / self.smooth_out_duration.to_ticks(&state.consts) as f32;
            self.activation = self.activation.max(0.0);
        }
        self.smooth_fn.get_with_bounds(
            &state.with_tick(
                Time::Seconds(self.activation).to_ticks(&state.consts),
            ),
            0.0,
            1.0,
        )
    }

    fn get_bounds(&self) -> (f32, f32) { (0.0, 1.0) }
}

impl create::FromSpec<Box<input::Bounded>> for SmoothBool {
    fn name() -> &'static str { "smooth-bool" }

    fn from_spec(value: Value, consts: &Consts) -> Result<Box<input::Bounded>> {
        let mut spec: Spec = value.as_type()?;
        let input =
            create::create_bool_input(&mut spec.consume("input")?, consts)?;
        let smooth_in_duration =
            Time::from_spec(spec.consume("smooth-in-duration")?, consts)?;
        let smooth_out_duration =
            Time::from_spec(spec.consume("smooth-out-duration")?, consts)?;
        match spec.consume_optional::<Spec>("smooth-fn")? {
            Some(mut smooth_fn) => {
                let smooth_fn =
                    create::create_bounded_input(&mut smooth_fn, consts)?;
                Ok(SmoothBool::new(
                    input,
                    smooth_fn,
                    smooth_in_duration,
                    smooth_out_duration,
                ))
            }
            None => Ok(SmoothBool::default_fn(
                input,
                smooth_in_duration,
                smooth_out_duration,
            )),
        }
    }
}
