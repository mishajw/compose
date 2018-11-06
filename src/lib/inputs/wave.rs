use consts;
use core::input;
use core::CompositionState;
use errors::*;
use spec::{FromSpec, Value};

/// A wave input, gets values from a function at a frequency
pub struct Wave {
    wave_fn: Box<Fn(f32) -> f32>,
    frequency: f32,
}

impl input::Bounded for Wave {
    fn get(&mut self, state: &CompositionState) -> f32 {
        let num_ticks = consts::SAMPLE_HZ / self.frequency;
        let fn_input = state.tick as f32 / num_ticks;
        (*self.wave_fn)(fn_input)
    }

    fn get_bounds(&self) -> (f32, f32) { (-1.0, 1.0) }
}

impl FromSpec<Box<input::Bounded>> for Wave {
    fn name() -> &'static str { "wave" }

    fn from_spec(value: Value) -> Result<Box<input::Bounded>> {
        let mut spec = value.as_spec()?;
        let wave_fn_name: String =
            spec.consume_with_default("fn", "sine".into())?;
        let frequency = spec.consume_with_default("frequency", 1.0)?;
        let wave_fn = match wave_fn_name.as_ref() {
            "sine" => |x| f32::sin(x * 2.0 * ::std::f32::consts::PI),
            value => {
                return Err(
                    ErrorKind::SpecBadValue("fn".into(), value.into()).into()
                )
            }
        };

        spec.ensure_all_used()?;
        Ok(Box::new(Wave {
            wave_fn: Box::new(wave_fn),
            frequency,
        }))
    }
}
