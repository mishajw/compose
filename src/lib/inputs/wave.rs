use core::input;
use core::CompositionState;
use errors::*;
use spec::{FromSpec, Spec};

/// A wave input, gets values from a function at a frequency
pub struct Wave {
    wave_fn: Box<Fn(f32) -> f32>,
    frequency: f32,
}

impl input::Continuous for Wave {
    fn get(&self, state: &CompositionState) -> f32 {
        let num_ticks = state.frequency / self.frequency;
        let fn_input = state.tick as f32 / num_ticks;
        (*self.wave_fn)(fn_input)
    }
}

impl FromSpec<Box<input::Continuous>> for Wave {
    fn name() -> &'static str { "wave" }

    fn from_spec(spec: &mut Spec) -> Result<Box<input::Continuous>> {
        let wave_fn_name = spec.use_str("fn")?;
        let frequency = spec.use_float("frequency")?;
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
