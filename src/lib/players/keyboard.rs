use core::input;
use core::spec::FromValue;
use core::spec::{Spec, Value};
use core::Consts;
use core::Player;
use error::*;
use players::Combiner;
use players::Volume;

/// Selectively plays from its children
pub struct Keyboard {}

impl FromValue<Combiner> for Keyboard {
    fn name() -> &'static str { "keyboard" }
    fn from_value(value: Value, consts: &Consts) -> Result<Combiner> {
        let mut spec: Spec = value.into_type(consts)?;
        let children: Vec<Box<Player>> =
            spec.consume_list("children", consts)?;
        let inputs: Vec<Box<input::Bounded>> =
            spec.consume_list("inputs", consts)?;
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
