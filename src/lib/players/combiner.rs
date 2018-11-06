use core::CompositionState;
use core::Playable;
use core::Player;
use errors::*;
use spec::{create_player, FromSpec, Value};

/// Sum several children `Player` output into one output
pub struct Combiner {
    children: Vec<Box<Player>>,
}

impl Combiner {
    #[allow(missing_docs)]
    pub fn new(children: Vec<Box<Player>>) -> Self { Combiner { children } }
}

impl Player for Combiner {
    fn play(&mut self, state: &CompositionState) -> Playable {
        self.children.iter_mut().map(|p| p.play(state)).sum()
    }
}

impl FromSpec<Box<Player>> for Combiner {
    fn name() -> &'static str { "combiner" }
    fn from_spec(value: Value) -> Result<Box<Player>> {
        let mut spec = value.as_spec()?;
        let children_values: Vec<Value> = spec.consume("children")?;
        let mut children_specs = children_values
            .into_iter()
            .map(|v| v.as_spec())
            .collect::<Result<Vec<_>>>()
            .chain_err(|| "Failed to create combiner children specs")?;
        let children = children_specs
            .iter_mut()
            .map(create_player)
            .collect::<Result<Vec<_>>>()
            .chain_err(|| "Failed to create combiner children")?;

        Ok(Box::new(Combiner::new(children)))
    }
}
