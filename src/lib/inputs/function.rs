use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::Consts;
use core::State;
use core::Time;
use error::*;

/// A function input, returns values from a function
pub struct Function {
    function: Box<Fn(f32) -> f32 + Send + Sync>,
    lower_bound: f32,
    upper_bound: f32,
    time_mod: Option<Time>,
}

impl Function {
    #[allow(missing_docs)]
    pub fn new(
        function: Box<Fn(f32) -> f32 + Send + Sync>,
        lower_bound: f32,
        upper_bound: f32,
    ) -> Box<input::Bounded>
    {
        Box::new(Function {
            function,
            lower_bound,
            upper_bound,
            time_mod: None,
        })
    }

    #[allow(missing_docs)]
    pub fn with_mod(
        function: Box<Fn(f32) -> f32 + Send + Sync>,
        lower_bound: f32,
        upper_bound: f32,
        time_mod: Time,
    ) -> Box<input::Bounded>
    {
        Box::new(Function {
            function,
            lower_bound,
            upper_bound,
            time_mod: Some(time_mod),
        })
    }

    #[allow(missing_docs)]
    pub fn from_string(wave_string: String) -> Result<Box<input::Bounded>> {
        let function = match wave_string.as_ref() {
            "sine" => Function::with_mod(
                Box::new(|x| f32::sin(x * 2.0 * ::std::f32::consts::PI)),
                -1.0,
                1.0,
                Time::Seconds(1.0),
            ),
            "cosine" => Function::with_mod(
                Box::new(|x| f32::cos(x * 2.0 * ::std::f32::consts::PI)),
                -1.0,
                1.0,
                Time::Seconds(1.0),
            ),
            value => {
                return Err(
                    ErrorKind::SpecBadValue("fn".into(), value.into()).into()
                );
            }
        };

        Ok(function)
    }

    #[allow(missing_docs)]
    pub fn default() -> Box<input::Bounded> {
        Self::from_string("sine".into())
            .expect("Failed to create default function")
    }
}

impl input::Bounded for Function {
    fn get(&mut self, state: &State) -> f32 {
        self.time_mod.clone().unwrap();
        let tick = match &self.time_mod {
            Some(time_mod) => state.tick % time_mod.to_ticks(&state.consts),
            None => state.tick,
        };
        let fn_input = tick as f32 / state.consts.sample_hz;
        (*self.function)(fn_input)
    }

    fn get_bounds(&self) -> (f32, f32) { (self.lower_bound, self.upper_bound) }
}

impl create::FromSpec<Box<input::Bounded>> for Function {
    fn name() -> &'static str { "function" }

    fn from_spec(
        value: Value,
        _consts: &Consts,
    ) -> Result<Box<input::Bounded>>
    {
        let mut spec: Spec = value.as_type()?;
        let wave_fn_name: String =
            spec.consume_with_default("fn", "sine".into())?;
        spec.ensure_all_used()?;
        Ok(Function::from_string(wave_fn_name)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::Consts;
    use std::sync::Arc;

    #[test]
    fn test_sine() {
        let consts = Arc::new(Consts::default().unwrap());
        let state = State::initial(consts.clone());
        let mut function = Function::from_string("sine".into()).unwrap();
        assert!((0.0 - function.get(&state.with_tick(0))).abs() < 0.001);
        assert!(
            (1.0 - function
                .get(&state.with_tick((consts.sample_hz * 0.25) as usize)))
            .abs()
                < 0.001
        );
        assert!(
            (0.0 - function
                .get(&state.with_tick((consts.sample_hz * 0.5) as usize)))
            .abs()
                < 0.001
        );
        assert!(
            (-1.0
                - function
                    .get(&state.with_tick((consts.sample_hz * 0.75) as usize)))
            .abs()
                < 0.001
        );
        assert!(
            (0.0 - function.get(&state.with_tick(consts.sample_hz as usize)))
                .abs()
                < 0.001
        );
    }
}
