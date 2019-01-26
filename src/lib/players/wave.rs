use core::input;
use core::spec::create;
use core::spec::Value;
use core::Player;
use error::*;
use inputs;

use core::spec::Spec;
use core::Consts;
use players::PlayInput;
use players::Speed;

/// Play a wave from a wave function
pub struct Wave {}

impl Wave {
    #[allow(missing_docs)]
    pub fn new(
        input: Box<input::Bounded>,
        frequency: f32,
    ) -> Result<Box<Player>>
    {
        Speed::new(PlayInput::new(input), frequency as f64)
    }
}

impl create::FromSpec<Box<Player>> for Wave {
    fn name() -> &'static str { "wave" }

    fn from_spec(value: Value, _consts: &Consts) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let function = match spec.consume_optional("fn")? {
            Some(string) => inputs::Function::from_string(string)?,
            None => inputs::Function::default(),
        };
        let frequency: f32 = spec.consume("frequency")?;
        spec.ensure_all_used()?;
        Wave::new(function, frequency)
    }
}
