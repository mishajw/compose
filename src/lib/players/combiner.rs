use core::spec::FromValue;
use core::spec::{Spec, Value};
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

/// Sum several children `Player` output into one output
pub struct Combiner {
    children: Vec<Box<Player>>,
}

impl Combiner {
    #[allow(missing_docs)]
    pub fn player(children: Vec<Box<Player>>) -> Self { Combiner { children } }
}

impl Player for Combiner {
    fn play(&mut self, state: &State) -> Playable {
        self.children.iter_mut().map(|p| p.play(state)).sum()
    }
}

impl Tree for Combiner {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        self.children.iter().map(|c| c.to_tree()).collect()
    }
}

impl FromValue for Combiner {
    fn name() -> &'static str { "combiner" }
    fn from_value(value: Value, consts: &Consts) -> Result<Self> {
        let mut spec: Spec = value.into_type(&consts)?;
        let children = spec.consume_list("children", consts)?;
        spec.ensure_all_used()?;
        Ok(Combiner::player(children))
    }
}
