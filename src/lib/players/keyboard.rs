use core::Player;
use errors::*;
use players::{Combiner, Toggle};
use spec::{create_multi_bool_input, create_player, FromSpec, Value};

/// Selectively plays from its children
pub struct Keyboard {}

impl FromSpec<Box<Player>> for Keyboard {
    fn name() -> &'static str { "keyboard" }
    fn from_spec(value: Value) -> Result<Box<Player>> {
        let mut spec = value.as_spec()?;
        let inputs = create_multi_bool_input(&mut spec.consume("input")?)?;
        let children_specs: Vec<Value> = spec.consume("children")?;
        let children = children_specs
            .into_iter()
            .zip(inputs)
            .map(|(v, input)| {
                Value::as_spec(v)
                    .and_then(|mut s| create_player(&mut s))
                    .map(|c| Toggle::new(c, input))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Box::new(Combiner::new(children)))
    }
}
