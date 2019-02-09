use core::spec::FieldDeclaration;
use core::spec::FieldDescription;
use core::spec::FromSpec;
use core::spec::Spec;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

field_decl!(
    CHILDREN,
    Vec<Box<Player>>,
    "Children players that are combined"
);

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

impl FromSpec for Combiner {
    fn name() -> &'static str { "combiner" }

    fn field_descriptions() -> Vec<FieldDescription> {
        vec![CHILDREN.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let children = CHILDREN.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(Combiner::player(children))
    }
}
