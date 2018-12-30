use core::spec::create;
use core::spec::{Spec, Value};
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use errors::*;

/// Sum several children `Player` output into one output
pub struct Combiner {
    children: Vec<Box<Player>>,
}

impl Combiner {
    #[allow(missing_docs)]
    pub fn new(children: Vec<Box<Player>>) -> Self { Combiner { children } }
}

impl Player for Combiner {
    fn play(&mut self, state: &State) -> Playable {
        self.children.iter_mut().map(|p| p.play(state)).sum()
    }
}

impl create::FromSpec<Box<Player>> for Combiner {
    fn name() -> &'static str { "combiner" }
    fn from_spec(value: Value, consts: &Consts) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let children_values: Vec<Value> = spec.consume("children")?;
        let mut children_specs = children_values
            .into_iter()
            .map(|v| v.as_type())
            .collect::<Result<Vec<_>>>()
            .chain_err(|| "Failed to create combiner children specs")?;
        let children = children_specs
            .iter_mut()
            .map(|s| create::create_player(s, consts))
            .collect::<Result<Vec<_>>>()
            .chain_err(|| "Failed to create combiner children")?;

        Ok(Box::new(Combiner::new(children)))
    }
}
