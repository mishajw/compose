use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::Consts;
use core::Input;
use core::Player;
use error::*;
use players::Combiner;
use players::Volume;

field_decl!(
    CHILDREN,
    Vec<Box<dyn Player>>,
    "The sounds played by the keyboard"
);
field_decl!(
    INPUTS,
    Vec<Box<dyn Input>>,
    "Controls what sounds are played"
);

/// Selectively plays from its children
pub struct Keyboard {}

impl SpecType<Combiner> for Keyboard {
    fn name() -> String {
        "keyboard".into()
    }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![CHILDREN.to_description(), INPUTS.to_description()]
    }

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
            .map(|(player, input)| Box::new(Volume::player(player, input)) as Box<dyn Player>)
            .collect();

        Ok(Combiner::player(children_with_input))
    }
}
