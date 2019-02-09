use core::input;
use core::spec::FieldDeclaration;
use core::spec::FieldDescription;
use core::spec::FromSpec;
use core::spec::Spec;
use core::Consts;
use error::*;
use inputs::Function;
use players::PlayInput;
use players::Speed;

field_decl!(
    FN,
    Box<input::Bounded>,
    "The function that defines the wave shape",
    |_| Box::new(Function::default()) as Box<input::Bounded>
);
field_decl!(FREQUENCY, f64, "Frequency of the wave");

/// Play a wave from a wave function
pub struct Wave {}

impl Wave {
    #[allow(missing_docs)]
    pub fn player(input: Box<input::Bounded>, frequency: f64) -> Result<Speed> {
        Speed::player(PlayInput::player(input), frequency)
    }
}

impl FromSpec<Speed> for Wave {
    fn name() -> &'static str { "wave" }

    fn field_descriptions() -> Vec<FieldDescription> {
        vec![FN.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Speed> {
        let function = FN.get(&mut spec, consts)?;
        let frequency = FREQUENCY.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Wave::player(function, frequency)
    }
}
