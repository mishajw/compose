use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Input;
use core::State;
use core::Time;
use error::*;

field_decl!(FN, String, "The name of the function", |_| "sine"
    .to_string());

/// A function input, returns values from a function
pub struct Function {
    function: Box<Fn(f64) -> f64 + Send + Sync>,
    time_mod: Option<Time>,
}

impl Function {
    #[allow(missing_docs)]
    pub fn bounded(function: Box<Fn(f64) -> f64 + Send + Sync>) -> Function {
        Function {
            function,
            time_mod: None,
        }
    }

    #[allow(missing_docs)]
    pub fn with_mod(
        function: Box<Fn(f64) -> f64 + Send + Sync>,
        time_mod: Time,
    ) -> Function
    {
        Function {
            function,
            time_mod: Some(time_mod),
        }
    }

    #[allow(missing_docs)]
    pub fn from_string(wave_string: String) -> Result<Function> {
        let function = match wave_string.as_ref() {
            "sine" => Function::with_mod(
                Box::new(|x| f64::sin(x * 2.0 * ::std::f64::consts::PI)),
                Time::Seconds(1.0),
            ),
            "cosine" => Function::with_mod(
                Box::new(|x| f64::cos(x * 2.0 * ::std::f64::consts::PI)),
                Time::Seconds(1.0),
            ),
            "saw" => {
                Function::with_mod(Box::new(|x| x * x), Time::Seconds(1.0))
            }
            function => {
                return Err(ErrorKind::SpecError(format!(
                    "Unrecognized function: {}",
                    function
                ))
                .into());
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

impl Input for Function {
    fn get(&mut self, state: &State) -> f64 {
        let milli_tick = match &self.time_mod {
            Some(time_mod) => {
                state.milli_tick % (time_mod.to_ticks(&state.consts) * 1000)
            }
            None => state.milli_tick,
        };
        let fn_input = (milli_tick as f64) / 1000.0 / state.consts.sample_hz;
        (*self.function)(fn_input)
    }
}

impl Tree for Function {
    fn to_tree(&self) -> &Tree { self as &Tree }
}

impl SpecType for Function {
    fn name() -> String { "function".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![FN.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let fn_name = FN.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(Function::from_string(fn_name)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::input::Bounded;
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
