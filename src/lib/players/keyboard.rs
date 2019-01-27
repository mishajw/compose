use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::Consts;
use core::Player;
use error::*;
use players::Combiner;
use players::Volume;

/// Selectively plays from its children
pub struct Keyboard {}

impl create::FromSpec<Box<Player>> for Keyboard {
    fn name() -> &'static str { "keyboard" }
    fn from_spec(value: Value, consts: &Consts) -> Result<Box<Player>> {
        let mut spec: Spec = value.into_type()?;
        let children: Vec<Box<Player>> = spec
            .consume_list("children")?
            .iter_mut()
            .map(|s| create::create_player(s, consts))
            .collect::<Result<Vec<_>>>()?;
        let inputs: Vec<Box<input::Bounded>> = spec
            .consume_list("inputs")?
            .iter_mut()
            .map(|s| create::create_bounded_input(s, consts))
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
            .map(|(player, input)| Volume::player(player, input))
            .collect::<Vec<_>>();

        Ok(Combiner::player(children_with_input))
    }
}
