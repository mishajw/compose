use core::input;
use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::Consts;
use core::Player;
use error::*;
use players::Combiner;
use players::Volume;

field_decl!(
    CHILDREN,
    Vec<Box<Player>>,
    "The sounds played by the keyboard"
);
field_decl!(
    INPUTS,
    Vec<Box<input::Bounded>>,
    "Controls what sounds are played"
);

/// Selectively plays from its children
pub struct Keyboard {}

impl SpecType<Combiner> for Keyboard {
    fn name() -> &'static str { "keyboard" }

    fn field_descriptions() -> Vec<SpecFieldDescription> { Vec::new() }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Combiner> {
        let children = CHILDREN.get(&mut spec, consts)?;
        let inputs = INPUTS.get(&mut spec, consts)?;
        spec.ensure_all_used()?;

        if children.len() != inputs.len() {
            return Err(ErrorKind::SpecError(format!(
                "Children and inputs are different lengths: {} and {} \
                 respectively",
                children.len(),
                inputs.len()
            ))
            .into());
        }

        let children_with_input = children
            .into_iter()
            .zip(inputs)
            .map(|(player, input)| {
                Box::new(Volume::player(player, input)) as Box<Player>
            })
            .collect();

        Ok(Combiner::player(children_with_input))
    }
}
