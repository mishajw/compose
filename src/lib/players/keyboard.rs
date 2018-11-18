use core::input;
use core::Player;
use errors::*;
use players::Combiner;
use players::Volume;
use spec::{create_bounded_input, create_player, FromSpec, Spec, Value};

/// Selectively plays from its children
pub struct Keyboard {}

impl FromSpec<Box<Player>> for Keyboard {
    fn name() -> &'static str { "keyboard" }
    fn from_spec(value: Value) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let children: Vec<Box<Player>> = spec
            .consume_list("children")?
            .iter_mut()
            .map(create_player)
            .collect::<Result<Vec<_>>>()?;
        let inputs: Vec<Box<input::Bounded>> = spec
            .consume_list("inputs")?
            .iter_mut()
            .map(create_bounded_input)
            .collect::<Result<Vec<_>>>()?;
        spec.ensure_all_used()?;

        if children.len() != inputs.len() {
            return Err(ErrorKind::SpecBadValue(
                "children/inputs".into(),
                format!(
                    "Different lengths: {}, {}",
                    children.len(),
                    inputs.len()
                ),
            )
            .into());
        }

        let children_with_input = children
            .into_iter()
            .zip(inputs)
            .map(|(player, input)| Volume::new(player, input))
            .collect::<Vec<_>>();

        Ok(Box::new(Combiner::new(children_with_input)))
    }
}
