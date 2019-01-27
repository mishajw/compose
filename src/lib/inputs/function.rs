use core::input;
use core::spec::FromValue;
use core::spec::{Spec, Value};
use core::tree::Tree;
use core::Consts;
use core::State;
use core::Time;
use error::*;

/// A function input, returns values from a function
pub struct Function {
    function: Box<Fn(f64) -> f64 + Send + Sync>,
    lower_bound: f64,
    upper_bound: f64,
    time_mod: Option<Time>,
}

impl Function {
    #[allow(missing_docs)]
    pub fn bounded(
        function: Box<Fn(f64) -> f64 + Send + Sync>,
        lower_bound: f64,
        upper_bound: f64,
    ) -> Function
    {
        Function {
            function,
            lower_bound,
            upper_bound,
            time_mod: None,
        }
    }

    #[allow(missing_docs)]
    pub fn with_mod(
        function: Box<Fn(f64) -> f64 + Send + Sync>,
        lower_bound: f64,
        upper_bound: f64,
        time_mod: Time,
    ) -> Function
    {
        Function {
            function,
            lower_bound,
            upper_bound,
            time_mod: Some(time_mod),
        }
    }

    #[allow(missing_docs)]
    pub fn from_string(wave_string: String) -> Result<Function> {
        let function = match wave_string.as_ref() {
            "sine" => Function::with_mod(
                Box::new(|x| f64::sin(x * 2.0 * ::std::f64::consts::PI)),
                -1.0,
                1.0,
                Time::Seconds(1.0),
            ),
            "cosine" => Function::with_mod(
                Box::new(|x| f64::cos(x * 2.0 * ::std::f64::consts::PI)),
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
    pub fn default() -> Function {
        Self::from_string("sine".into())
            .expect("Failed to create default function")
    }
}

impl input::Bounded for Function {
    fn get(&mut self, state: &State) -> f64 {
        let tick = match &self.time_mod {
            Some(time_mod) => state.tick % time_mod.to_ticks(&state.consts),
            None => state.tick,
        };
        let fn_input = tick as f64 / state.consts.sample_hz;
        (*self.function)(fn_input)
    }

    fn get_bounds(&self) -> (f64, f64) { (self.lower_bound, self.upper_bound) }
}

impl Tree for Function {
    fn to_tree(&self) -> &Tree { self as &Tree }
}

impl FromValue for Function {
    fn name() -> &'static str { "function" }

    fn from_value(value: Value, consts: &Consts) -> Result<Self> {
        let mut spec: Spec = value.into_type(consts)?;
        let wave_fn_name: String =
            spec.consume_with_default("fn", "sine".into(), consts)?;
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
