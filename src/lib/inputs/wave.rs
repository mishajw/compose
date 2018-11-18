use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::CompositionState;
use errors::*;

/// A wave input, gets values from a function at a frequency
pub struct Wave {
    wave_fn: Box<Fn(f32) -> f32>,
    frequency: f32,
    lower_bound: f32,
    upper_bound: f32,
}

impl Wave {
    #[allow(missing_docs)]
    pub fn new(
        wave_fn: Box<Fn(f32) -> f32>,
        frequency: f32,
        lower_bound: f32,
        upper_bound: f32,
    ) -> Box<input::Bounded>
    {
        Box::new(Wave {
            wave_fn,
            frequency,
            lower_bound,
            upper_bound,
        })
    }

    #[allow(missing_docs)]
    pub fn from_string(
        wave_string: String,
        frequency: f32,
    ) -> Result<Box<input::Bounded>>
    {
        let (wave_fn, lower_bound, upper_bound): (
            Box<Fn(f32) -> f32>,
            f32,
            f32,
        ) = match wave_string.as_ref() {
            "sine" => (
                Box::new(|x| f32::sin(x * 2.0 * ::std::f32::consts::PI)),
                -1.0,
                1.0,
            ),
            "cosine" => (
                Box::new(|x| f32::cos(x * 2.0 * ::std::f32::consts::PI)),
                -1.0,
                1.0,
            ),
            value => {
                return Err(
                    ErrorKind::SpecBadValue("fn".into(), value.into()).into()
                )
            }
        };

        Ok(Wave::new(wave_fn, frequency, lower_bound, upper_bound))
    }
}

impl input::Bounded for Wave {
    fn get(&mut self, state: &CompositionState) -> f32 {
        let num_ticks = state.consts.sample_hz / self.frequency;
        let fn_input = state.tick as f32 / num_ticks;
        (*self.wave_fn)(fn_input)
    }

    fn get_bounds(&self) -> (f32, f32) { (self.lower_bound, self.upper_bound) }
}

impl create::FromSpec<Box<input::Bounded>> for Wave {
    fn name() -> &'static str { "wave" }

    fn from_spec(value: Value) -> Result<Box<input::Bounded>> {
        let mut spec: Spec = value.as_type()?;
        let wave_fn_name: String =
            spec.consume_with_default("fn", "sine".into())?;
        let frequency = spec.consume_with_default("frequency", 1.0)?;
        spec.ensure_all_used()?;
        Ok(Wave::from_string(wave_fn_name, frequency)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::CompositionConsts;

    #[test]
    fn test_sine() {
        let consts = CompositionConsts::default();
        let state = CompositionState::initial(consts.clone());
        let mut wave = Wave::from_string("sine".into(), 1.0).unwrap();
        assert!((0.0 - wave.get(&state.with_tick(0))).abs() < 0.001);
        assert!(
            (1.0 - wave
                .get(&state.with_tick((consts.sample_hz * 0.25) as usize)))
            .abs()
                < 0.001
        );
        assert!(
            (0.0 - wave
                .get(&state.with_tick((consts.sample_hz * 0.5) as usize)))
            .abs()
                < 0.001
        );
        assert!(
            (-1.0
                - wave
                    .get(&state.with_tick((consts.sample_hz * 0.75) as usize)))
            .abs()
                < 0.001
        );
        assert!(
            (0.0 - wave.get(&state.with_tick(consts.sample_hz as usize))).abs()
                < 0.001
        );
    }
}
