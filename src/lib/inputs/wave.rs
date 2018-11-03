use core::input;
use core::CompositionState;
use errors::*;
use spec::{FromSpec, Spec};

/// A wave input, gets values from a function at a frequency
pub struct Wave {
    wave_fn: Box<Fn(f32) -> f32>,
    frequency: u64,
}

impl input::Continuous for Wave {
    fn get(&self, state: &CompositionState) -> f32 {
        (state.tick % (state.frequency / self.frequency)) as f32
            / state.tick as f32
    }
}

impl FromSpec for Wave {
    fn name() -> &'static str { "wave" }

    fn from_spec(spec: &mut Spec) -> Result<Box<Self>> {
        let wave_fn_name = spec.use_str("fn")?;
        let wave_fn = match wave_fn_name.as_ref() {
            "sine" => f32::sin,
            value => {
                return Err(
                    ErrorKind::SpecBadValue("fn".into(), value.into()).into()
                )
            }
        };

        spec.ensure_all_used()?;
        Ok(Box::new(Wave {
            wave_fn: Box::new(wave_fn),
            frequency: 1,
        }))
    }
}
