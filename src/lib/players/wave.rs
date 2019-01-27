use core::input;
use core::spec::FromValue;
use core::spec::Value;
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
    pub fn player(input: Box<input::Bounded>, frequency: f64) -> Result<Speed> {
        Speed::player(PlayInput::player(input), frequency)
    }
}

impl FromValue<Speed> for Wave {
    fn name() -> &'static str { "wave" }

    fn from_value(value: Value, consts: &Consts) -> Result<Speed> {
        let mut spec: Spec = value.into_type(consts)?;
        let function = spec.consume_with_default::<Box<input::Bounded>>(
            "fn",
            Box::new(inputs::Function::default()),
            consts,
        )?;
        let frequency: f64 = spec.consume("frequency", consts)?;
        spec.ensure_all_used()?;
        Wave::player(function, frequency)
    }
}
