use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::CompositionConsts;
use core::CompositionState;
use errors::*;

/// A function input, returns values from a function
pub struct Function {
    function: Box<Fn(f32) -> f32 + Send + Sync>,
    lower_bound: f32,
    upper_bound: f32,
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
        })
    }

    #[allow(missing_docs)]
    pub fn from_string(wave_string: String) -> Result<Box<input::Bounded>> {
        let (function, lower_bound, upper_bound): (
            Box<Fn(f32) -> f32 + Send + Sync>,
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

        Ok(Function::new(function, lower_bound, upper_bound))
    }
}

impl input::Bounded for Function {
    fn get(&mut self, state: &CompositionState) -> f32 {
        let fn_input = state.tick as f32 / state.consts.sample_hz;
        (*self.function)(fn_input)
    }

    fn get_bounds(&self) -> (f32, f32) { (self.lower_bound, self.upper_bound) }
}

impl create::FromSpec<Box<input::Bounded>> for Function {
    fn name() -> &'static str { "function" }

    fn from_spec(
        value: Value,
        _consts: &CompositionConsts,
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
    use core::CompositionConsts;

    #[test]
    fn test_sine() {
        let consts = CompositionConsts::default();
        let state = CompositionState::initial(consts.clone());
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
